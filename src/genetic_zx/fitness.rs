use quizx::{
    circuit::{ Circuit, CircuitStats }, cli, decompose::{BssWithCatsDriver, Decomposer, Driver}, fscalar::FScalar, graph::{ self, BasisElem, GraphLike }, scalar::Scalar4, simplify, tensor::{ TensorF, ToTensor }, vec_graph::Graph
};
use std::sync::LazyLock;

use num_complex::Complex;

use super::models::{ PopulationComponents, ExtractStatus };

static GOAL_CIRCUIT: LazyLock<Circuit> = LazyLock::new(|| {
    Circuit::from_file("circuits/small/grover_5.qasm").unwrap().to_basic_gates()
});

static GOAL_TENSOR: LazyLock<TensorF> = LazyLock::new(|| { GOAL_CIRCUIT.to_tensorf() });

static GOAL_GRAPH: LazyLock<Graph> = LazyLock::new(|| {
    GOAL_CIRCUIT.to_graph_with_options(false, false)
});

static GOAL_CIRCUIT_STATS: LazyLock<CircuitStats> = LazyLock::new(|| { GOAL_CIRCUIT.stats() });

fn get_approximation_error_tensor(graph: &Graph, circuit: &Circuit) -> i64 {
    let prediction: TensorF = circuit.to_tensorf();
    let target: TensorF = GOAL_CIRCUIT.to_tensorf();

    // inner product = sum over i of conj(prediction_i) * target_i
    let inner: Complex<f64> = prediction
        .iter()
        .zip(target.iter())
        .map(|(a, b)| a.conj() * b)
        .sum();

    // fidelity = |<prediction|target>|^2 = real^2 + imag^2
    // in [0,1]
    let fidelity = inner.norm_sqr();

    // scale and convert to int
    let scale = f64::powf(2.0, 32.0);
    let approximation_error = ((1.0 - fidelity) * scale).round() as i64;

    approximation_error
}

/// Run the provided decomposer on a graph.
fn decomp_graph(
    mut g: Graph,
    decomposer: &mut Decomposer<Graph>,
    driver: &impl Driver,
    parallel: Option<usize>
) -> Scalar4 {
    simplify::full_simp(&mut g);
    decomposer.set_target(g);
    if let Some(_depth) = parallel {
        decomposer.decompose_parallel(driver).scalar()
    } else {
        decomposer.decompose(driver).scalar()
    }
}

fn get_approximation_error_fidelity(
    circ_u: &Circuit,
    circ_v: &Circuit,
    parallel: Option<usize>
) -> f64 {

    let mut decomposer: Decomposer<Graph> = Decomposer::empty();
    decomposer.with_full_simp();

    let driver = BssWithCatsDriver { random_t: false };

    let mut graph_u = circ_u.to_graph::<Graph>();
    let graph_v_adjoint = circ_v.to_graph::<Graph>().to_adjoint();
    
    graph_u.plug(&graph_v_adjoint);
    graph_u.plug_inputs(&vec![BasisElem::Z0; circ_u.num_qubits()]);

    let scalar = decomp_graph(graph_u, &mut decomposer, &driver, parallel);
    let amp = scalar * scalar.conj();
    amp.complex_value().re
}

fn get_approximation_error_testcases(graph: &Graph, circuit: &Circuit) -> i64 {
    //TODO https://github.com/Qiskit/qiskit-rs for simulation
    // maybe https://docs.rs/quantr/latest/quantr/#example
    // https://docs.rs/quizx/latest/quizx/cli/sim/struct.SimArgs.html
    unimplemented!();
}

fn get_depth(graph: &Graph, _circuit: &Circuit) -> i64 {
    (graph.depth() - GOAL_GRAPH.depth()) as i64
}

fn get_oneq_gates(_graph: &Graph, circuit: &Circuit) -> i64 {
    let stats = circuit.stats();
    (stats.oneq as i64) - (GOAL_CIRCUIT_STATS.oneq as i64)
}

fn get_complex_gates(_graph: &Graph, circuit: &Circuit) -> i64 {
    let stats = circuit.stats();
    (stats.twoq as i64) - (GOAL_CIRCUIT_STATS.twoq as i64)
}

fn get_input_encodings(graph: &Graph, _circuit: &Circuit) -> i64 {
    (graph.inputs().len() as i64) - (GOAL_GRAPH.inputs().len() as i64)
}

fn get_fitness(population: &PopulationComponents, i: usize) -> i64 {
    let graph = &population.graph[i];
    let circuit = &population.circuit[i];

    let approximation_error = 1; //get_approximation_error_tensor(graph, circuit);
    let depth = get_depth(graph, circuit);
    let complex_gates = get_complex_gates(graph, circuit);
    let oneq_gates = get_oneq_gates(graph, circuit);
    let input_encodings = get_input_encodings(graph, circuit);

    let fail_penalty = match population.extract_status[i] {
        ExtractStatus::Success => 0,
        ExtractStatus::Fail => -100000,
        ExtractStatus::Panic => -100000,
    };

    //println!("Getting fitness\nstats {:?}\n components {} {} {} {}", circuit.stats(),
    //   approximation_error, depth, complex_gates, input_encodings);

    return (
        approximation_error / 1000 -
        10 * depth +
        -3 * oneq_gates +
        -10 * complex_gates +
        -10 * input_encodings +
        fail_penalty
    );
}

pub fn set_fitness_values(population: &mut PopulationComponents) {
    for i in 0..population.graph.len() {
        population.fitness[i] = get_fitness(population, i);
    }
}
