use std::fs::File;
use std::time::SystemTime;
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
    fitness_file: File,
    circuits_file: File,
    pub overall_highest_fitness: i64,
}

impl Logger {
    
    pub fn new(base_path: &str) -> Self {
        let fitness_path = format!("{}_fitness_table.txt", base_path);
        let circuits_path = format!("{}_best_circuits.txt", base_path);

        let open_file = |path: &str| -> File {
            OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(path)
                .unwrap_or_else(|_| panic!("failed to open log file at {}", path))
        };

        Logger { 
            fitness_file: open_file(&fitness_path),
            circuits_file: open_file(&circuits_path),
            overall_highest_fitness: i64::MIN,
        }
    }

    pub fn begin(&mut self, run_info: &String) {
        let now = SystemTime::now();
        
        // Setup fitness table
        writeln!(self.fitness_file, "# Log started at {:?}", now)
            .expect("failed to write fitness header");
        writeln!(self.fitness_file, "Generation\tHighest_Fitness")
            .expect("failed to write fitness columns");

        // Setup circuits log
        writeln!(self.fitness_file, "# Log started at {:?}", now)
            .expect("failed to write circuits header");

        writeln!(self.circuits_file, "RUN INFO:\n{}", run_info).expect("failed to write run info");
        
        writeln!(self.circuits_file, "BEST CIRCUITS LOG - Started at {:?}", now)
            .expect("failed to write circuits header");
        writeln!(self.circuits_file, "==================================================")
            .expect("failed to write circuits header");
    }

    pub fn log(&mut self, population: &PopulationComponents, generation: u32) {

        let most_fit_individual = get_highest_fitness_index(population);
        let highest_fitness = population.fitness[most_fit_individual];
        
        // Write to the table every generation
        writeln!(self.fitness_file, "{}\t{}", generation, highest_fitness)
            .expect("failed to write fitness log");
        
        // Write to best circuits only if a new best is found
        if highest_fitness > self.overall_highest_fitness {
            self.overall_highest_fitness = highest_fitness;

            let qasm = population.circuit[most_fit_individual].to_qasm();
            let stats = population.circuit[most_fit_individual].stats();

            writeln!(
                self.circuits_file,
                "\n### New best found at generation {} ###\n\
                Fitness Value: {}\n\
                Stats: {}\n\
                QASM:\n{}\n\
                --------------------------------------------------",
                generation, highest_fitness, stats, qasm
            ).expect("failed to write best circuit log");
        }
    }
}