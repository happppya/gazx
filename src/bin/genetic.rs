use quizx::circuit::Circuit;

use workspace::genetic_zx;
use genetic_zx::algorithm;
use genetic_zx::results;
use genetic_zx::models::{ ExtractStatus, PopulationComponents };

use workspace::genetic_zx::algorithm::init_goal_circuit;
use workspace::genetic_zx::models::Hyperparameters;
use workspace::mutation::mutation_runner::MutationType;

use std::io;
use std::io::Write;
use std::thread;

fn pause() {

    let mut input = String::new();
    
    print!("press Enter to continue...");

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

}

/// Runs a single instance of the genetic algorithm
fn run_ga_worker(given_idx: usize, population_size: u32, generations: u32) {

    let mut worker_idx = given_idx;
    if worker_idx == 0 {
        worker_idx = 5;
    }

    let graphs = algorithm::build_population(population_size);

    let mut population = PopulationComponents {
        graph: graphs,
        last_mutation: vec![MutationType::NoMutation; population_size as usize],
        circuit: vec![Circuit::new(0); population_size as usize],
        extract_status: vec![ExtractStatus::Success; population_size as usize],
        fitness: vec![0; population_size as usize],
        mutation_retries: vec![0; population_size as usize],
    };

    let mut parameters = Hyperparameters {
        elitism_rate: 0.1,
        //crossover_rate: 0.1 + (worker_idx as f64 * 0.1), 
        crossover_rate: (1f64 / 32f64) * (worker_idx as f64),
        //crossover_rate: 0.3,
        tournament_size: 3,  
    };

    let logger_path = format!("results_{}", worker_idx);
    let mut logger = results::Logger::new(&logger_path);
    logger.begin(&results::get_fitness_info(), &parameters);

    for generation in 0..generations {

        algorithm::mutate_and_extract(&mut population);

        algorithm::set_fitness_values(&mut population);

        algorithm::repopulate(
            &mut population,
            &mut parameters,
        );

        if generation % 25 == 0 {
            println!("Worker {} - Generation {}", worker_idx, generation);
            results::print_population(&mut population);
        }

        logger.log(&population, generation);

        //pause();
    }
    
    println!("Worker {} finished.", worker_idx);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let num_workers: usize = 1;
    let population_size: u32 = 80;
    let generations: u32 = 125;

    init_goal_circuit("circuits/small/tof_3.qasm");

    let mut handles = vec![];

    for worker_idx in 0..num_workers {
        let handle = thread::spawn(move || {
            run_ga_worker(worker_idx, population_size, generations);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("All workers completed");

    Ok(())
}