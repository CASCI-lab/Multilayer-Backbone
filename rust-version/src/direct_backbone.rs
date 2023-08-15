use crate::{
    is_metric_in_n_steps,
    multidistance::{MultiDistance, NodeID},
    EdgeMap,
};
use rayon::prelude::*;
use std::{
    collections::{hash_map::RandomState, HashMap},
    hash::BuildHasher,
};
pub type MultilayerBackbone = HashMap<NodeID, HashMap<NodeID, Vec<MultiDistance>>>;

pub fn n_step_backbone_structural<S: BuildHasher + std::marker::Sync + Default>(
    edge_map: &EdgeMap<S>,
    n_steps: usize,
) -> EdgeMap<RandomState> {
    // let mut bb_edges = HashMap::default();

    let bb_map = edge_map
        .par_iter()
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
                neighbor_edge_map
            },
        )
        .reduce(HashMap::new, |a, b| {
            b.iter().fold(a, |mut acc, (k, v)| {
                acc.entry(*k).or_insert(v.clone());
                acc
            })
        });
    bb_map

    // for (source, neighbors) in edge_map.iter() {
    //     let n_step_metric_neighbors: Vec<(NodeID, MultiDistance)> = neighbors
    //         .par_iter()
    //         .map(std::clone::Clone::clone)
    //         .filter(|(target, _)| {
    //             is_metric_in_n_steps(edge_map, *source, *target, n_steps).unwrap_or(false)
    //         })
    //         .collect();

    //     bb_edges.insert(*source, n_step_metric_neighbors);
    // }

    // bb_edges
}
