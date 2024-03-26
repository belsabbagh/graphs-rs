use std::collections::HashMap;
use crate::backend;
pub struct Graph<T: backend::EdgeValComparable, B: backend::Backend<T, T>> {
    adj_matrix: B,
    nodes: Vec<T>,
}

impl<T: backend::EdgeValComparable, B: backend::Backend<T, T>> Graph<T, B> {
    pub fn new() -> Self {
        Self {
            adj_matrix: B::new(),
            nodes: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: T) {
        self.nodes.push(node);
        self.adj_matrix.add_node();
    }

    pub fn add_edge(&mut self, from: usize, to: usize, weight: T) {
        self.adj_matrix.add_edge(from, to, weight);
    }

    pub fn get_edges(&self, from: usize) -> HashMap<usize, T> {
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
