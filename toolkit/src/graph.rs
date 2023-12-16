pub mod bfs;

pub use bfs::*;

pub trait GraphAdjacents<P, V> {
    fn get_adjacents(&self, index: &P) -> Vec<(P, &V)>;
}

pub struct Node<V> {
    pub value: V,
    pub adjacents: Vec<usize>,
}

pub struct Graph<V> {
    pub nodes: Vec<Node<V>>,
}

impl<V> GraphAdjacents<usize, V> for Graph<V> {
    fn get_adjacents(&self, index: &usize) -> Vec<(usize, &V)> {
        self.nodes[*index]
            .adjacents
            .iter()
            .map(|i| (*i, &self.nodes[*i].value))
            .collect()
    }
}
