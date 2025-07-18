use std::{fs::OpenOptions, sync::Arc};

use rand::{
    Rng, RngCore, SeedableRng,
    distr::{Distribution, weighted::WeightedIndex},
    rngs::SmallRng,
};
use rand_chacha::ChaCha8Rng;
use rayon::prelude::*;

use crate::graph::{Graph, Pheromones};

fn find_path(
    g: &Arc<[f32]>,
    p: &Vec<f32>,
    n_nodes: usize,
    start: usize,
    alpha: f32,
    beta: f32,
    rng: &mut SmallRng,
) -> (Vec<usize>, f32) {
    let mut path = vec![start];
    path.reserve(n_nodes + 1);
    let mut visited = vec![false; n_nodes];
    visited[start] = true;

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

            let base = pheromone.powf(alpha) * (1.0 / (distance + 1e-6)).powf(beta);
            let weight = base * (1.0 + rng.gen_range(0.01..0.05));

            if weight.is_finite() {
                choices.push(next);
                weights.push(weight);
            }
        }

        let dist = WeightedIndex::new(&weights).unwrap();
        let next = choices[dist.sample(rng)];

        cost += g[curr * n_nodes + next]; // update cost based on distance

        visited[next] = true;
        path.push(next);
        curr = next;
    }

    path.push(start);
    cost += g[curr * n_nodes + start];

    (path, cost)
}

use ctrlc;
use parking_lot::Mutex;
use std::{
    fs::File,
    io::Write,
    sync::atomic::{AtomicBool, Ordering},
};

pub fn run(
    g: &Arc<[f32]>,
    p: &mut Pheromones,
    greedy_score: f32,
    n_nodes: usize,
    n_ants: usize,
    n_iters: usize,
    alpha: f32,
    beta: f32,
    rho: f32,
    pheta: f32,
    reset_rho: f32,
    reset_time: usize,
    running: Arc<AtomicBool>,
) -> Option<(Vec<usize>, f32)> {
    let mut best_path = None;
    let mut best_score = f32::MAX;
    let mut stagnation_counter = 0;
    let initial_pheromone = 1.0 / (n_nodes as f32 * n_nodes as f32);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("best_path_log.txt")
        .expect("Failed to open best_path_log.txt");

    for j in 0..n_iters {
        if !running.load(Ordering::SeqCst) {
            eprintln!("Interrupted at iteration {j}, saving best path...");
            break;
        }

        let mut master_rng = rand::rng();

        let seeds: Vec<[u8; 32]> = (0..n_ants)
            .map(|_| {
                let mut seed = [0u8; 32];
                master_rng.fill_bytes(&mut seed);
                seed
            })
            .collect();

        let results: Vec<_> = seeds
            .into_par_iter()
            .map(|seed| {
                let mut rng = SmallRng::from_seed(seed);
                let start = rng.next_u32() as usize % n_nodes;
                find_path(g, &p.values, n_nodes, start, alpha, beta, &mut rng)
            })
            .collect();

        // Find best path this iteration
        let (best_this_iter_path, best_this_iter_score) = results
            .iter()
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap();

        if *best_this_iter_score < best_score {
            best_score = *best_this_iter_score;
            best_path = Some(best_this_iter_path.clone());
            stagnation_counter = 0;

            writeln!(
                file,
                "Iteration {j}, New Best Score: {best_score}\nBest Path: {:?}\n",
                best_this_iter_path
            )
            .expect("Failed to write to best_path_log.txt");

            file.flush().expect("Failed to flush file");
        } else {
            stagnation_counter += 1;
        }

        // Reset if stagnating
        if stagnation_counter >= reset_time {
            for val in &mut p.values {
                *val = initial_pheromone;
            }
            stagnation_counter = 0;
            continue;
        }

        // --- Build delta matrix ---
        let mut deltas = vec![0.0; n_nodes * n_nodes];
        let progress = j as f32 / n_iters as f32;

        for (path, cost) in &results {
            if !cost.is_finite() || *cost <= 0.0 {
                continue;
            }

            // More emphasis on better paths over time
            let influence = pheta * ((1.0 / cost).powf(1.0 + 0.5 * progress));

            for i in 0..path.len() - 1 {
                let from = path[i];
                let to = path[i + 1];
                let idx1 = from * n_nodes + to;
                let idx2 = to * n_nodes + from;

                deltas[idx1] += influence;
                deltas[idx2] += influence;
            }
        }

        // --- Pheromone evaporation + reinforcement ---
        let rho_adj = if stagnation_counter >= 50 {
            rho * 2.0
        } else {
            rho
        };

        p.values
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, pher_val)| {
                *pher_val *= 1.0 - rho_adj;
                *pher_val += deltas[i];

                // Clamp values to avoid degenerate values
                if !pher_val.is_finite() || *pher_val <= 1e-8 {
                    *pher_val = 1e-6;
                } else if *pher_val > 1e6 {
                    *pher_val = 1e6;
                }
            });

        eprintln!("Iteration {j}, Best Score: {best_score}");
    }

    if let Some(ref path) = best_path {
        if let Err(e) = std::fs::write(
            "best_path.txt",
            format!("Best score: {}\nBest path: {:?}\n", best_score, path),
        ) {
            eprintln!("Failed to save best path: {}", e);
        }
    }

    best_path.map(|path| (path, best_score))
}

pub fn greedy_path(g: &Arc<[f32]>, n_nodes: usize, start: usize) -> (Vec<usize>, f32) {
    let mut path = vec![start];
    path.reserve(n_nodes + 1);
    let mut visited = vec![false; n_nodes];
    visited[start] = true;

    let mut cost = 0.0;
    let mut curr = start;

    while path.len() < n_nodes {
        let mut next = None;
        let mut min_dist = f32::INFINITY;

        for i in 0..n_nodes {
            if visited[i] {
                continue;
            }

            let dist = g[curr * n_nodes + i];
            if dist < min_dist {
                min_dist = dist;
                next = Some(i);
            }
        }

        let next = next.expect("No unvisited nodes found, but path is incomplete");
        visited[next] = true;
        path.push(next);
        cost += g[curr * n_nodes + next];
        curr = next;
    }

    // Return to start
    path.push(start);
    cost += g[curr * n_nodes + start];

    (path, cost)
}
