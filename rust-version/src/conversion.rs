use std::collections::HashMap;

use crate::multidistance::{EdgeLayerID, MultiDistance, NodeID};

#[must_use]
pub fn edges_to_multiplex(
    edges: &[(usize, usize, usize, usize, f32)],
) -> HashMap<NodeID, Vec<(NodeID, MultiDistance)>> {
    let mut multiplex = HashMap::new();
    for edge in edges {
        let source = NodeID(edge.0);
        let target = NodeID(edge.1);
        let layer = EdgeLayerID {
            layer_start: edge.2,
            layer_end: edge.3,
            layer_weight_index: 0,
        };
        let edge_weight = edge.4;

        let multidist = MultiDistance {
            total: HashMap::from([(layer, edge_weight)]),
        };

        multiplex
            .entry(source)
            .or_insert(vec![])
            .push((target, multidist));
        multiplex.entry(target).or_insert(vec![]);
    }

    multiplex
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(clippy::redundant_clone)]
    #[test]
    fn test_simple_conversion() {
        let layer_0_0 = EdgeLayerID {
            layer_start: 0,
            layer_end: 0,
            layer_weight_index: 0,
        };
        let layer_1_1 = EdgeLayerID {
            layer_start: 1,
            layer_end: 1,
            layer_weight_index: 0,
        };
        let layer_0_1 = EdgeLayerID {
            layer_start: 0,
            layer_end: 1,
            layer_weight_index: 0,
        };

        let m01 = MultiDistance {
            total: HashMap::from([(layer_0_0, 1.0)]),
        };

        let m03 = MultiDistance {
            total: HashMap::from([(layer_0_1, 2.0)]),
        };

        let m12 = MultiDistance {
            total: HashMap::from([(layer_0_1, 1.0)]),
        };

        let m23 = MultiDistance {
            total: HashMap::from([(layer_1_1, 1.0)]),
        };

        let edge_list_expected: HashMap<NodeID, Vec<(NodeID, MultiDistance)>> = HashMap::from([
            (
                NodeID(0),
                vec![(NodeID(1), m01.clone()), (NodeID(3), m03.clone())],
            ),
            (NodeID(1), vec![(NodeID(2), m12.clone())]),
            (NodeID(2), vec![(NodeID(3), m23.clone())]),
            (NodeID(3), vec![]),
        ]);

        let multiplex = edges_to_multiplex(&[
            (0, 1, 0, 0, 1.0),
            (0, 3, 0, 1, 2.0),
            (1, 2, 0, 1, 1.0),
            (2, 3, 1, 1, 1.0),
        ]);
        assert_eq!(edge_list_expected, multiplex);
    }
}
