use quizx::{
    circuit::Circuit
};

use workspace::geneticZX::{self, step::step_population};
use geneticZX::{models, population, output};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let population_size: u32 = 100;
    let num_qubits: usize = 10usize;
    let generations : u32 = 10;

    let goal_circuit = &Circuit::from_file("circuits/small/grover_5.qasm")?.to_basic_gates();

    let (graphs, graph_millis) = output::benchmark(|| {population::build_population(population_size, num_qubits)});
    
    let population: &mut models::GraphPopulation = &mut models::GraphPopulation {
        graphs: graphs,
    };

    println!("Population build time (ms): {:?}", graph_millis);

    for generation in 0..generations {
        step_population(population);
    }

    Ok(())
}
