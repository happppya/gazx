use core::num;
use std::{ arch::x86_64::_popcnt64, panic, time::Instant };

use quizx::{
    circuit::Circuit, extract::ToCircuit, gate::GType::InitAncilla, graph, random_graph::EquatorialStabilizerStateBuilder, simplify::clifford_simp, vec_graph::Graph
};
use rand::{ SeedableRng, rngs::StdRng };
use rand_distr::num_traits::{ PrimInt, ToPrimitive };

#[derive(Debug)]
struct GraphPopulation {
    graphs: Vec<Graph>,
}

fn print_percentage<I: PrimInt + ToPrimitive>(
    message: &'static str,
    progress: I,
    goal: I,
    last_printed: &mut i32
) {
    let progress_f = progress.to_f64().unwrap();
    let goal_f = goal.to_f64().unwrap();

    let percentage_population_build = ((progress_f + 1.0) / goal_f) * 100.0;
    let percentage_int = percentage_population_build.floor() as i32;

    if percentage_int == *last_printed {
        return;
    }

    if percentage_int % 5 == 0 {
        *last_printed = percentage_int;
        println!("{:}: {:.0}%", message, percentage_int);
    }
}

fn print_population(mut population: GraphPopulation) {
    let mut last = -1;

    let goal = population.graphs.len();

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

        print_percentage("Extracting", i, goal, &mut last);
    }
}

fn build_population(population_size: u32, num_qubits: usize) -> Vec<Graph> {
    let mut last = -1;

    let mut random_population: Vec<Graph> = Vec::new();

    for i in 0..population_size {
        let mut graph = EquatorialStabilizerStateBuilder {
            rng: StdRng::from_os_rng(),
            qubits: num_qubits,
        };
        random_population.push(graph.build());

        print_percentage("Building population", i, population_size, &mut last);
    }

    random_population
}

fn benchmark<F, R>(f: F) -> (R, f64)
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    let millis = duration.as_secs_f64() * 1000.0; // milliseconds
    (result, millis)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let population_size: u32 = 100000;
    let num_qubits: usize = 10usize;

    let asdf = &Circuit::from_file("circuits/small/grover_5.qasm")?.to_basic_gates();

    let (graphs, graph_millis) = benchmark(|| {build_population(population_size, num_qubits)});

    let population: GraphPopulation = GraphPopulation {
        graphs: build_population(population_size, num_qubits),
    };

    let (_, extract_millis) = benchmark(|| {print_population(population)});

    println!("Population build time: {:?}\nPrinting time: {:?}", graph_millis, extract_millis);

    Ok(())
}
