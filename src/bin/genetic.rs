use quizx::circuit::Circuit;

use workspace::genetic_zx;
use genetic_zx::algorithm;
use genetic_zx::results;
use genetic_zx::models::{ ExtractStatus, PopulationComponents };

use workspace::genetic_zx::algorithm::init_goal_circuit;
use workspace::genetic_zx::models::Hyperparameters;
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
    let population_size: u32 = 80;
    let generations: u32 = 10000;

    init_goal_circuit("circuits/small/tof_3.qasm");

    /*let (graphs, graph_millis) =
        output::benchmark(|| genetic_util::build_population(population_size, num_qubits));*/

    let graphs = algorithm::build_population(population_size);

    let population: &mut PopulationComponents = &mut (PopulationComponents {
        graph: graphs,
        last_mutation: vec![MutationType::NoMutation; population_size as usize],
        circuit: vec![Circuit::new(0); population_size as usize],
        extract_status: vec![ExtractStatus::Success; population_size as usize],
        fitness: vec![0; population_size as usize],
        mutation_retries: vec![0; population_size as usize],
    });

    let parameters: &mut Hyperparameters = &mut Hyperparameters {
        elitism_rate: 0.1,
        crossover_rate: 0.5,
        tournament_size: 3,
    };

    //println!("Population build time (ms): {:?}", graph_millis);

    let mut logger= results::Logger::new("results");
    logger.begin(&results::get_fitness_info());

    for generation in 0..generations {
        
        println!("Generation {}", generation);

        algorithm::mutate_and_extract(population);

        algorithm::set_fitness_values(population);

        //pause();

        results::print_population(population);

        algorithm::repopulate(
            population,
            parameters,
        );

        //pause();

        logger.log(population, generation);
        
        //pause();

    }

    Ok(())
}
