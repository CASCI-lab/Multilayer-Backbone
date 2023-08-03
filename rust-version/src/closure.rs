use crate::{
    multidistance::{MultiDistance, NodeID},
    shortest_paths::parteto_shortest_distance_from_source,
};
use rayon::prelude::*;
use std::{collections::HashMap, hash::BuildHasher};
#[allow(clippy::module_name_repetitions)]
pub type MultidistanceClosure = HashMap<NodeID, HashMap<NodeID, Vec<MultiDistance>>>;

#[allow(clippy::module_name_repetitions)]
#[must_use]
pub fn multidistance_closure<S: BuildHasher + std::marker::Sync>(
    edge_list: &HashMap<NodeID, Vec<(NodeID, MultiDistance)>, S>,
) -> MultidistanceClosure {
    edge_list
        .par_iter()
        .map(|(source, _)| {
            let pareto_dists = parteto_shortest_distance_from_source(*source, edge_list);
            let mut dist_map = HashMap::new();
            dist_map.insert(*source, pareto_dists);
            dist_map
        })
        .reduce(HashMap::new, |a, b| {
            b.iter().fold(a, |mut acc, (k, v)| {
                acc.entry(*k).or_insert(v.clone());
                acc
            })
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::multidistance::EdgeLayerID;
    #[allow(clippy::redundant_clone)]
    #[test]
    fn test_simple_multilayer_closure() {
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

        let expected_from_0: HashMap<NodeID, Vec<MultiDistance>> = HashMap::from([
            (NodeID(1), vec![m01.clone()]),
            (NodeID(2), vec![m01.clone() + m12.clone()]),
            (
                NodeID(3),
                vec![m01.clone() + m12.clone() + m23.clone(), m03.clone()],
            ),
        ]);
        let expected_from_1: HashMap<NodeID, Vec<MultiDistance>> = HashMap::from([
            (NodeID(2), vec![m12.clone()]),
            (NodeID(3), vec![m12.clone() + m23.clone()]),
        ]);
        let expected_from_2: HashMap<NodeID, Vec<MultiDistance>> =
            HashMap::from([(NodeID(3), vec![m23.clone()])]);
        let expected_from_3: HashMap<NodeID, Vec<MultiDistance>> = HashMap::from([]);

        let expected = HashMap::from([
            (NodeID(0), expected_from_0),
            (NodeID(1), expected_from_1),
            (NodeID(2), expected_from_2),
            (NodeID(3), expected_from_3),
        ]);

        let closure = multidistance_closure(&edge_list);

        assert_eq!(expected, closure);
    }
}
