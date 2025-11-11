use indicatif::ProgressBar;
use quizx::{generate::RandomCircuitBuilder, random_graph::EquatorialStabilizerStateBuilder, vec_graph::Graph};
use rand::{SeedableRng, rngs::StdRng};

use super::bar_styles;

pub fn build_population(population_size: u32, num_qubits: usize) -> Vec<Graph> {

    let mut random_population: Vec<Graph> = Vec::new();

    let mut random_circuit_builder = RandomCircuitBuilder {
        rng: StdRng::from_os_rng(),
        qubits: num_qubits,
        depth: 4,
        p_cnot: 0.2,
        p_cz: 0.2,
        p_h: 0.2,
        p_t: 0.2,
        p_s: 0.2,
    };

    let progress_bar = ProgressBar::new( population_size as u64);

    progress_bar.set_style(bar_styles::style_build_population());
    progress_bar.set_message(format!("Building population with size {}", population_size));

    for i in 0..population_size {

        let graph = random_circuit_builder.build().to_graph();

        random_population.push(graph);
        progress_bar.inc(1);
        
    }
    
    progress_bar.finish();

    random_population
}