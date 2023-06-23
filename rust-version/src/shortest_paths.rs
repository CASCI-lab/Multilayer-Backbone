use crate::multidistance::{multimin, MultiDistance};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct NodeID(usize);

struct FringeNode {
    node_id: NodeID,
    dists: Vec<MultiDistance>,
}

pub fn parteto_shortest_distance_from_source(
    source: NodeID,
    edge_list: &HashMap<NodeID, Vec<(NodeID, MultiDistance)>>,
) -> HashMap<NodeID, Vec<MultiDistance>> {
    let mut dist_map: HashMap<NodeID, Vec<MultiDistance>> = HashMap::new();

    let initial_dist = MultiDistance::default(); // need to check default

    let mut seen = HashMap::from([(source, vec![initial_dist.clone()])]);

    let mut fringe = vec![FringeNode {
        node_id: source,
        dists: vec![initial_dist],
    }];

    while !fringe.is_empty() {
        let fringe_node = fringe.pop().unwrap();

        // it would be more efficient to handle in-map and not-in-map cases
        // separately; can fix this later
        let fringe_dist = dist_map
            .entry(fringe_node.node_id)
            .or_insert(Vec::new())
            .clone();
        let old_dist = dist_map.get_mut(&fringe_node.node_id).unwrap();
        let mut new_dist = fringe_dist;
        new_dist.append(old_dist);
        new_dist = multimin(&new_dist);
        *dist_map.entry(fringe_node.node_id).or_default() = new_dist;

        if let Some(neighbors) = edge_list.get(&fringe_node.node_id) {
            for (child, edge) in neighbors {
                let mut child_dist = seen.entry(*child).or_insert(Vec::new());

                let mut fringe_to_child_dist = dist_map.get(&fringe_node.node_id).unwrap().clone();
                fringe_to_child_dist = fringe_to_child_dist
                    .iter()
                    .map(|x| x.clone() + edge.clone())
                    .collect();
                fringe_to_child_dist.append(child_dist);
                fringe_to_child_dist = multimin(&fringe_to_child_dist);

                let seen_rep = &child_dist[0].clone(); // all are incomparable
                let fringe_rep = &fringe_to_child_dist[0].clone(); // all are incomparable
                *child_dist = fringe_to_child_dist;
                if fringe_rep < seen_rep {
                    fringe.push(FringeNode {
                        node_id: *child,
                        dists: child_dist.clone(),
                    });
                }
            }
        }
    }

    dist_map
}
