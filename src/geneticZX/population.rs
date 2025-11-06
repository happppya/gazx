use indicatif::ProgressBar;
use quizx::{vec_graph::Graph, random_graph::EquatorialStabilizerStateBuilder};
use rand::{SeedableRng, rngs::StdRng};

use super::bar_styles;

pub fn build_population(population_size: u32, num_qubits: usize) -> Vec<Graph> {

    let mut random_population: Vec<Graph> = Vec::new();

    let progress_bar = ProgressBar::new( population_size as u64);

    progress_bar.set_style(bar_styles::style_build_population());
    progress_bar.set_message(format!("Building population with size {}", population_size));

    for i in 0..population_size {
        let mut graph = EquatorialStabilizerStateBuilder {
            rng: StdRng::from_os_rng(),
            qubits: num_qubits,
        };
        random_population.push(graph.build());
        progress_bar.inc(1);
    }
    
    progress_bar.finish();

    random_population
}