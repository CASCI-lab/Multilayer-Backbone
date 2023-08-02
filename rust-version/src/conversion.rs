use std::collections::HashMap;

use crate::multidistance::{EdgeLayerID, MultiDistance, NodeID};

#[must_use]
pub fn edges_to_multiplex(
    edges: &[(usize, usize, usize, usize, usize, f32)],
) -> HashMap<NodeID, Vec<(NodeID, MultiDistance)>> {
    let mut multiplex = HashMap::new();
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
    use crate::{closure, parteto_shortest_distance_from_source};

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
            (0, 1, 0, 0, 0, 1.0),
            (0, 3, 0, 1, 0, 2.0),
            (1, 2, 0, 1, 0, 1.0),
            (2, 3, 1, 1, 0, 1.0),
        ]);
        assert_eq!(edge_list_expected, multiplex);
    }

    #[test]
    fn test_complicated_example() {
        let edges = [
            (7, 0, 1, 0, 0, 0.0),
            (0, 7, 0, 1, 0, 0.0),
            (8, 4, 1, 0, 0, 0.0),
            (4, 8, 0, 1, 0, 0.0),
            (9, 1, 1, 0, 0, 0.0),
            (1, 9, 0, 1, 0, 0.0),
            (10, 3, 1, 0, 0, 0.0),
            (3, 10, 0, 1, 0, 0.0),
            (11, 5, 1, 0, 0, 0.0),
            (5, 11, 0, 1, 0, 0.0),
            (12, 2, 1, 0, 0, 0.0),
            (2, 12, 0, 1, 0, 0.0),
            (13, 0, 2, 0, 0, 0.0),
            (0, 13, 0, 2, 0, 0.0),
            (13, 6, 2, 1, 0, 0.0),
            (6, 13, 1, 2, 0, 0.0),
            (14, 1, 2, 0, 0, 0.0),
            (1, 14, 0, 2, 0, 0.0),
            (14, 8, 2, 1, 0, 0.0),
            (8, 14, 1, 2, 0, 0.0),
            (15, 4, 2, 0, 0, 0.0),
            (4, 15, 0, 2, 0, 0.0),
            (15, 7, 2, 1, 0, 0.0),
            (7, 15, 1, 2, 0, 0.0),
            (16, 3, 2, 0, 0, 0.0),
            (3, 16, 0, 2, 0, 0.0),
            (16, 9, 2, 1, 0, 0.0),
            (9, 16, 1, 2, 0, 0.0),
            (17, 2, 2, 0, 0, 0.0),
            (2, 17, 0, 2, 0, 0.0),
            (17, 11, 2, 1, 0, 0.0),
            (11, 17, 1, 2, 0, 0.0),
            (18, 5, 2, 0, 0, 0.0),
            (5, 18, 0, 2, 0, 0.0),
            (18, 10, 2, 1, 0, 0.0),
            (10, 18, 1, 2, 0, 0.0),
            (0, 1, 0, 0, 0, 1.0),
            (0, 2, 0, 0, 0, 5.0),
            (0, 3, 0, 0, 0, 1.0),
            (1, 4, 0, 0, 0, 1.0),
            (2, 5, 0, 0, 0, 1.0),
            (3, 2, 0, 0, 0, 1.0),
            (4, 2, 0, 0, 0, 1.0),
            (6, 7, 1, 1, 0, 1.0),
            (7, 8, 1, 1, 0, 1.0),
            (8, 9, 1, 1, 0, 1.0),
            (8, 10, 1, 1, 0, 1.0),
            (9, 11, 1, 1, 0, 1.0),
            (12, 13, 2, 2, 0, 1.0),
            (14, 15, 2, 2, 0, 1.0),
            (16, 17, 2, 2, 0, 1.0),
        ];
        let multiplex = edges_to_multiplex(&edges);
        let _closure = closure::multidistance_closure(&multiplex);
    }
}
