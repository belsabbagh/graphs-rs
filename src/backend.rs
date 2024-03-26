pub trait EdgeValComparable: PartialEq +Default +Clone {
  fn compare(&self, other: &Self) -> bool;

  fn is_default(&self) -> bool {
      self == &Self::default()
  }
}
pub trait Backend<NodeVal, EdgeVal: EdgeValComparable> {
    fn new() -> Self;
    fn add_node(&mut self);
    fn add_edge(&mut self, from: usize, to: usize, weight: EdgeVal);
    fn get_outs(&self, from: usize) -> Vec<(usize, EdgeVal)>;
    fn get_ins(&self, to: usize) -> Vec<(usize, EdgeVal)>;
    fn get_edges(&self, node: usize) -> Vec<(usize, EdgeVal)> {
        let mut edges = Vec::new();

        edges.extend(self.get_outs(node));
        edges.extend(self.get_ins(node));

        edges
    }
}

#[derive(Debug)]
pub struct AdjacencyMatrixBackend<T: EdgeValComparable> {
    data: Vec<Vec<T>>,
}

impl<T: EdgeValComparable> Backend<T, T> for AdjacencyMatrixBackend<T> {
    fn new() -> Self {
        Self { data: Vec::new() }
    }
    fn add_node(&mut self) {
        self.data.push(vec![T::default(); self.data.len()]);
        let length = self.data.len();
        for row in self.data.iter_mut() {
            row.resize(length, T::default());
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, weight: T) {
        self.data[from][to] = weight;
    }

    fn get_outs(&self, from: usize) -> Vec<(usize, T)> {
        let mut edges = Vec::new();
        for (i, weight) in self.data[from].iter().enumerate() {
            if !weight.is_default() {
                edges.push((i, weight.clone()));
            }
        }

        edges
    }

    fn get_ins(&self, to: usize) -> Vec<(usize, T)> {
        let mut edges = Vec::new();
        for (i, weight) in self.data[to].iter().enumerate() {
            if !weight.is_default() {
                edges.push((i, weight.clone()));
            }

        }

        edges
    }
}

pub struct AdjacencyListBackend<T: EdgeValComparable> {
    data: Vec<Vec<(usize, T)>>,
}

impl<T: EdgeValComparable> Backend<T, T> for AdjacencyListBackend<T> {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn add_node(&mut self) {
        self.data.push(Vec::new());
    }

    fn add_edge(&mut self, from: usize, to: usize, weight: T) {
        self.data[from].push((to, weight));
    }

    fn get_outs(&self, from: usize) -> Vec<(usize, T)> {
        self.data[from].clone()
    }

    fn get_ins(&self, to: usize) -> Vec<(usize, T)> {
        self.data[to].clone()
    }

    fn get_edges(&self, node: usize) -> Vec<(usize, T)> {
        self.data[node].clone()
    }

}
