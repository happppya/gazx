use quizx::{
    circuit::Circuit
};

use workspace::geneticZX;
use geneticZX::{types, population, output};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let population_size: u32 = 10000;
    let num_qubits: usize = 10usize;

    let goal_circuit = &Circuit::from_file("circuits/small/grover_5.qasm")?.to_basic_gates();

    let (graphs, graph_millis) = output::benchmark(|| {population::build_population(population_size, num_qubits)});
    
    let population: types::GraphPopulation = types::GraphPopulation {
        graphs: graphs,
    };

    let (_, extract_millis) = output::benchmark(|| {output::print_population(population)});

    println!("Population build time (ms): {:?}\nPrinting time (ms): {:?}", graph_millis, extract_millis);

    Ok(())
}
