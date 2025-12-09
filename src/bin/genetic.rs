use quizx::circuit::Circuit;

use workspace::genetic_zx;
use genetic_zx::algorithm;
use genetic_zx::models::{ ExtractStatus, GraphPopulation };
use workspace::mutation::mutation_runner::MutationType;

use std::io::{self, Write};

fn pause() {
    let mut input = String::new();

    print!("press Enter to continue...");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let population_size: u32 = 100;
    let num_qubits: usize = 10usize;
    let generations: u32 = 100;

    let goal_circuit = &Circuit::from_file("circuits/small/grover_5.qasm")?.to_basic_gates();

    println!("starting stats {:?}", goal_circuit.stats());

    /*let (graphs, graph_millis) =
        output::benchmark(|| genetic_util::build_population(population_size, num_qubits));*/

    let graphs = algorithm::build_population(population_size, num_qubits);

    let population: &mut GraphPopulation = &mut (GraphPopulation {
        graphs: graphs,
        last_mutations: vec![MutationType::NoMutation; population_size as usize],

        circuits: vec![Circuit::new(0); population_size as usize],
        extract_statuses: vec![ExtractStatus::Success; population_size as usize],
        fitness_values: vec![0; population_size as usize],
    });

    //println!("Population build time (ms): {:?}", graph_millis);

    for generation in 0..generations {
        algorithm::mutate_population(population);

        let (_, extract_millis) = algorithm::benchmark(|| {
            algorithm::extract_population(population);
        });

        algorithm::set_fitness_values(population);
        algorithm::print_population(population);

        println!("Population extraction time: {:?}", extract_millis);

        algorithm::repopulate(
            population, 
            algorithm::worst_individuals_iter(population, (population_size/2) as usize),
        );

        pause();

    }

    Ok(())
}
