use std::sync::LazyLock;

use quizx::{circuit::{Circuit, CircuitStats}, tensor::{TensorF, ToTensor}, vec_graph::Graph};

pub static GOAL_CIRCUIT: LazyLock<Circuit> = LazyLock::new(|| {
    //Circuit::from_file("circuits/small/tof_5.qasm").unwrap().to_basic_gates()
    Circuit::from_file("circuits/small/tof_5.qasm").unwrap().to_basic_gates()
});

pub static GOAL_TENSOR: LazyLock<TensorF> = LazyLock::new(|| { GOAL_CIRCUIT.to_tensorf() });

pub static GOAL_GRAPH: LazyLock<Graph> = LazyLock::new(|| {
    GOAL_CIRCUIT.to_graph_with_options(false, false)
});

pub static GOAL_CIRCUIT_STATS: LazyLock<CircuitStats> = LazyLock::new(|| { GOAL_CIRCUIT.stats() });