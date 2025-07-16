use std::sync::Arc;

use rand::distr::{Distribution, weighted::WeightedIndex};
use rayon::prelude::*;

use crate::graph::{Graph, Pheromones};

fn find_path(g: &Arc<[f32]>, p: &Vec<f32>, n_nodes: usize, start: usize) -> (Vec<usize>, f32) {
    let mut path = vec![start];
    path.reserve(n_nodes);
    let mut visited = vec![false; n_nodes];
    visited[start] = true;

    let mut rng = rand::rng();
    let mut curr = start;

    let mut cost = 0.0;

    while path.len() < n_nodes {
        let mut choices = vec![];
        choices.reserve(n_nodes);
        let mut weights = vec![];
        weights.reserve(n_nodes);

        for next in 0..n_nodes {
            if visited[next] {
                continue;
            }

            let distance = g[curr * n_nodes + next];
            let pheromone = p[curr * n_nodes + next];

            let alpha = 1.0;
            let beta = 2.0;
            let weight = pheromone.powf(alpha) * (1.0 / distance).powf(beta); // p^alfa * 1/distance^beta

            choices.push(next);
            weights.push(weight);
        }

        let dist = WeightedIndex::new(&weights).unwrap();
        let next = choices[dist.sample(&mut rng)];

        cost += g[curr * n_nodes + next]; // update cost based on distance

        visited[next] = true;
        path.push(next);
        curr = next;
    }

    (path, cost)
}

pub fn run(
    g: &Graph,
    p: &mut Pheromones,
    n_iters: usize,
    n_ants: usize,
    n_nodes: usize,
    evap_mult: f32,
) -> anyhow::Result<()> {
    let graph = &g.values;

    for _ in 0..n_iters {
        let all_deltas: Vec<Vec<f32>> = (0..n_ants)
            .into_par_iter()
            .map(|_| {
                let mut delta = vec![0.0f32; n_nodes * n_nodes];

                let (path, score) = find_path(graph, &p.values, n_nodes, 0);

                for i in 0..path.len() - 1 {
                    let from = path[i];
                    let to = path[i + 1];
                    delta[from * n_nodes + to] += score;
                    delta[to * n_nodes + from] += score;
                }

                delta
            })
            .collect();

        // global update (multi threaded too)
        let pher = &mut p.values;
        pher.par_iter_mut().enumerate().for_each(|(i, pher_val)| {
            *pher_val *= evap_mult;
            for delta in &all_deltas {
                *pher_val += delta[i];
            }
        });
    }

    Ok(())
}
