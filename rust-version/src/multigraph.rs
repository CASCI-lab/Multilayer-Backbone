use std::collections::HashMap;

use crate::multidistance::{MultiDistance, NodeID};

pub trait MultidistanceGraph {
    fn nodes(&self) -> Vec<NodeID>;
    fn add_edge(&mut self, from: NodeID, to: NodeID, weight: MultiDistance);
    fn remove_edge(&mut self, from: NodeID, to: NodeID);
    fn neighbor_edges(&self, node: &NodeID) -> Vec<(NodeID, MultiDistance)>;
    fn edge_weight(&self, from: NodeID, to: NodeID) -> Option<&MultiDistance>;
}

pub trait ClosureGraph {
    fn extend_edge<I>(&mut self, from: NodeID, to: NodeID, weights: I)
    where
        I: IntoIterator<Item = MultiDistance>;

    fn remove_edge(&mut self, from: NodeID, to: NodeID);
    fn neighbors(&self, node: NodeID) -> Vec<NodeID>;
    fn edge_weight(&self, from: NodeID, to: NodeID) -> Option<&MultiDistance>;
}

#[derive(Default, Clone)]
pub struct MultidistanceGraphHashmap {
    pub(crate) edges: HashMap<NodeID, HashMap<NodeID, MultiDistance>>,
}

impl MultidistanceGraphHashmap {
    #[must_use]
    pub fn new() -> MultidistanceGraphHashmap {
        MultidistanceGraphHashmap {
            edges: HashMap::new(),
        }
    }

    #[must_use]
    pub fn from_multidistance_edge_list(
        edges: Vec<(NodeID, NodeID, MultiDistance)>,
    ) -> MultidistanceGraphHashmap {
        let mut graph = MultidistanceGraphHashmap::new();
        for (edge_from, edge_to, weight) in edges {
            graph.add_edge(edge_from, edge_to, weight);
        }
        graph
    }

    #[must_use]
    pub fn from_tuple_edge_list(
        edges: &[(usize, usize, usize, usize, usize, f32)],
    ) -> MultidistanceGraphHashmap {
        let mut graph = MultidistanceGraphHashmap::new();
        for (edge_from, edge_to, layer_start, layer_end, layer_weight_index, weight) in edges {
            let edge_weight =
                MultiDistance::from_tuple(*layer_start, *layer_end, *layer_weight_index, *weight);

            graph.add_edge(NodeID(*edge_from), NodeID(*edge_to), edge_weight);
        }
        graph
    }
}

impl MultidistanceGraph for MultidistanceGraphHashmap {
    fn nodes(&self) -> Vec<NodeID> {
        self.edges.keys().copied().collect()
    }

    fn add_edge(&mut self, from: NodeID, to: NodeID, weight: MultiDistance) {
        self.edges.entry(from).or_default().insert(to, weight);
        self.edges.entry(to).or_insert(HashMap::default()); // to ensure that sink nodes appear in node list
    }

    fn remove_edge(&mut self, from: NodeID, to: NodeID) {
        if let Some(neighbors) = self.edges.get_mut(&from) {
            neighbors.remove(&to);
        }
    }

    fn neighbor_edges(&self, node: &NodeID) -> Vec<(NodeID, MultiDistance)> {
        self.edges
            .get(node)
            .unwrap_or(&HashMap::new())
            .iter()
            .map(|(k, v)| (*k, v.clone()))
            .collect()
    }

    fn edge_weight(&self, from: NodeID, to: NodeID) -> Option<&MultiDistance> {
        self.edges.get(&from).and_then(|x| x.get(&to))
    }
}
