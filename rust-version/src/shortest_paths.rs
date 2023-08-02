use crate::multidistance::{multimin, MultiDistance, NodeID};
use std::{collections::HashMap, hash::BuildHasher};

struct FringeNode {
    node_id: NodeID,
    dists: Vec<MultiDistance>,
}

#[must_use]
pub fn parteto_shortest_distance_from_source<S: BuildHasher>(
    source: NodeID,
    edge_list: &HashMap<NodeID, Vec<(NodeID, MultiDistance)>, S>,
) -> HashMap<NodeID, Vec<MultiDistance>> {
    let mut dist_map: HashMap<NodeID, Vec<MultiDistance>> = HashMap::new();

    let initial_dist = MultiDistance::default(); // need to check default

    let mut seen = HashMap::from([(source, vec![initial_dist.clone()])]);

    let mut fringe = vec![FringeNode {
        node_id: source,
        dists: vec![initial_dist.clone()],
    }];

    while let Some(fringe_node) = fringe.pop() {
        // it would be more efficient to handle in-map and not-in-map cases
        // separately; can fix this later
        let fringe_dist = dist_map
            .entry(fringe_node.node_id)
            .or_insert(Vec::new())
            .clone();

        let mut old_dist = fringe_node.dists;
        let mut new_dist = fringe_dist;
        new_dist.append(&mut old_dist);
        new_dist = multimin(&new_dist);
        *dist_map.entry(fringe_node.node_id).or_default() = new_dist;

        if let Some(neighbors) = edge_list.get(&fringe_node.node_id) {
            for (child, edge) in neighbors {
                let child_dist = seen.entry(*child).or_insert(Vec::new());

                let mut fringe_to_child_dist = dist_map
                    .get(&fringe_node.node_id)
                    .unwrap_or(&vec![MultiDistance::default()])
                    .clone();
                fringe_to_child_dist = fringe_to_child_dist
                    .iter()
                    .map(|x| x.clone() + edge.clone())
                    .collect();
                fringe_to_child_dist.extend(child_dist.iter().cloned());
                fringe_to_child_dist = multimin(&fringe_to_child_dist);

                let push_to_fringe = fringe_to_child_dist != *child_dist;

                // let push_to_fringe = if child_dist.is_empty() {
                //     true
                // } else {
                //     let seen_rep = &child_dist[0].clone(); // all are incomparable
                //     let fringe_rep = &fringe_to_child_dist[0].clone(); // all are incomparable
                //     fringe_rep < seen_rep
                // };
                *child_dist = fringe_to_child_dist;
                if push_to_fringe {
                    fringe.push(FringeNode {
                        node_id: *child,
                        dists: child_dist.clone(),
                    });
                }
            }
        }
    }
    if dist_map[&source] == vec![initial_dist] {
        dist_map.remove(&source);
    }
    dist_map
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::multidistance::EdgeLayerID;
    #[test]
    fn test_simple_single_layer_shortest_path() {
        let layer1 = EdgeLayerID {
            layer_start: 0,
            layer_end: 0,
            layer_weight_index: 0,
        };

        let m01 = MultiDistance {
            total: HashMap::from([(layer1, 2.0)]),
        };

        let m02 = MultiDistance {
            total: HashMap::from([(layer1, 4.0)]),
        };

        let m12 = MultiDistance {
            total: HashMap::from([(layer1, 2.0)]),
        };

        let edge_list: HashMap<NodeID, Vec<(NodeID, MultiDistance)>> = HashMap::from([
            (
                NodeID(0),
                vec![(NodeID(1), m01.clone()), (NodeID(2), m02.clone())],
            ),
            (NodeID(1), vec![(NodeID(2), m12)]),
        ]);

        let expected: HashMap<NodeID, Vec<MultiDistance>> =
            HashMap::from([(NodeID(1), vec![m01]), (NodeID(2), vec![m02])]);

        let shortest_paths = parteto_shortest_distance_from_source(NodeID(0), &edge_list);

        assert_eq!(expected, shortest_paths);
    }

    #[test]
    fn test_simple_multilayer_shortest_path() {
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

        let edge_list: HashMap<NodeID, Vec<(NodeID, MultiDistance)>> = HashMap::from([
            (
                NodeID(0),
                vec![(NodeID(1), m01.clone()), (NodeID(3), m03.clone())],
            ),
            (NodeID(1), vec![(NodeID(2), m12.clone())]),
            (NodeID(2), vec![(NodeID(3), m23.clone())]),
            (NodeID(3), vec![]),
        ]);

        let expected: HashMap<NodeID, Vec<MultiDistance>> = HashMap::from([
            (NodeID(1), vec![m01.clone()]),
            (NodeID(2), vec![m01.clone() + m12.clone()]),
            (NodeID(3), vec![m01 + m12 + m23, m03]),
        ]);

        let shortest_paths = parteto_shortest_distance_from_source(NodeID(0), &edge_list);

        assert_eq!(expected, shortest_paths);
    }

    #[test]
    fn test_simple_cycle_shortest_path() {
        let layer1 = EdgeLayerID {
            layer_start: 0,
            layer_end: 0,
            layer_weight_index: 0,
        };

        let m0 = MultiDistance {
            total: HashMap::from([(layer1, 2.0)]),
        };

        let m1 = MultiDistance {
            total: HashMap::from([(layer1, 4.0)]),
        };

        let edge_list: HashMap<NodeID, Vec<(NodeID, MultiDistance)>> = HashMap::from([
            (
                NodeID(0),
                vec![(NodeID(1), m0.clone()), (NodeID(2), m1.clone())],
            ),
            (
                NodeID(1),
                vec![(NodeID(0), m0.clone()), (NodeID(2), m0.clone())],
            ),
            (NodeID(2), vec![(NodeID(1), m0.clone())]),
        ]);

        let expected: HashMap<NodeID, Vec<MultiDistance>> =
            HashMap::from([(NodeID(1), vec![m0]), (NodeID(2), vec![m1])]);

        let shortest_paths = parteto_shortest_distance_from_source(NodeID(0), &edge_list);

        assert_eq!(expected, shortest_paths);
    }
}
