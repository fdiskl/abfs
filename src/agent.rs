use std::sync::Arc;

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
    seed: [u8; 32],
) -> (Vec<usize>, f32) {
    let mut path = vec![start];
    path.reserve(n_nodes + 1);
    let mut visited = vec![false; n_nodes];
    visited[start] = true;

    let mut curr = start;

    let mut rng = SmallRng::from_seed(seed);

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
        let next = choices[dist.sample(&mut rng)];

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
    g: &Graph,
    p: &mut Pheromones,
    n_iters: usize, // this will be ignored now
    n_ants: usize,
    n_nodes: usize,
    mut rho: f32,
    mut alpha: f32,
    mut beta: f32,
    reset_time: usize,
    reset_rho: f32,
    pheta: f32,
    seed: &[u8; 32],
) -> anyhow::Result<(Vec<usize>, f32, Vec<f32>, Vec<f32>)> {
    let graph = &g.values;

    let best_path = Arc::new(Mutex::new((vec![], f32::INFINITY)));
    let running = Arc::new(AtomicBool::new(true));

    // Clone handles for signal handler
    let best_path_clone = Arc::clone(&best_path);
    let running_clone = Arc::clone(&running);

    ctrlc::set_handler(move || {
        println!("\nInterrupted! Saving best path...");
        running_clone.store(false, Ordering::SeqCst);

        let (path, score) = &*best_path_clone.lock();
        if !path.is_empty() {
            let mut file = File::create("best_path.txt").expect("Failed to create file");
            writeln!(file, "score = {:.4}", score).ok();
            writeln!(file, "path = {:?}", path).ok();
            println!("Best path saved to 'best_path.txt'");
        } else {
            println!("No path found to save.");
        }
    })?;

    let tau = 10.0;
    let mut stagnation_counter = 0;

    let mut master_rng = ChaCha8Rng::from_seed(*seed);

    let mut j = 0;
    while running.load(Ordering::SeqCst) {
        println!("{j:?} {:?}", best_path.clone().lock().1);

        let progress = (j as f32).min(1000.0) / 1000.0; // dampen beyond 1000 iters

        let alpha_adj = alpha * (1.0 + progress);
        let beta_adj = beta * (1.0 - progress * 0.5);
        let rho_adj = rho * (1.0 - progress * 0.5);

        let runs: Vec<([u8; 32], usize)> = (0..n_ants)
            .map(|_| {
                let mut s = [0u8; 32];
                master_rng.fill_bytes(&mut s);
                let idx = master_rng.random_range(0..n_nodes);
                (s, idx)
            })
            .collect();

        let improved;

        let results: Vec<(Vec<usize>, f32)> = runs
            .into_par_iter()
            .map(|(s, to_find)| {
                find_path(graph, &p.values, n_nodes, to_find, alpha_adj, beta_adj, s)
            })
            .collect();

        let mut deltas = vec![0.0f32; n_nodes * n_nodes];

        let (best_this_iter_path, best_this_iter_score) = results
            .iter()
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap();

        let mut best = best_path.lock();
        if *best_this_iter_score < best.1 {
            best.0 = best_this_iter_path.clone();
            best.1 = *best_this_iter_score;
            improved = true;
        } else {
            improved = false;
        }

        // Only reinforce best path of this iteration
        let influence = pheta * ((1.0 / best_this_iter_score).powf(1.0 + 0.5 * progress));

        for i in 0..best_this_iter_path.len() - 1 {
            let from = best_this_iter_path[i];
            let to = best_this_iter_path[i + 1];

            let idx1 = from * n_nodes + to;
            let idx2 = to * n_nodes + from;

            deltas[idx1] += influence;
            deltas[idx2] += influence;
        }

        if !improved {
            stagnation_counter += 1;
        } else {
            stagnation_counter = 0;
        }

        if stagnation_counter >= 150 {
            println!("Resetting pheromones due to stagnation");
            for val in &mut p.values {
                *val = 1.0;
            }
            stagnation_counter = 0;
            j += 1;
            continue;
        }

        let rho_strong = if stagnation_counter >= 50 {
            rho_adj * 2.0
        } else {
            rho_adj
        };

        p.values
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, pher_val)| {
                // Evaporation
                *pher_val *= 1.0 - rho_strong;

                // Deposit (from best path delta matrix)
                *pher_val += deltas[i];

                // Clamp to avoid zero or nan
                if !pher_val.is_finite() || *pher_val <= 1e-8 {
                    *pher_val = 1e-6;
                } else if *pher_val > 1e6 {
                    *pher_val = 1e6;
                }
            });

        j += 1;
    }

    let best = best_path.lock();
    Ok((best.0.clone(), best.1, vec![], vec![]))
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
