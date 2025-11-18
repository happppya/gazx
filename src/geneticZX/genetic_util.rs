use indicatif::{ProgressBar, ProgressState};
use quizx::{
    generate::RandomCircuitBuilder, random_graph::EquatorialStabilizerStateBuilder,
    vec_graph::Graph,
};

use rand::{prelude::IndexedRandom, rng, rngs::StdRng, SeedableRng};

use colored::Colorize;

use quizx::extract::ToCircuit;
use quizx::simplify::clifford_simp;

use crate::mutation::mutation_runner;

use super::bar_styles;
use super::models::{ExtractStatus, GraphPopulation};

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

    let progress_bar = ProgressBar::new(population_size as u64);

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

pub fn mutate_population(population: &mut GraphPopulation) {
    for (i, graph) in population.graphs.iter_mut().enumerate() {
        let mutation = mutation_runner::MUTATIONS_ALL.choose(&mut rng()).unwrap();
        population.last_mutations[i] = mutation;

        mutation_runner::run_mutation(graph, mutation);
    }
}

pub fn extract_population(population: &mut GraphPopulation) {
    let population_size = population.graphs.len();
    let progress_bar = ProgressBar::new(population_size as u64);

    progress_bar.set_style(bar_styles::style_build_population());
    progress_bar.set_message(format!(
        "Extracting population with size {}",
        population_size
    ));

    for (i, graph) in population.graphs.iter_mut().enumerate() {
        let extract_result =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Result<_, _> {
                clifford_simp(graph);
                graph.extractor().gflow().up_to_perm().extract()
            }));

        match extract_result {
            Ok(Ok(circuit)) => {
                population.extract_statuses[i] = &ExtractStatus::Success;
                population.circuits[i] = circuit;
            }
            Ok(Err(e)) => {
                population.extract_statuses[i] = &ExtractStatus::Fail;
            }
            Err(e) => {
                population.extract_statuses[i] = &ExtractStatus::Panic;
                println!("{} with {:?}", "PANIC".red(), population.last_mutations[i]);
            }
        }

        progress_bar.inc(1);
    }

    progress_bar.finish();
}
