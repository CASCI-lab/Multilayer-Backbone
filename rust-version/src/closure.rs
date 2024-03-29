use crate::{
    multidistance::{MultiDistance, NodeID},
    shortest_paths::parteto_shortest_distance_from_source,
    MultidistanceGraph,
};
use rayon::prelude::*;
use std::collections::HashMap;
#[allow(clippy::module_name_repetitions)]
pub type MultidistanceClosure = HashMap<NodeID, HashMap<NodeID, Vec<MultiDistance>>>;

#[allow(clippy::module_name_repetitions)]
#[must_use]
pub fn multidistance_closure(graph: &(impl MultidistanceGraph + Sync)) -> MultidistanceClosure {
    graph
        .nodes()
        .par_iter()
        .map(|source| {
            let pareto_dists = parteto_shortest_distance_from_source(*source, graph, None, None);
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
    use crate::MultidistanceGraphHashmap;
    #[allow(clippy::redundant_clone)]
    #[test]
    fn test_simple_multilayer_closure() {
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

        let closure = multidistance_closure(&graph);

        assert_eq!(expected, closure);
    }
}
