use crate::mutation;

use super::{
    fitness,
    models::{ExtractStatus, PopulationComponents},
};
use colored::Colorize;
use quizx::circuit::CircuitStats;

pub fn benchmark<F, R>(f: F) -> (R, f64)
where
    F: FnOnce() -> R,
{
    let start = std::time::Instant::now();
    let result = f();
    let duration = start.elapsed();
    let millis = duration.as_secs_f64() * 1000.0; // milliseconds
    (result, millis)
}

fn print_at_index(population: &PopulationComponents, i: usize) {
    let extract_status = population.extract_status[i];

    match extract_status {
        ExtractStatus::Success => {
            let circuit = &population.circuit[i];
            println!(
                "{} {} Extract Success {:?}",
                "[S]".green(),
                i,
                circuit.stats()
            );
        }
        ExtractStatus::Fail => {
            println!("{} {} Extract Fail", "[F]".yellow(), i);
        }
        ExtractStatus::Panic => {
            println!("{} {} Panic", "[P]".red(), i);
        }
    }

    let mutation = population.last_mutation[i];
    let fitness_value = population.fitness[i];
    let mutation_retries = population.mutation_retries[i];
    
    println!("\tMutation {:?} with {} tries", mutation, mutation_retries);
    println!("\tFitness {:?}", fitness_value);
}

pub fn print_population(population: &mut PopulationComponents) {
    for i in 0..population.graph.len() {
        print_at_index(population, i);
    }
}
