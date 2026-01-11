use std::{ fs::OpenOptions };
use std::io::{ BufWriter, Write };

use super::{ models::{ ExtractStatus, PopulationComponents } };
use colored::Colorize;
use quizx::circuit::CircuitStats;

const TAB: &'static str = "	";

pub fn benchmark<F, R>(f: F) -> (R, f64) where F: FnOnce() -> R {
    let start = std::time::Instant::now();
    let result = f();
    let duration = start.elapsed();
    let millis = duration.as_secs_f64() * 1000.0; // milliseconds
    (result, millis)
}

fn get_highest_fitness(population: &PopulationComponents) -> i64 {
    *population.fitness.iter().max().expect("population has no fitness values")
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
}

impl Logger {
    pub fn new(path: &str) -> Self {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .expect("failed to open log file");
        Logger { file }
    }

    pub fn begin(&mut self) {
        writeln!(self.file, "New log started at {:?}", std::time::SystemTime::now())
            .expect("failed to write log header");
        writeln!(self.file, "Generation\tHighest Fitness")
            .expect("failed to write log header");
    }

    pub fn log(&mut self, population: &PopulationComponents, generation: u32) {
        let highest_fitness = get_highest_fitness(population);
        writeln!(self.file, "{}\t{}", generation, highest_fitness)
            .expect("failed to write log");

        // File writes are immediate (unbuffered)
    }
}