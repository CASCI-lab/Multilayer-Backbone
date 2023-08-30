use crate::{
    multidistance::{multimin, MultiDistance, NodeID},
    multigraph::MultidistanceGraph,
};

use std::collections::{HashMap, VecDeque};

struct FringeNode {
    node_id: NodeID,
    dists: Vec<MultiDistance>,
    depth: usize,
}

#[must_use]
pub fn parteto_shortest_distance_from_source(
    source: NodeID,
    graph: &impl MultidistanceGraph,
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

        for (child, edge) in &graph.neighbor_edges(&fringe_node.node_id) {
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

    if dist_map[&source] == vec![initial_dist] {
        dist_map.remove(&source);
    }
    dist_map
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::multigraph::MultidistanceGraphHashmap;
    #[test]
    fn test_simple_single_layer_shortest_path() {
        let m01 = MultiDistance::from_tuple(0, 0, 0, 2.0);
        let m02 = MultiDistance::from_tuple(0, 0, 0, 4.0);

        let expected: HashMap<NodeID, Vec<MultiDistance>> =
            HashMap::from([(NodeID(1), vec![m01]), (NodeID(2), vec![m02])]);

        let graph = MultidistanceGraphHashmap::from_tuple_edge_list(&[
            (0, 1, 0, 0, 0, 2.0),
            (0, 2, 0, 0, 0, 4.0),
            (1, 2, 0, 0, 0, 2.0),
        ]);
        let shortest_paths = parteto_shortest_distance_from_source(NodeID(0), &graph, None, None);

        assert_eq!(expected, shortest_paths);
    }

    #[test]
    fn test_simple_multilayer_shortest_path() {
        let m01 = MultiDistance::from_tuple(0, 0, 0, 1.0);
        let m03 = MultiDistance::from_tuple(0, 1, 0, 2.0);
        let m12 = MultiDistance::from_tuple(0, 1, 0, 1.0);
        let m23 = MultiDistance::from_tuple(1, 1, 0, 1.0);

        let graph = MultidistanceGraphHashmap::from_tuple_edge_list(&[
            (0, 1, 0, 0, 0, 1.0),
            (0, 3, 0, 1, 0, 2.0),
            (1, 2, 0, 1, 0, 1.0),
            (2, 3, 1, 1, 0, 1.0),
        ]);

        let expected: HashMap<NodeID, Vec<MultiDistance>> = HashMap::from([
            (NodeID(1), vec![m01.clone()]),
            (NodeID(2), vec![m01.clone() + m12.clone()]),
            (NodeID(3), vec![m01 + m12 + m23, m03]),
        ]);

        let shortest_paths = parteto_shortest_distance_from_source(NodeID(0), &graph, None, None);

        assert_eq!(expected, shortest_paths);
    }

    #[test]
    fn test_simple_cycle_shortest_path() {
        let m0 = MultiDistance::from_tuple(0, 0, 0, 2.0);
        let m1 = MultiDistance::from_tuple(0, 0, 0, 4.0);

        let graph = MultidistanceGraphHashmap::from_tuple_edge_list(&[
            (0, 1, 0, 0, 0, 2.0),
            (0, 2, 0, 0, 0, 4.0),
            (1, 0, 0, 1, 0, 2.0),
            (1, 2, 0, 0, 0, 2.0),
            (2, 1, 0, 0, 0, 2.0),
        ]);

        let expected: HashMap<NodeID, Vec<MultiDistance>> =
            HashMap::from([(NodeID(1), vec![m0]), (NodeID(2), vec![m1])]);

        let shortest_paths = parteto_shortest_distance_from_source(NodeID(0), &graph, None, None);

        assert_eq!(expected, shortest_paths);
    }
}
