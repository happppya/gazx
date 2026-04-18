use std::sync::LazyLock;

use indicatif::ProgressBar;
use quizx::circuit::Circuit;
use quizx::graph::GraphLike;
use quizx::{generate::RandomCircuitBuilder, vec_graph::Graph};

use rand::{prelude::IndexedRandom, rng, rngs::StdRng, SeedableRng};

use colored::Colorize;

use quizx::extract::ToCircuit;
use quizx::simplify::clifford_simp;

use crate::genetic_zx::constants::GOAL_CIRCUIT;
use crate::mutation::mutation_runner;

use super::models::{ExtractStatus, PopulationComponents};
use super::constants::{ GOAL_GRAPH, MUTATION_RETRIES };

pub fn build_population(population_size: u32) -> Vec<Graph> {
    let mut random_population: Vec<Graph> = Vec::new();

    let mut random_circuit_builder = RandomCircuitBuilder {
        rng: StdRng::from_os_rng(),
        qubits: GOAL_CIRCUIT.num_qubits(),
        depth: 4,
        p_cnot: 0.5,
        p_cz: 0.2,
        p_h: 0.2,
        p_t: 0.2,
        p_s: 0.2,
    };

    for _i in 0..population_size {
        //let graph = random_circuit_builder.build().to_graph_with_options(false, true);
        let graph = GOAL_GRAPH.clone();
        random_population.push(graph);
    }

    random_population
}


pub fn update_extract_status(population : &mut PopulationComponents, target_idx : usize) -> bool {

    let extract_result =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Result<_, _> {
                let mut clone = population.graph[target_idx].clone();
                clifford_simp(&mut clone);
                clone.extractor().gflow().up_to_perm().extract()
            }));

    match extract_result {
        Ok(Ok(circuit)) => {
            population.circuit[target_idx] = circuit;
            population.extract_status[target_idx] = ExtractStatus::Success;
            true
        }
        Ok(Err(_)) | Err(_) => {
            population.extract_status[target_idx] = ExtractStatus::Fail;
            false
        }
    }
}

pub fn mutate_and_extract(population: &mut PopulationComponents) {
    let len = population.graph.len();

    for i in 0..len {
        let mutation = *mutation_runner::MUTATIONS_ALL.choose(&mut rng()).unwrap();
        population.last_mutation[i] = mutation;
        population.extract_status[i] = ExtractStatus::Panic;
        population.mutation_retries[i] = 0;
    }

    for _ in 1..=MUTATION_RETRIES {
        for i in 0..len {
            if population.extract_status[i] == ExtractStatus::Success {
                continue;
            }

            population.mutation_retries[i] += 1;

            let mut candidate = population.graph[i].clone();
            let last_mutation = population.last_mutation[i];
            mutation_runner::run_mutation(&mut candidate, &last_mutation);

            // Swap the mutated candidate into the population for testing
            std::mem::swap(&mut population.graph[i], &mut candidate);
            let extract_success = update_extract_status(population, i);

            if !extract_success {
                if population.mutation_retries[i] >= MUTATION_RETRIES {
                    // Don't swap back if out of retries
                    population.extract_status[i] = ExtractStatus::Fail;
                } else {
                    // Revert the swap if failed
                    std::mem::swap(&mut population.graph[i], &mut candidate);
                }
            }
  
        }
    }
}

pub fn mutate_population(population: &mut PopulationComponents) {
    for (i, graph) in population.graph.iter_mut().enumerate() {
        let mutation = mutation_runner::MUTATIONS_ALL.choose(&mut rng()).unwrap();
        population.last_mutation[i] = *mutation;

        mutation_runner::run_mutation(graph, mutation);
    }
}

pub fn extract_population(population: &mut PopulationComponents) {
    let population_size = population.graph.len();

    for (i, graph) in population.graph.iter_mut().enumerate() {
        let extract_result =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Result<_, _> {
                let mut clone = graph.clone();
                clifford_simp(&mut clone);
                clone.extractor().gflow().up_to_perm().extract()
            }));

        match extract_result {
            Ok(Ok(circuit)) => {
                population.extract_status[i] = ExtractStatus::Success;
                population.circuit[i] = circuit;
            }
            Ok(Err(_e)) => {
                population.extract_status[i] = ExtractStatus::Fail;
            }
            Err(_e) => {
                population.extract_status[i] = ExtractStatus::Panic;
                //println!("{} with {:?}", "PANIC".red(), population.last_mutation[i]);
            }
        }

    }

}
