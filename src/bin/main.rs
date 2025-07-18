use abfs::agent::greedy_path;
use chrono::Local;
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

    let mut f = File::open("nearest1000.txt")?;

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
    let n_iters = 500;
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

    run(
        &g,
        &mut Pheromones::new(n, greedy_score),
        n_iters,
        n_ants,
        n,
        rho,
        alpha,
        beta,
        reset_time,
        reset_rho,
        pheta,
        &seed,
    );

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

fn run_simulation(
    g: &Graph,
    answer: &Vec<usize>,
    p: &mut Pheromones,
    n: usize,
    n_iters: usize,
    n_ants: usize,
    alpha: f32,
    beta: f32,
    rho: f32,
    reset_time: usize,
    reset_rho: f32,
    pheta: f32,
    timestamp: &str,
    seed: &[u8; 32],
) -> anyhow::Result<()> {
    let t1 = Local::now();

    let (path, score, scores, phers) = run(
        g, p, n_iters, n_ants, n, rho, alpha, beta, reset_time, reset_rho, pheta, seed,
    )?;

    let t2 = Local::now();

    let params_path = format!("results/params_{}.txt", timestamp);
    write_params(
        &params_path,
        &[
            ("alpha", alpha),
            ("beta", beta),
            ("rho", rho),
            ("reset_time", reset_time as f32),
            ("reset_rho", reset_rho),
            ("pheta", pheta),
            ("ants", n_ants as f32),
            ("iters", n_iters as f32),
            ("score", score),
            ("result", g.calc_distance(&path, n)),
            ("time (ms)", (t2 - t1).num_milliseconds() as f32),
        ],
        &seed,
    );

    // build_graph(timestamp, scores, phers);

    println!("RESULT: {:?}", g.calc_distance(&path, n));
    println!("ANSWER: {:?}", g.calc_distance(&answer, n));
    Ok(())
}

fn rand_range<T: uniform::SampleUniform + Copy + std::cmp::PartialOrd>(
    range: std::ops::Range<T>,
) -> T {
    rand::rng().random_range(range)
}

fn write_params(
    path: &str,
    params: &[(&str, f32)],
    seed_bytes: &[u8; 32],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(path)?;
    for (name, value) in params {
        writeln!(file, "{} = {}", name, value)?;
    }

    writeln!(file, "seed_bytes = {}", hex::encode(seed_bytes))?;

    Ok(())
}

fn build_graph(
    timestamp: &str,
    scores: Vec<f32>,
    phers: Vec<f32>,
) -> Result<(), Box<dyn std::error::Error>> {
    let score_path = format!("results/scores_{}.png", timestamp);
    let pher_path = format!("results/pheromones_{}.png", timestamp);

    plot_single_series(&score_path, "Scores over Time", "Score", &scores)?;
    plot_single_series(&pher_path, "Pheromones over Time", "Pheromone", &phers)?;
    Ok(())
}

fn plot_single_series(
    filename: &str,
    title: &str,
    y_label: &str,
    data: &[f32],
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(filename, (1280, 720)).into_drawing_area();
    root.fill(&WHITE)?;

    let x_range = 0..data.len();
    let y_min = data.iter().copied().fold(f32::INFINITY, f32::min);
    let y_max = data.iter().copied().fold(f32::NEG_INFINITY, f32::max);

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(x_range.clone(), y_min..y_max)?;

    chart
        .configure_mesh()
        .x_desc("Iteration")
        .y_desc(y_label)
        .draw()?;

    chart.draw_series(LineSeries::new(x_range.zip(data.iter().copied()), &RED))?;

    Ok(())
}
