use quizx::circuit::Circuit;

use workspace::genetic_zx;
use genetic_zx::algorithm;
use genetic_zx::results;
use genetic_zx::models::{ ExtractStatus, PopulationComponents };

use workspace::mutation::mutation_runner::MutationType;

use std::io::{self, Write};

fn pause() {
    let mut input = String::new();

    print!("press Enter to continue...");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
}

//TODO visualizer with https://quantum.cloud.ibm.com/composer

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let population_size: u32 = 100;
    let num_qubits: usize = 4usize;
    let generations: u32 = 10000;

    //let goal_circuit = &Circuit::from_file("circuits/small/grover_5.qasm")?.to_basic_gates();
    let goal_circuit = &Circuit::from_file("circuits/small/tof_5.qasm")?.to_basic_gates();
    println!("starting stats {:?}", goal_circuit.stats());

    /*let (graphs, graph_millis) =
        output::benchmark(|| genetic_util::build_population(population_size, num_qubits));*/

    let graphs = algorithm::build_population(population_size, num_qubits);

    let population: &mut PopulationComponents = &mut (PopulationComponents {
        graph: graphs,
        last_mutation: vec![MutationType::NoMutation; population_size as usize],
        circuit: vec![Circuit::new(0); population_size as usize],
        extract_status: vec![ExtractStatus::Success; population_size as usize],
        fitness: vec![0; population_size as usize],
        mutation_retries: vec![0; population_size as usize],
    });

    //println!("Population build time (ms): {:?}", graph_millis);

    let mut logger= results::Logger::new("genetic_log.txt");
    logger.begin();

    for generation in 0..generations {
        
        println!("Generation {}", generation);

        algorithm::mutate_and_extract(population);

        algorithm::set_fitness_values(population);

        //pause();
        results::print_population(population);

        algorithm::repopulate(
            population, 
            algorithm::worst_individuals_iter(population, (population_size/2) as usize),
        );

        logger.log(population, generation);
        
        //pause();

    }

    Ok(())
}
