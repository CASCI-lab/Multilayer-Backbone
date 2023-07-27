mod closure;
mod conversion;
mod multidistance;
mod shortest_paths;

use std::collections::HashMap;

pub use closure::*;
pub use conversion::*;
pub use multidistance::*;
pub use shortest_paths::*;

pub type MultilayerBackbone = HashMap<NodeID, HashMap<NodeID, Vec<MultiDistance>>>;

#[must_use]
pub fn distance_closure(
    edges: &[(usize, usize, usize, usize, usize, f32)],
) -> MultidistanceClosure {
    let multiplex = edges_to_multiplex(edges);
    multidistance_closure(&multiplex)
}

/// # Panics
/// Will panic if the computed closure does not contain an entry for a direct edge.
#[must_use]
pub fn multilayer_backbone(
    edges: &[(usize, usize, usize, usize, usize, f32)],
) -> MultilayerBackbone {
    let multiplex = edges_to_multiplex(edges);
    let closure = multidistance_closure(&multiplex);

    let mut backbone = HashMap::new();

    for edge in edges {
        let source = NodeID(edge.0);
        let target = NodeID(edge.1);
        let layer = EdgeLayerID {
            layer_start: edge.2,
            layer_end: edge.3,
            layer_weight_index: edge.4,
        };
        let multidist = MultiDistance {
            total: HashMap::from([(layer, edge.5)]),
        };

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
