use std::sync::Arc;

use rand::distr::{Distribution, weighted::WeightedIndex};
use rayon::prelude::*;

use crate::graph::{Graph, Pheromones};

fn find_path(
    g: &Arc<[f32]>,
    p: &Vec<f32>,
    n_nodes: usize,
    start: usize,
    alpha: f32,
    beta: f32,
) -> (Vec<usize>, f32) {
    let mut path = vec![start];
    path.reserve(n_nodes + 1);
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

            let mut weight = pheromone.powf(alpha) * (1.0 / (distance)).powf(beta); // p^alfa * 1/distance^beta

            if !weight.is_finite() || weight <= 0.0 {
                weight = 1e-6;
            }

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

    path.push(start);

    (path, cost)
}

pub fn run(
    g: &Graph,
    p: &mut Pheromones,
    n_iters: usize,
    n_ants: usize,
    n_nodes: usize,
    rho: f32,
    alpha: f32,
    beta: f32,
    reset_time: usize,
    reset_rho: f32,
    pheta: f32,
) -> anyhow::Result<(Vec<usize>, f32, Vec<f32>, Vec<f32>)> {
    let graph = &g.values;

    let mut best_path = vec![];
    let mut best_score = f32::INFINITY;

    let mut all_scores = vec![];
    all_scores.reserve(n_iters);

    let mut all_pheromones_sum = vec![];
    all_pheromones_sum.reserve(n_iters);

    for i in 0..n_iters {
        let results: Vec<(Vec<usize>, f32, Vec<f32>)> = (0..n_ants)
            .into_par_iter()
            .map(|_| {
                let (path, score) = find_path(graph, &p.values, n_nodes, 0, alpha, beta);
                let mut delta = vec![0.0f32; n_nodes * n_nodes];

                for i in 0..path.len() - 1 {
                    let from = path[i];
                    let to = path[i + 1];
                    delta[from * n_nodes + to] += pheta / score;
                    delta[to * n_nodes + from] += pheta / score;
                }

                (path, score, delta)
            })
            .collect();

        let pher = &mut p.values;

        // global update (multi threaded too)
        let pher_sum: f32 = pher
            .par_iter_mut()
            .enumerate()
            .map(|(i, pher_val)| {
                if i % reset_time == 0 {
                    *pher_val *= 1.0 - reset_rho;
                } else {
                    *pher_val *= 1.0 - rho;
                }
                for (_, _, delta) in &results {
                    *pher_val += delta[i];
                }
                *pher_val
            })
            .sum();

        all_pheromones_sum.push(pher_sum);

        let mut avg_score: f32 = 0.0;
        let res_len = results.len();
        for (path, score, _) in results {
            avg_score += score;
            if score < best_score {
                best_score = score;
                best_path = path;
            }
        }

        avg_score /= res_len as f32;
        all_scores.push(avg_score);
    }

    Ok((best_path, best_score, all_scores, all_pheromones_sum))
}
