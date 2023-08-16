use crate::{
    is_metric_in_n_steps,
    multidistance::{MultiDistance, NodeID},
    multimin, reverse_edges, EdgeMap,
};
use rayon::prelude::*;
use std::{
    collections::{hash_map::RandomState, HashMap, HashSet},
    hash::BuildHasher,
};
pub type MultilayerBackbone = HashMap<NodeID, HashMap<NodeID, Vec<MultiDistance>>>;

pub fn fast_backbone<S: BuildHasher + std::marker::Sync + Default>(
    edge_map: &EdgeMap<S>,
) -> EdgeMap<RandomState> {
    let bb_map = structural_backbone(edge_map, Some(2));

    let mut known_metric_edges = one_step_metric_edges(edge_map);
    two_step_metric_edges(edge_map, &mut known_metric_edges); // modifies `known_metric_edges` in-place

    bb_map
        .par_iter()
        .map(
            |(source, neighbors)| -> HashMap<NodeID, Vec<(NodeID, MultiDistance)>> {
                let mut neighbor_edge_map = HashMap::default();
                let metric_neighbors: Vec<(NodeID, MultiDistance)> = neighbors
                    .par_iter()
                    .map(std::clone::Clone::clone)
                    .filter(|(target, _)| {
                        known_metric_edges.contains(&(*source, *target))
                            || is_metric_in_n_steps(&bb_map, *source, *target, None)
                                .unwrap_or(false)
                    })
                    .collect();
                neighbor_edge_map.insert(*source, metric_neighbors);
                neighbor_edge_map // has only source as a key
            },
        )
        // merge the hashmaps (which each have only one key) into one hashmap
        // with a key for each node
        .reduce(HashMap::new, |a, b| {
            b.iter().fold(a, |mut acc, (k, v)| {
                acc.entry(*k).or_insert(v.clone());
                acc
            })
        })
}

pub fn structural_backbone<S: BuildHasher + std::marker::Sync + Default>(
    edge_map: &EdgeMap<S>,
    n_steps: Option<usize>, // if None, computes full structural backbone
) -> EdgeMap<RandomState> {
    edge_map
        .par_iter()
        // map each key value pair to a hashmap that contains only each source
        // considered and the neighbors that are metric up to n steps
        .map(
            |(source, neighbors)| -> HashMap<NodeID, Vec<(NodeID, MultiDistance)>> {
                let mut neighbor_edge_map = HashMap::default();
                let n_step_metric_neighbors: Vec<(NodeID, MultiDistance)> = neighbors
                    .par_iter()
                    .map(std::clone::Clone::clone)
                    .filter(|(target, _)| {
                        is_metric_in_n_steps(edge_map, *source, *target, n_steps).unwrap_or(false)
                    })
                    .collect();
                neighbor_edge_map.insert(*source, n_step_metric_neighbors);
                neighbor_edge_map // has only source as a key
            },
        )
        // merge the hashmaps (which each have only one key) into one hashmap
        // with a key for each node
        .reduce(HashMap::new, |a, b| {
            b.iter().fold(a, |mut acc, (k, v)| {
                acc.entry(*k).or_insert(v.clone());
                acc
            })
        })
}

fn one_step_metric_edges<S: BuildHasher + std::marker::Sync + Default>(
    edge_map: &EdgeMap<S>,
) -> HashSet<(NodeID, NodeID)> {
    // let mut known_metric_edges = HashSet::new();

    // for (source, neighbors) in edge_map.iter().chain(reverse_edges(edge_map).iter()) {
    //     let out_edges: Vec<MultiDistance> =
    //         neighbors.iter().map(|(_, dist)| dist.clone()).collect();

    //     let multimin_for_source = multimin(&out_edges);

    //     known_metric_edges.extend(
    //         neighbors
    //             .iter()
    //             .filter(|(_, md)| multimin_for_source.contains(md))
    //             .map(|(target, _)| (*source, *target)),
    //     );
    // }

    // known_metric_edges

    let forward = edge_map.par_iter();
    let binding = reverse_edges(edge_map);
    let reverse = binding.par_iter();
    let combined = forward.chain(reverse);

    // combined
    //     .map(|(source, neighbors)| -> HashSet<(NodeID, NodeID)> {
    //         let out_edges: Vec<MultiDistance> =
    //             neighbors.iter().map(|(_, dist)| dist.clone()).collect();

    //         let multimin_for_source = multimin(&out_edges);

    //         HashSet::from_par_iter(
    //             neighbors
    //                 .par_iter()
    //                 .filter(|(_, md)| multimin_for_source.contains(md))
    //                 .map(|(target, _)| (*source, *target)),
    //         )
    //     })
    //     .flatten()
    //     .collect()

    min_edges_with_condition(combined, |_, _, _| true)
}

fn min_edges_with_condition<'a>(
    edge_iter: impl ParallelIterator<Item = (&'a NodeID, &'a Vec<(NodeID, MultiDistance)>)>,
    condition: impl Fn(&'a NodeID, &'a NodeID, &'a MultiDistance) -> bool + Send + Sync + 'a,
) -> HashSet<(NodeID, NodeID)> {
    edge_iter
        .map(|(source, neighbors)| -> HashSet<(NodeID, NodeID)> {
            let out_edges: Vec<MultiDistance> = neighbors
                .iter()
                .filter(|(t, d)| condition(source, t, d))
                .map(|(_, dist)| dist.clone())
                .collect();

            let multimin_for_source = multimin(&out_edges);

            HashSet::from_par_iter(
                neighbors
                    .par_iter()
                    .filter(|(_, md)| multimin_for_source.contains(md))
                    .map(|(target, _)| (*source, *target)),
            )
        })
        .flatten()
        .collect()
}

fn two_step_metric_edges<S: BuildHasher + std::marker::Sync + Default>(
    edge_map: &EdgeMap<S>,
    known_metric_edges: &mut HashSet<(NodeID, NodeID)>,
) {
    for (source, neighbors) in edge_map.iter() {
        let mut two_hop_known_metric_dists = Vec::new();
        for (s, target) in known_metric_edges.iter() {
            if s != source {
                continue;
            }
            if let Some((_, dist)) = neighbors.iter().find(|(t, _)| t == target) {
                for (_, dist2) in edge_map.get(target).unwrap_or(&Vec::new()) {
                    two_hop_known_metric_dists.push(dist.clone() + dist2.clone());
                }
            }
        }

        let mut source_edge_map = HashMap::new();
        source_edge_map.insert(*source, neighbors.clone());

        let mut continue_search = true;
        while continue_search {
            continue_search = false;
            let remainder = min_edges_with_condition(source_edge_map.par_iter(), |s, t, _| {
                !known_metric_edges.contains(&(*s, *t))
            });

            for (_, target) in remainder {
                // remainder is constructed so that all edges begin as `source`
                if let Some((_, dist)) = neighbors.iter().find(|(t, _)| *t == target) {
                    if two_hop_known_metric_dists
                        .iter()
                        .all(|d2| d2 > dist || d2.partial_cmp(dist).is_none())
                    {
                        known_metric_edges.insert((*source, target));
                        continue_search = true;
                    }
                }
            }
        }
    }
}
