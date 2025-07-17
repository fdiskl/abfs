use anyhow::anyhow;
use chrono::Local;
use plotters::prelude::*;
use std::fs::{File, create_dir_all};
use std::io::Write;

use abfs::{
    agent::run,
    graph::{Graph, Pheromones},
    input::{read_answer, read_graph},
};

fn main() -> anyhow::Result<()> {
    create_dir_all("results")?;

    let mut f = File::open("berlin52.txt")?;
    let mut answer_f = File::open("berlin52_answer.txt")?;

    let (g, n) = read_graph(&mut f)?;
    let answer = read_answer(&mut answer_f, n)?;

    let mut p = Pheromones::new(n);

    // ==== Parameters ====
    let n_iters = 10000;
    let n_ants = 4;
    let alpha = 0.2;
    let beta = 2.0;
    let rho = 1.8;
    let reset_time = 1000;
    let reset_rho = 0.6;
    let pheta = 11.0;

    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();

    let (path, score, scores, phers) = run(
        &g, &mut p, n_iters, n_ants, n, rho, alpha, beta, reset_time, reset_rho, pheta,
    )?;

    // ==== Save parameters ====
    let params_path = format!("results/params_{}.txt", timestamp);
    match write_params(
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
        ],
    ) {
        Err(e) => return Err(anyhow!(format!("{e:?}"))),
        _ => {}
    };

    // ==== Save graphs ====
    match build_graph(&timestamp, scores, phers) {
        Err(e) => return Err(anyhow!(format!("{e:?}"))),
        _ => {}
    }

    println!("RESULT: {:?}", g.calc_distance(path, n));
    println!("ANSWER: {:?}", g.calc_distance(answer, n));

    Ok(())
}

fn write_params(path: &str, params: &[(&str, f32)]) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(path)?;
    for (name, value) in params {
        writeln!(file, "{} = {}", name, value)?;
    }
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

    chart.draw_series(LineSeries::new(
        x_range.clone().zip(data.iter().copied()),
        &RED,
    ))?;

    Ok(())
}
