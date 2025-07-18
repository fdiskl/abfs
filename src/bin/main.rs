use abfs::agent::greedy_path;
use chrono::{Local, Utc};
use plotters::prelude::*;
use rand::distr::uniform;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::fs::{File, create_dir_all};
use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::usize::{self, MAX};
use std::{thread, time};

use abfs::{
    agent::run,
    graph::{Graph, Pheromones},
    input::read_graph,
};

fn main() -> anyhow::Result<()> {
    create_dir_all("results")?;

    let mut f = File::open("nearest10000.txt")?;

    println!("Reading graph...");
    let (g, n) = read_graph(&mut f)?;
    println!("We have graph :)");

    let mut seed = [0u8; 32];
    rand::rng().fill(&mut seed);

    let alpha = 2.0;
    let beta = 5.0;
    let rho = 0.5;
    let pheta = 1.0;
    let n_ants = 20;

    let reset_rho = 0.3;
    let reset_time = usize::MAX;
    let n_iters = 10000;
    println!(
        "Running with alpha={:.2}, beta={:.2}, rho={:.2}, reset_time={}, reset_rho={:.2}, pheta={:.2}, ants={}",
        alpha, beta, rho, reset_time, reset_rho, pheta, n_ants
    );

    let (greedy_p, greedy_score) = greedy_path(&g.values, n, 0);
    println!(
        "{:?} {greedy_score:?} {:?}",
        g.calc_distance(&greedy_p, n),
        greedy_p.len()
    );

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        eprintln!("Ctrl+C received, stopping...");
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl+C handler");

    if let Some((best_path, best_score)) = run(
        &g.values,
        &mut Pheromones::new(n, greedy_score),
        greedy_score,
        n,
        n_ants,
        n_iters,
        alpha,
        beta,
        rho,
        pheta,
        reset_rho,
        reset_time,
        Arc::new(AtomicBool::new(true)),
    ) {
        save_best_path(&best_path, best_score)?;
    }

    // ctrlc::set_handler(move || {
    //     r.store(false, Ordering::SeqCst);
    // })?;

    // while keep_running.load(Ordering::SeqCst) {
    //     let mut seed = [0u8; 32];
    //     rand::rng().fill(&mut seed);

    //     let alpha = 1.0;
    //     let beta = 2.0;
    //     let rho = 0.1;
    //     let reset_rho = 0.6;
    //     let reset_time = 200;
    //     let pheta = 1.0;
    //     let n_ants = 4;
    //     let n_iters = 1000;

    //     let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    //     let mut p = Pheromones::new(n);

    //     println!(
    //         "Running with alpha={:.2}, beta={:.2}, rho={:.2}, reset_time={}, reset_rho={:.2}, pheta={:.2}, ants={}",
    //         alpha, beta, rho, reset_time, reset_rho, pheta, n_ants
    //     );

    //     let result = run_simulation(
    //         &g, &answer, &mut p, n, n_iters, n_ants, alpha, beta, rho, reset_time, reset_rho,
    //         pheta, &timestamp, &seed,
    //     );

    //     if let Err(e) = result {
    //         eprintln!("Simulation failed: {:?}", e);
    //     }

    //     thread::sleep(time::Duration::from_millis(100)); // Avoid too rapid looping
    // }

    println!("Exiting gracefully.");
    Ok(())
}

fn save_best_path(path: &[usize], score: f32) -> std::io::Result<()> {
    let now = Utc::now();
    // Format timestamp for filename (e.g. 2025-07-18T11-30-00Z)
    let timestamp = now.format("%Y-%m-%dT%H-%M-%SZ").to_string();

    let filename = format!("best_path_{}.txt", timestamp);
    let mut file = File::create(filename)?;

    writeln!(file, "Best score: {}", score)?;
    writeln!(file, "Best path: {:?}", path)?;
    Ok(())
}
