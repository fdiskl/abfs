use std::sync::{Arc, Mutex};

pub struct Graph {
    pub values: Arc<[f32]>,
}

pub struct Pheromones {
    pub values: Vec<f32>, // len = n_nodes * n_nodes
}
