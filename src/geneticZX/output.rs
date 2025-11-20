use super::models::{ExtractStatus, GraphPopulation};
use colored::Colorize;

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

pub fn print_population(population: &mut GraphPopulation) {
    for (i, extract_status) in population.extract_statuses.iter_mut().enumerate() {
        match extract_status {
            ExtractStatus::Success => {
                let circuit = &population.circuits[i];
                println!(
                    "{} Extract success: {:?} at index {} when running mutation {:?}",
                    "[S]".green(),
                    circuit.stats(),
                    i,
                    population.last_mutations[i]
                );
            }
            ExtractStatus::Fail => {
                // Extract error
                println!(
                    "{} Extract fail: at index {} when running mutation {:?}",
                    "[F]".yellow(),
                    i,
                    population.last_mutations[i]
                );
            }
            ExtractStatus::Panic => {
                println!(
                    "{} Panic: at index {} when running mutation {:?}",
                    "[P]".red(),
                    i,
                    population.last_mutations[i]
                );
            }
        }
    }
}
