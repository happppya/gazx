use clap::Parser;
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
use std::panic;
use std::thread;

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = 0.4723)]
    elitism_rate: f64,

    #[arg(long, default_value_t = 0.1495)]
    crossover_rate: f64,

    #[arg(long, default_value_t = 7)]
    tournament_size: usize,

    #[arg(long, default_value_t = 150)]
    population_size: u32,

    #[arg(long, default_value_t = 500)]
    generations: u32,

    // Supresses all output except the final best score, which is read by Python side
    #[arg(long, default_value_t = false)]
    quiet: bool,
}

fn pause() {
    let mut input = String::new();

    print!("press Enter to continue...");

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
}

/// Runs a single instance of the genetic algorithm
fn run_ga_worker(
    given_idx: usize,
    population_size: u32,
    generations: u32,
    elitism_rate: f64,
    crossover_rate: f64,
    tournament_size: usize,
    quiet: bool
) -> f64 {
    let worker_idx = given_idx;
    let graphs = algorithm::build_population(population_size);

    let mut population = PopulationComponents {
        graph: graphs,
        last_mutation: vec![MutationType::NoMutation; population_size as usize],
        circuit: vec![Circuit::new(0); population_size as usize],
        extract_status: vec![ExtractStatus::Success; population_size as usize],
        fitness: vec![0.0; population_size as usize],
        mutation_retries: vec![0; population_size as usize],
    };

    let mut parameters = Hyperparameters {
        elitism_rate,
        crossover_rate,
        tournament_size,
    };

    let mut logger_option: Option<results::Logger> = None;

    if !quiet {
        let logger_path = format!("results_{}", worker_idx);
        let mut logger = results::Logger::new(&logger_path);
        logger.begin(&results::get_fitness_info(), &parameters);
        logger_option = Some(logger);
    }

    for generation in 0..generations {
        algorithm::mutate_and_extract(&mut population);
        algorithm::set_fitness_values(&mut population);
        algorithm::repopulate(&mut population, &mut parameters);

        if !quiet && generation % 10 == 0 {
            println!("Worker {} - Generation {}", worker_idx, generation);
            results::print_population(&mut population);
        }
        
        if let Some(logger) = &mut logger_option {
            logger.log(&population, generation);
        }
    }

    if !quiet {
        println!("Worker {} finished.", worker_idx);
    }

    // Return best fitness
    population.fitness.into_iter().fold(f64::MIN, f64::max)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Args::parse();

    if args.quiet {
        // ignore panic messages
        panic::set_hook(Box::new(|_| {}));
    }

    init_goal_circuit("circuits/small/mod5_4.qasm");

    let num_workers: usize = 1;
    let best_overall_score : f64;
    
    if num_workers == 1 {
        best_overall_score = run_ga_worker(
            0,
            args.population_size,
            args.generations,
            args.elitism_rate,
            args.crossover_rate,
            args.tournament_size,
            args.quiet
        );
    } else {
        let mut handles = vec![];

        for worker_idx in 0..num_workers {
            let pop_size = args.population_size;
            let gens = args.generations;
            let e_rate = args.elitism_rate;
            let c_rate = args.crossover_rate;
            let t_size = args.tournament_size;
            let quiet = args.quiet;

            let handle = thread::spawn(move || {
                run_ga_worker(worker_idx, pop_size, gens, e_rate, c_rate, t_size, quiet)
            });

            handles.push(handle);
        }

        let mut final_scores = vec![];
        for handle in handles {
            let score = handle.join().unwrap();
            final_scores.push(score);
        }

        // Absolute best score from workers
        best_overall_score = final_scores.into_iter().fold(f64::MIN, f64::max);
    }

    // Only print the number and Python side will read it
    if args.quiet {
        println!("{}", best_overall_score);
    } else {
        println!("All workers completed. Best Score: {}", best_overall_score);
    }

    Ok(())
}
