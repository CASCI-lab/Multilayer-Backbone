use crate::{
    multidistance::{MultiDistance, NodeID},
    parteto_shortest_distance_from_source, MultidistanceGraph,
};
use std::collections::HashMap;
pub type EdgeMap<S> = HashMap<NodeID, Vec<(NodeID, MultiDistance)>, S>;

#[allow(dead_code)]
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
pub fn is_metric_in_n_steps(
    graph: &impl MultidistanceGraph,
    source: NodeID,
    target: NodeID,
    n_steps: Option<usize>,
) -> Result<bool, MissingEdgeError> {
    let neighbors = graph.neighbor_edges(&source);
    if let Some((_, test_edge_weight)) = neighbors.iter().find(|(node, _)| node == &target) {
        let shortest_dists = parteto_shortest_distance_from_source(
            source,
            graph,
            n_steps,
            Some((&target, &test_edge_weight.clone())),
        );

        if let Some(dist_to_target) = shortest_dists.get(&target) {
            return Ok(dist_to_target.iter().any(|md| test_edge_weight == md));
        }
    }

    Err(MissingEdgeError { source, target })
}
