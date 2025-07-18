use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Graph {
    pub values: Arc<[f32]>, // len = n_nodes * n_nodes
}

impl Graph {
    pub fn calc_distance(&self, path: &Vec<usize>, n: usize) -> f32 {
        assert_eq!(path.len(), n + 1);
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
    pub fn new(n: usize, greedy_n: f32) -> Self {
        Self {
            values: vec![2.0 / greedy_n; n * n],
        }
    }

    pub fn reset(&mut self) {
        let len = self.values.len();
        for val in self.values.iter_mut().take(len - 1) {
            *val = 0.0;
        }
    }
}
