use std::{ fs::OpenOptions };
use std::io::{ Write };

use super::{ models::{ ExtractStatus, PopulationComponents } };
use colored::Colorize;

const TAB: &'static str = "	";

pub fn benchmark<F, R>(f: F) -> (R, f64) where F: FnOnce() -> R {
    let start = std::time::Instant::now();
    let result = f();
    let duration = start.elapsed();
    let millis = duration.as_secs_f64() * 1000.0; // milliseconds
    (result, millis)
}

fn get_highest_fitness_index(population: &PopulationComponents) -> usize {
    population
        .fitness
        .iter()
        .enumerate()
        .max_by_key(|(_, &fitness)| fitness)
        .map(|(idx, _)| idx)
        .expect("population has no fitness values")
}

fn print_at_index(population: &PopulationComponents, i: usize) {
    let extract_status = population.extract_status[i];

    match extract_status {
        ExtractStatus::Success => {
            let circuit = &population.circuit[i];
            println!("{} {} Extract Success {:?}", "[S]".green(), i, circuit.stats());
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

pub struct Logger {
    file: std::fs::File,
    overall_highest_fitness : i64,
}

impl Logger {
    pub fn new(path: &str) -> Self {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .expect("failed to open log file");
        Logger { file: file, overall_highest_fitness: i64::MIN }
    }

    pub fn begin(&mut self) {
        writeln!(self.file, "New log started at {:?}", std::time::SystemTime::now())
            .expect("failed to write log header");
        writeln!(self.file, "Generation\tHighest Fitness")
            .expect("failed to write log header");
    }

    pub fn log(&mut self, population: &PopulationComponents, generation: u32) {

        let most_fit_individual = get_highest_fitness_index(population);
        let highest_fitness = population.fitness[most_fit_individual];
        writeln!(self.file, "{}\t{}", generation, highest_fitness)
            .expect("failed to write log");
        
        if highest_fitness > self.overall_highest_fitness {
            self.overall_highest_fitness = highest_fitness;

            let qasm = population.circuit[most_fit_individual].to_qasm();
            let stats = population.circuit[most_fit_individual].stats();

            writeln!(self.file, "New best circuit with stats: {}\n", stats)
                .expect("failed to write log");
            writeln!(self.file, "And has qasm:\n{}", qasm)
                .expect("failed to write qasm");
        }
    }
}