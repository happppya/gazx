use quizx::{
    circuit::{Circuit, CircuitStats},
    graph::GraphLike,
    vec_graph::Graph,
};
use std::sync::LazyLock;

static GOAL_CIRCUIT: LazyLock<Circuit> = LazyLock::new(|| {
    Circuit::from_file("circuits/small/grover_5.qasm")
        .unwrap()
        .to_basic_gates()
});

static GOAL_CIRCUIT_STATS: LazyLock<CircuitStats> = LazyLock::new(|| GOAL_CIRCUIT.stats());

fn get_component_approximation_error(graph: &Graph, circuit: &Circuit) -> i64 {
    0
}

fn get_component_depth(graph: &Graph, circuit: &Circuit) -> i64 {
    0
}

fn get_component_complex_gates(graph: &Graph, circuit: &Circuit) -> i64 {
    let stats = circuit.stats();
    let two_qubit_diff = stats.twoq - GOAL_CIRCUIT_STATS.twoq;
    return two_qubit_diff as i64;
}

fn get_component_input_encodings(graph: &Graph, circuit: &Circuit) -> i64 {
    -1 * graph.inputs().len() as i64
}

pub fn get_fitness(graph: Graph, circuit: Circuit) -> i64 {
    let component_approximation_error = get_component_approximation_error(&graph, &circuit);
    let component_depth = get_component_depth(&graph, &circuit);
    let component_complex_gates = get_component_complex_gates(&graph, &circuit);
    let component_input_encodings = get_component_input_encodings(&graph, &circuit);

    return 1 * component_approximation_error
        + 1 * component_depth
        + 1 * component_complex_gates
        + 1 * component_input_encodings;
}
