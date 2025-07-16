use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Graph {
    pub values: Arc<[f32]>, // len = n_nodes * n_nodes
}

impl Graph {
    pub fn calc_distance(&self, path: Vec<usize>, n: usize) -> f32 {
        let mut res = 0.0;
        for i in 0..path.len() - 1 {
            let from = path[i];
            let to = path[i + 1];
            res += self.values[from * n + to];
        }
        res
    }
}

pub struct Pheromones {
    pub values: Vec<f32>, // len = n_nodes * n_nodes
}

impl Pheromones {
    pub fn new(n: usize) -> Self {
        Self {
            values: vec![1000.0; n * n],
        }
    }
}
