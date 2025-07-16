use std::sync::Arc;
use std::thread;

use crate::graph::{Graph, Pheromones};

const EVAP_MULT: f32 = 0.9;

pub fn run(
    g: &Graph,
    p: &mut Pheromones,
    n_iters: usize,
    n_ants: usize,
    n_nodes: usize,
) -> anyhow::Result<()> {
    let graph = Arc::clone(&g.values); // shared read-only

    for _ in 0..n_iters {
        // Launch threads for ants
        let mut handles = Vec::with_capacity(n_ants);

        for _ in 0..n_ants {
            let graph = Arc::clone(&graph);

            let handle = thread::spawn(move || {
                let mut delta = vec![0.0f32; n_nodes * n_nodes];

                // TODO Build ant path here
                let path = vec![0, 2, 3, 1];
                let score = 1.0;

                // update local pheromone delta
                for i in 0..path.len() - 1 {
                    let from = path[i];
                    let to = path[i + 1];
                    delta[from * n_nodes + to] += score;
                    delta[to * n_nodes + from] += score;
                }

                delta
            });

            handles.push(handle);
        }

        // collect all deltas
        let mut all_deltas = Vec::with_capacity(n_ants);
        for handle in handles {
            let delta = handle.join().unwrap();
            all_deltas.push(delta);
        }

        // global update (single-threaded tho)
        let pher = &mut p.values;
        for i in 0..n_nodes * n_nodes {
            pher[i] *= EVAP_MULT;
            for delta in &all_deltas {
                pher[i] += delta[i];
            }
        }
    }

    Ok(())
}
