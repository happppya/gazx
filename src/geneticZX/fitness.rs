use quizx::circuit::{Circuit, CircuitStats};
use std::sync::LazyLock;

static GOAL_CIRCUIT: LazyLock<Circuit> =
    LazyLock::new(|| Circuit::from_file("circuits/small/grover_5.qasm")?.to_basic_gates());

static GOAL_STATS: LazyLock<CircuitStats> = LazyLock::new(|| GOAL_CIRCUIT.stats());

fn get_component_approximation_error(circuit: Circuit) {}

fn get_component_depth(circuit: Circuit) -> i64 {
    let stats = circuit.stats();
}

fn get_component_complex_gates(circuit: Circuit) -> i64 {
    let stats = circuit.stats();
    let two_qubit_diff = GOAL_STATS.twoq - stats.twoq;

    return two_qubit_diff as i64;
}

fn get_component_input_encodings() {}
