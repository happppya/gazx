use quizx::circuit::Circuit;

use geneticZX::{models, output, population_util};
use workspace::mutation::mutation_runner;
use workspace::{
    geneticZX::{self, genetic_main::step_population, models::ExtractStatus},
    mutation::mutation_runner::MutationType,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let population_size: u32 = 50;
    let num_qubits: usize = 10usize;
    let generations: u32 = 10;

    let goal_circuit = &Circuit::from_file("circuits/small/grover_5.qasm")?.to_basic_gates();

    let (graphs, graph_millis) =
        output::benchmark(|| population_util::build_population(population_size, num_qubits));

    let population: &mut models::GraphPopulation = &mut models::GraphPopulation {
        graphs: graphs,
        last_mutations: vec![&MutationType::NoMutation; population_size as usize],

        circuits: vec![Circuit::new(0); population_size as usize],
        extract_statuses: vec![&ExtractStatus::Success; population_size as usize],
    };

    println!("Population build time (ms): {:?}", graph_millis);

    for generation in 0..generations {
        step_population(population);
    }

    Ok(())
}
