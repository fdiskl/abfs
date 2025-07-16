use std::fs::File;

use abfs::{agent::run, graph::Pheromones, input::read_graph};

// i got 7578 with 100000 iters, alpha = 1, beta = 2, decay = 0.9

fn main() -> anyhow::Result<()> {
    let mut f = File::open("berlin52.txt")?;

    let (g, n) = read_graph(&mut f)?;

    let mut p = Pheromones::new(n);

    let (path, _) = run(&g, &mut p, 100000, 4, n, 0.9)?;

    println!("{:?}", g.calc_distance(path, n));

    Ok(())
}
