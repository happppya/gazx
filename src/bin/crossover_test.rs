use std::path;

use quizx::{circuit::Circuit, graph::GraphLike, vec_graph::Graph};
use workspace::{genetic_zx::{crossover, genetic_util, models::{self, ExtractStatus}}, mutation::mutation_runner::MutationType};

fn get_graph(path : &str) -> quizx::vec_graph::Graph {
    let circuit = Circuit::from_file(path).unwrap().to_basic_gates();
    circuit.to_graph_with_options(false, false)
}

fn print_result(population: &mut models::PopulationComponents, graph: Graph, index: usize) {

    population.graph[index] = graph;
    genetic_util::update_extract_status(population, index);
    
    let result_circuit = &population.circuit[index];
    let result_extract_status = &population.extract_status[index];

    println!("Result at index {}: status: {:?} circuit {:?}", index, result_extract_status, result_circuit.stats());
}

fn main() {
    let path = "circuits/small/tof_10.qasm";
    let parent1 = get_graph(path);
    let parent2 = get_graph(path);

    let circuit = Circuit::from_file(path).unwrap().to_basic_gates();

    let pop_size = 10usize;

    let mut population = models::PopulationComponents {
        graph: {
            let mut v = vec![parent1, parent2];
            v.resize(pop_size, Graph::new()); 
            v
        },
        
        circuit: {
            let mut v = vec![circuit.clone(), circuit.clone()];
            v.resize(pop_size, Circuit::new(0));
            v
        },

        // 2. Vectors with no presets: Just use the initialization macro directly
        last_mutation: vec![MutationType::NoMutation; pop_size],
        extract_status: vec![ExtractStatus::Success; pop_size],
        fitness: vec![0.0; pop_size],
        mutation_retries: vec![0; pop_size],
    };

    let result_subgraph_number2 = crossover::crossover_patch_replacement(&population, 0usize, 1usize);
    print_result(&mut population, result_subgraph_number2, 4usize);

    let result_gate_list = crossover::crossover_gate_list(&population, 0usize, 1usize);
    print_result(&mut population, result_gate_list, 2usize);
    

}