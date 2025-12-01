use quizx::{circuit::{Circuit, CircuitStats}, graph::GraphLike, vec_graph::Graph};
use std::sync::LazyLock;

static GOAL_CIRCUIT : LazyLock<Circuit> = LazyLock::new(|| {
    Circuit::from_file("circuits/small/grover_5.qasm").unwrap().to_basic_gates()
});

static GOAL_GRAPH : LazyLock<Graph> = LazyLock::new(|| {
    GOAL_CIRCUIT.to_graph_with_options(true, true)
});

static GOAL_CIRCUIT_STATS : LazyLock<CircuitStats> = LazyLock::new(|| {
    GOAL_CIRCUIT.stats()
});

fn get_approximation_error(graph : &Graph, circuit : &Circuit) -> i64 {
    0
}

fn get_depth(graph : &Graph, _circuit : &Circuit) -> i64 {
    (graph.depth() - GOAL_GRAPH.depth()) as i64
}

fn get_complex_gates(_graph : &Graph, circuit : &Circuit) -> i64 {
    let stats = circuit.stats();
    let two_qubit_diff = stats.twoq - GOAL_CIRCUIT_STATS.twoq;
    return two_qubit_diff as i64
}

fn get_input_encodings(graph : &Graph, _circuit : &Circuit) -> i64 {
    (graph.inputs().len() - GOAL_GRAPH.inputs().len()) as i64
}

pub fn get_fitness(graph : Graph, circuit : Circuit) -> i64 {

    let approximation_error = get_approximation_error(&graph, &circuit);
    let depth = get_depth(&graph, &circuit);
    let complex_gates = get_complex_gates(&graph, &circuit);
    let input_encodings = get_input_encodings(&graph, &circuit);

    return 
        0 * approximation_error + // unimplemented
        -10 * depth +
        -10 * complex_gates + 
        -10 * input_encodings;
}
