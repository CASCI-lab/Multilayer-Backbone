use crate::{
    bfs_tools::EdgeMap,
    multidistance::{multimin, MultiDistance, NodeID},
};

use std::{
    collections::{HashMap, VecDeque},
    hash::BuildHasher,
};

struct FringeNode {
    node_id: NodeID,
    dists: Vec<MultiDistance>,
    depth: usize,
}

#[must_use]
pub fn parteto_shortest_distance_from_source<S: BuildHasher>(
    source: NodeID,
    edge_map: &EdgeMap<S>,
    max_depth: Option<usize>,
    edge_compare: Option<(&NodeID, &MultiDistance)>,
) -> HashMap<NodeID, Vec<MultiDistance>> {
    let mut dist_map: HashMap<NodeID, Vec<MultiDistance>> = HashMap::new();

    let initial_dist = MultiDistance::default(); // need to check default

    let mut seen = HashMap::from([(source, vec![initial_dist.clone()])]);

    let mut fringe = VecDeque::from([FringeNode {
        node_id: source,
        dists: vec![initial_dist.clone()],
        depth: 0,
    }]);

    while let Some(fringe_node) = fringe.pop_front() {
        if max_depth.is_some_and(|d| fringe_node.depth > d) {
            continue;
        }
        let fringe_dist = dist_map.entry(fringe_node.node_id).or_default().clone();

        let mut old_dist = fringe_node.dists;
        let mut new_dist = fringe_dist;
        new_dist.append(&mut old_dist);
        new_dist = multimin(&new_dist);

        if let Some((t, md)) = edge_compare {
            if fringe_node.node_id == *t && new_dist.iter().any(|x| x < md) {
                *dist_map.entry(fringe_node.node_id).or_default() = new_dist;
                break;
            }
            new_dist.retain(|x| x.partial_cmp(md) != Some(std::cmp::Ordering::Greater)); // !(x > md)
            if new_dist.is_empty() {
                continue;
            }
        }

        *dist_map.entry(fringe_node.node_id).or_default() = new_dist;

        if let Some(neighbors) = edge_map.get(&fringe_node.node_id) {
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

                let mut push_to_fringe = fringe_to_child_dist != *child_dist;
                if max_depth.is_some_and(|d| fringe_node.depth >= d) {
                    push_to_fringe = false;
                }
                *child_dist = fringe_to_child_dist;
                if push_to_fringe {
                    fringe.push_back(FringeNode {
                        node_id: *child,
                        dists: child_dist.clone(),
                        depth: fringe_node.depth + 1,
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

        let edge_map: EdgeMap<_> = HashMap::from([
            (
                NodeID(0),
                vec![(NodeID(1), m01.clone()), (NodeID(2), m02.clone())],
            ),
            (NodeID(1), vec![(NodeID(2), m12)]),
        ]);

        let expected: HashMap<NodeID, Vec<MultiDistance>> =
            HashMap::from([(NodeID(1), vec![m01]), (NodeID(2), vec![m02])]);

        let shortest_paths =
            parteto_shortest_distance_from_source(NodeID(0), &edge_map, None, None);

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

        let edge_map: HashMap<NodeID, Vec<(NodeID, MultiDistance)>> = HashMap::from([
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

        let shortest_paths =
            parteto_shortest_distance_from_source(NodeID(0), &edge_map, None, None);

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

        let edge_map: EdgeMap<_> = HashMap::from([
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

        let shortest_paths =
            parteto_shortest_distance_from_source(NodeID(0), &edge_map, None, None);

        assert_eq!(expected, shortest_paths);
    }
}
