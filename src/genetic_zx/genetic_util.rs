use std::sync::LazyLock;

use indicatif::ProgressBar;
use quizx::circuit::Circuit;
use quizx::{generate::RandomCircuitBuilder, vec_graph::Graph};

use rand::{prelude::IndexedRandom, rng, rngs::StdRng, SeedableRng};

use colored::Colorize;

use quizx::extract::ToCircuit;
use quizx::simplify::clifford_simp;

use crate::mutation::mutation_runner;

use super::bar_styles;
use super::models::{ExtractStatus, GraphPopulation};

static TARGET_GRAPH : LazyLock<Graph> = LazyLock::new(|| {
    Circuit::from_file("circuits/small/grover_5.qasm").unwrap().to_basic_gates().to_graph()
});

pub fn build_population(population_size: u32, num_qubits: usize) -> Vec<Graph> {
    let mut random_population: Vec<Graph> = Vec::new();

    let mut random_circuit_builder = RandomCircuitBuilder {
        rng: StdRng::from_os_rng(),
        qubits: num_qubits,
        depth: 4,
        p_cnot: 0.5,
        p_cz: 0.2,
        p_h: 0.2,
        p_t: 0.2,
        p_s: 0.2,
    };

    let progress_bar = ProgressBar::new(population_size as u64);

    progress_bar.set_style(bar_styles::style_build_population());
    progress_bar.set_message(format!("Building population with size {}", population_size));

    for _i in 0..population_size {
        //let graph = random_circuit_builder.build().to_graph_with_options(false, true);
        let graph = TARGET_GRAPH.clone();
        random_population.push(graph);
        progress_bar.inc(1);
    }

    progress_bar.finish();

    random_population
}

pub fn mutate_population(population: &mut GraphPopulation) {
    for (i, graph) in population.graphs.iter_mut().enumerate() {
        let mutation = mutation_runner::MUTATIONS_ALL.choose(&mut rng()).unwrap();
        population.last_mutations[i] = *mutation;

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
                let mut clone = graph.clone();
                clifford_simp(&mut clone);
                clone.extractor().gflow().up_to_perm().extract()
            }));

        match extract_result {
            Ok(Ok(circuit)) => {
                population.extract_statuses[i] = ExtractStatus::Success;
                population.circuits[i] = circuit;
            }
            Ok(Err(_e)) => {
                population.extract_statuses[i] = ExtractStatus::Fail;
            }
            Err(_e) => {
                population.extract_statuses[i] = ExtractStatus::Panic;
                println!("{} with {:?}", "PANIC".red(), population.last_mutations[i]);
            }
        }

        progress_bar.inc(1);
    }

    progress_bar.finish();
}
