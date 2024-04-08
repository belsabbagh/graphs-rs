use crate::backend::{self, EdgeValComparable};
use std::collections::HashMap;
pub struct Graph<N, E: EdgeValComparable, B: backend::Backend<EdgeVal = E>> {
    adj_matrix: B,
    nodes: Vec<N>,
}

impl<N, E: EdgeValComparable, B: backend::Backend<EdgeVal = E>> Graph<N, E, B> {
    pub fn new() -> Self {
        Self {
            adj_matrix: B::new(),
            nodes: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: N) {
        self.nodes.push(node);
        self.adj_matrix.add_node();
    }

    pub fn add_edge(&mut self, from: usize, to: usize, weight: E) {
        self.adj_matrix.add_edge(from, to, weight);
    }

    pub fn get_edges(&self, from: usize) -> HashMap<usize, E> {
        let mut edges = HashMap::new();
        for (to, weight) in self.adj_matrix.get_outs(from) {
            edges.insert(to, weight);
        }
        edges
    }

    pub fn get_adj_matrix(&self) -> &B {
        &self.adj_matrix
    }
}
