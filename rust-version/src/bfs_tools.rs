use crate::{
    multidistance::{MultiDistance, NodeID},
    parteto_shortest_distance_from_source,
};
use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    hash::BuildHasher,
};
pub type EdgeMap<S> = HashMap<NodeID, Vec<(NodeID, MultiDistance)>, S>;

pub struct MissingEdgeError {
    source: NodeID,
    target: NodeID,
}

/// # Errors
/// `MissingEdgeError` is returned if the specified edge does not exist in the
/// edge map.
///
/// # Panics
/// Panics if a direct edge is detected, but no shortest path between its
/// endpoints is found. Should not be possible.
pub fn is_metric_in_n_steps<S: BuildHasher + std::marker::Sync + Default>(
    edge_map: &EdgeMap<S>,
    source: NodeID,
    target: NodeID,
    n_steps: usize,
) -> Result<bool, MissingEdgeError> {
    if let Some(neighbors) = edge_map.get(&source) {
        if let Some((_, test_edge_weight)) = neighbors.iter().find(|(node, _)| node == &target) {
            let shortest_dists =
                parteto_shortest_distance_from_source(source, edge_map, Some(n_steps));

            let dist_to_target = shortest_dists.get(&target).unwrap();

            return Ok(dist_to_target.iter().any(|md| test_edge_weight == md));
        }
    }
    Err(MissingEdgeError { source, target })
}
