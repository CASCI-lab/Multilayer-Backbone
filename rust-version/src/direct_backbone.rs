use crate::{
    is_metric_in_n_steps,
    multidistance::{MultiDistance, NodeID},
    multimin, parteto_shortest_distance_from_source, MultidistanceGraph,
};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
pub type MultilayerBackbone = HashMap<NodeID, HashMap<NodeID, Vec<MultiDistance>>>;

/// # Panics
/// Panics if `bb_map` lacks a node present in `edge_map`. This indicates a bug.
pub fn fast_backbone_costa<T>(graph: &mut T)
where
    T: MultidistanceGraph + Sync,
{
    for source in graph.nodes() {
        let distances = parteto_shortest_distance_from_source(source, graph, None, None);

        let neighbors = graph.neighbor_edges(&source);
        for (target, direct_weight) in neighbors {
            if distances.get(&target).is_some_and(|distances_to_target| {
                distances_to_target.iter().any(|d| d < &direct_weight)
            }) {
                graph.remove_edge(source, target);
            }
        }
    }
}

pub fn fast_backbone_simas<T>(graph: &mut T)
where
    T: MultidistanceGraph + Sync,
{
    // let mut bb_graph = *graph.clone();
    //let bb_map = &structural_backbone(edge_map, Some(2));

    let mut known_metric_edges = one_step_metric_edges(graph);
    two_step_metric_edges(graph, &mut known_metric_edges); // modifies `known_metric_edges` in-place

    let semimetric_edges: Vec<(NodeID, NodeID)> = graph
        .nodes()
        .par_iter()
        .flat_map(|source| -> Vec<(NodeID, NodeID)> {
            graph
                .neighbor_edges(source)
                .par_iter()
                .map(std::clone::Clone::clone)
                .filter(|(target, _)| {
                    !known_metric_edges.contains(&(*source, *target))
                        && !is_metric_in_n_steps(graph, *source, *target, None).unwrap_or(false)
                })
                .map(|(target, _)| (*source, target))
                .collect()
        })
        .collect();

    for (u, v) in &semimetric_edges {
        graph.remove_edge(*u, *v);
    }
}

pub fn structural_backbone<T>(
    graph: &mut T,
    n_steps: Option<usize>, // if None, computes full structural backbone
) where
    T: MultidistanceGraph + Sync,
{
    let semimetric_edges: Vec<(NodeID, NodeID)> = graph
        .nodes()
        .par_iter()
        .flat_map(|source| -> Vec<(NodeID, NodeID)> {
            graph
                .neighbor_edges(source)
                .par_iter()
                .map(std::clone::Clone::clone)
                .filter(|(target, _)| {
                    !is_metric_in_n_steps(graph, *source, *target, n_steps).unwrap_or(false)
                })
                .map(|(target, _)| (*source, target))
                .collect()
        })
        .collect();

    for (u, v) in &semimetric_edges {
        graph.remove_edge(*u, *v);
    }
}

fn one_step_metric_edges<T>(graph: &T) -> HashSet<(NodeID, NodeID)>
where
    T: MultidistanceGraph + Sync,
{
    // TODO: incorporate edge reversal for min incoming edges
    min_edges_with_condition(graph, |_, _, _| true)
}

fn min_edges_with_condition<T>(
    graph: &T,
    condition: impl Fn(&NodeID, &NodeID, &MultiDistance) -> bool + Send + Sync,
) -> HashSet<(NodeID, NodeID)>
where
    T: MultidistanceGraph + Sync,
{
    graph
        .nodes()
        .iter()
        .flat_map(|source| -> HashSet<(NodeID, NodeID)> {
            let out_edges: Vec<MultiDistance> = graph
                .neighbor_edges(source)
                .iter()
                .filter(|(t, d)| condition(source, t, d))
                .map(|(_, dist)| dist.clone())
                .collect();

            let multimin_for_source = multimin(&out_edges);

            HashSet::from_par_iter(
                graph
                    .neighbor_edges(source)
                    .par_iter()
                    .filter(|(_, md)| multimin_for_source.contains(md))
                    .map(|(target, _)| (*source, *target)),
            )
        })
        .collect()
}

fn two_step_metric_edges<T>(graph: &T, known_metric_edges: &mut HashSet<(NodeID, NodeID)>)
where
    T: MultidistanceGraph + Sync,
{
    for source in &graph.nodes() {
        let neighbors = graph.neighbor_edges(source);
        let mut two_hop_known_metric_dists = Vec::new();
        for (s, target) in known_metric_edges.iter() {
            if s != source {
                continue;
            }
            if let Some((_, dist)) = neighbors.iter().find(|(t, _)| t == target) {
                for (_, dist2) in graph.neighbor_edges(target) {
                    two_hop_known_metric_dists.push(dist.clone() + dist2.clone());
                }
            }
        }

        let mut continue_search = true;
        while continue_search {
            continue_search = false;

            let mut remainder = neighbors.clone();
            remainder.retain(|(target, _)| !known_metric_edges.contains(&(*source, *target)));
            let remainder_weights: Vec<MultiDistance> =
                remainder.iter().map(|(_, d)| d.clone()).collect();
            let min_weights = multimin(&remainder_weights);

            for (target, multidist) in remainder {
                if !min_weights.contains(&multidist) {
                    continue;
                }
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
