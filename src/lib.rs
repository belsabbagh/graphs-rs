pub mod backend;
pub mod graph;

#[cfg(test)]
mod tests {
    use crate::{backend::{self, EdgeValComparable}, graph::Graph};
    
    #[test]
    fn it_works() {
        #[derive(PartialEq, Clone, Debug)]
        struct MyInt(i32);

        impl Default for MyInt {
            fn default() -> Self {
                Self(-1)
            }
        }

        impl EdgeValComparable for MyInt {
            fn compare(&self, other: &Self) -> bool {
                self == other
            }

            fn is_default(&self) -> bool {
                self == &Self::default()
            }
        }
        let mut graph: Graph<MyInt, MyInt, backend::AdjacencyMatrixBackend<MyInt>> = Graph::new();
        graph.add_node(MyInt(1));
        graph.add_node(MyInt(2));
        graph.add_node(MyInt(3));
        graph.add_edge(0, 1, MyInt(1));
        graph.add_edge(0, 2, MyInt(2));
        graph.add_edge(1, 2, MyInt(3));
        println!("Edges from 0: {:?}", graph.get_edges(0));
    }
}