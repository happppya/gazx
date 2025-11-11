use indicatif::ProgressBar;
use quizx::extract::ToCircuit;
use quizx::simplify::clifford_simp;

use super::models::GraphPopulation;
use super::bar_styles;

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

    let population_size = population.graphs.len();
    
    let progress_bar = ProgressBar::new( population_size as u64);

    progress_bar.set_style(bar_styles::style_print_population());
    progress_bar.set_message("Extracting circuits");

    for (i, graph) in population.graphs.iter_mut().enumerate() {
        let extract_result = std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(
                || -> Result<_, _> {
                    clifford_simp(graph);
                    graph.extractor().gflow().up_to_perm().extract()
                }
            )
        );

        match extract_result {
            Ok(Ok(circuit)) => {
                println!("Extract success: {:?} at index {}", circuit.stats(), i);
            }
            Ok(Err(_e)) => {
                // Extract error
            }
            Err(_) => {
                // Panic occurred
            }
        }
        
    }

    progress_bar.finish();
}