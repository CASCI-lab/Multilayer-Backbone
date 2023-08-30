mod bfs_tools;
mod closure;
mod conversion;
mod direct_backbone;
mod multidistance;
mod multigraph;
mod shortest_paths;

use std::collections::HashMap;

pub use bfs_tools::*;
pub use closure::*;
pub use conversion::*;
pub use direct_backbone::*;
pub use multidistance::*;
pub use multigraph::*;
pub use shortest_paths::*;
// pub type MultilayerBackbone = HashMap<NodeID, HashMap<NodeID, Vec<MultiDistance>>>;

use pyo3::prelude::*;

#[pymodule]
fn backbone(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(distance_closure_py, m)?)?;
    m.add_function(wrap_pyfunction!(backbone_py, m)?)?;
    m.add_function(wrap_pyfunction!(structural_backbone_simas, m)?)?;
    m.add_function(wrap_pyfunction!(structural_backbone_costa, m)?)?;
    m.add_function(wrap_pyfunction!(structural_backbone_naive, m)?)?;

    Ok(())
}

#[pyfunction]
#[allow(clippy::needless_pass_by_value)] // this makes it easier to deal with pyO3
fn distance_closure_py(
    edges: Vec<(usize, usize, usize, usize, usize, f32)>,
) -> MultidistanceClosure {
    distance_closure(&edges)
}

#[pyfunction]
#[allow(clippy::needless_pass_by_value)] // this makes it easier to deal with pyO3
fn backbone_py(edges: Vec<(usize, usize, usize, usize, usize, f32)>) -> MultilayerBackbone {
    multilayer_backbone(&edges)
}

#[pyfunction]
#[allow(clippy::needless_pass_by_value)] // this makes it easier to deal with pyO3
fn structural_backbone_simas(
    edges: Vec<(usize, usize, usize, usize, usize, f32)>,
) -> HashMap<NodeID, HashMap<NodeID, MultiDistance>> {
    let mut graph = MultidistanceGraphHashmap::from_tuple_edge_list(&edges);
    fast_backbone_simas(&mut graph);
    graph.edges
}

#[pyfunction]
#[allow(clippy::needless_pass_by_value)] // this makes it easier to deal with pyO3
fn structural_backbone_costa(
    edges: Vec<(usize, usize, usize, usize, usize, f32)>,
) -> HashMap<NodeID, HashMap<NodeID, MultiDistance>> {
    let mut graph = MultidistanceGraphHashmap::from_tuple_edge_list(&edges);
    fast_backbone_costa(&mut graph);
    graph.edges
}

#[pyfunction]
#[allow(clippy::needless_pass_by_value)] // this makes it easier to deal with pyO3
fn structural_backbone_naive(
    edges: Vec<(usize, usize, usize, usize, usize, f32)>,
) -> HashMap<NodeID, HashMap<NodeID, MultiDistance>> {
    let mut graph = MultidistanceGraphHashmap::from_tuple_edge_list(&edges);
    structural_backbone(&mut graph, None);
    graph.edges
}

/// The function `distance_closure` takes a list of edges and returns a
/// multidistance closure.
///
/// # Arguments
///
/// * `edges` - A slice of tuples representing edges in a multigraph. Each tuple
///   contains six elements: source node, target node, source layer, target
///   layer, layer wieght index, and edge weight.
///
/// # Returns
///
/// The function `distance_closure` returns a value of type
/// `MultidistanceClosure`.
#[must_use]
pub fn distance_closure(
    edges: &[(usize, usize, usize, usize, usize, f32)],
) -> MultidistanceClosure {
    let graph = MultidistanceGraphHashmap::from_tuple_edge_list(edges);
    multidistance_closure(&graph)
}

/// The function `multilayer_backbone` takes a list of edges and returns a multilayer backbone, which is
/// a subset of the edges that satisfy certain conditions.
///
/// # Panics
/// * Will panic if the computed closure does not contain an entry for a direct edge.
///
/// # Arguments
///
/// * `edges` - A slice of tuples representing edges in a multigraph. Each tuple
///   contains six elements: source node, target node, source layer, target
///   layer, layer wieght index, and edge weight.
///
/// # Returns
///
/// The function `multilayer_backbone` returns a `MultilayerBackbone` object.
#[must_use]
pub fn multilayer_backbone(
    edges: &[(usize, usize, usize, usize, usize, f32)],
) -> MultilayerBackbone {
    let graph = MultidistanceGraphHashmap::from_tuple_edge_list(edges);
    let closure = multidistance_closure(&graph);

    let mut backbone = HashMap::new();

    for edge in edges {
        let source = NodeID(edge.0);
        let target = NodeID(edge.1);
        let layer = EdgeLayerID {
            layer_start: edge.2,
            layer_end: edge.3,
            layer_weight_index: edge.4,
        };

        let total = if edge.5 == 0.0 {
            HashMap::new()
        } else {
            HashMap::from([(layer, edge.5)])
        };

        let multidist = MultiDistance { total };

        let mins = closure.get(&source).unwrap().get(&target).unwrap();

        if mins.contains(&multidist) {
            backbone
                .entry(source)
                .or_insert(HashMap::new())
                .entry(target)
                .or_insert(mins.clone());
        }
    }

    backbone
}
