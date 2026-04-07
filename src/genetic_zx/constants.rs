use std::sync::{LazyLock, OnceLock};
use quizx::{circuit::{Circuit, CircuitStats}, tensor::{TensorF, ToTensor}, vec_graph::Graph};

pub const MUTATION_RETRIES: u32 = 10;

static INTERNAL_GOAL_CIRCUIT: OnceLock<Circuit> = OnceLock::new();

pub fn init_goal_circuit(path: &str) {
    let circuit = Circuit::from_file(path).unwrap().to_basic_gates();
    
    if INTERNAL_GOAL_CIRCUIT.set(circuit).is_err() {
        panic!("GOAL_CIRCUIT has already been initialized!");
    }

}

fn get_goal_circuit() -> &'static Circuit {
    INTERNAL_GOAL_CIRCUIT.get().expect("goal circuit must be initialized first")
}

pub static GOAL_CIRCUIT: LazyLock<&Circuit> = LazyLock::new(|| {
    get_goal_circuit()
});

pub static GOAL_GRAPH: LazyLock<Graph> = LazyLock::new(|| {
    get_goal_circuit().to_graph_with_options(false, false)
});

pub static GOAL_CIRCUIT_STATS: LazyLock<CircuitStats> = LazyLock::new(|| { 
    get_goal_circuit().stats() 
});