use itertools::sorted_unstable;
use quizx::{
    circuit::{ self, Circuit }, cli, decompose::{BssWithCatsDriver, Decomposer, Driver}, graph::{ BasisElem, GraphLike }, scalar::Scalar4, tensor::{ TensorF, ToTensor }, vec_graph::Graph
};
use rand::{Rng, seq::IndexedRandom};
use std::{collections::HashMap, sync::LazyLock};

use num_complex::Complex;

use super::models::{ PopulationComponents, ExtractStatus };
use super::constants::{ GOAL_CIRCUIT, GOAL_GRAPH, GOAL_TENSOR, GOAL_CIRCUIT_STATS };
use super::simulator;

const NUM_CASES: usize = 8;

static TESTCASES: LazyLock<Vec<(Vec<bool>, String)>> = LazyLock::new(|| {
    let mut testcases = Vec::with_capacity(NUM_CASES);
    let mut rng = rand::rng();
    let num_qubits = GOAL_CIRCUIT.num_qubits();

    let mut decomposer: Decomposer<Graph> = Decomposer::empty();
    decomposer.with_full_simp();
    let driver = BssWithCatsDriver { random_t: false };

    for _ in 0..NUM_CASES {
        // Random input state
        let input_bits: Vec<bool> = (0..num_qubits)
            .map(|_| rng.random_bool(0.5))
            .collect();

        // Sample the circuit mimicking what a call to QPU would do
        let output_str = simulator::sample_with_input(
            &GOAL_CIRCUIT,
            &input_bits,
            &mut decomposer,
            &driver,
            Some(4),
        );

        println!("Generated testcase | Input: {:?}, Sampled Output: {:?}", input_bits, output_str);

        testcases.push((input_bits, output_str));
    }

    testcases
});

pub fn get_fitness_info() -> String {
    format!("NUM CASES: {}\n TESTCASES: {:?}\n GOAL CIRCUIT: {:?}\n GOAL GRAPH: {:?}\n GOAL CIRCUIT STATS: {:?}", NUM_CASES, *TESTCASES, *GOAL_CIRCUIT, *GOAL_GRAPH, *GOAL_CIRCUIT_STATS)
}

/*
static TESTCASES: LazyLock<Vec<(Vec<bool>, Vec<bool>)>> = LazyLock::new(|| {
    let mut testcases = vec![];
    for (idx, &amp) in GOAL_TENSOR.iter().enumerate() {
        if amp.norm_sqr() > 1e-4 {
            let output = (0..GOAL_CIRCUIT.num_qubits())
                .map(|q| (idx >> q) & 1 == 1)
                .collect::<Vec<bool>>();

            println!("created output {} with amp {}", output.iter().map(|b| if *b { "1" } else { "0" }).collect::<String>(), amp);

            testcases.push((vec![false; GOAL_CIRCUIT.num_qubits()], output));
        }
    }
     
    testcases.choose_multiple(&mut rand::rng(), NUM_CASES).cloned().collect()

});*/

/*static GOAL_AMPLITUDES: LazyLock<Vec<f64>> = LazyLock::new(|| {
    let mut decomposer: Decomposer<Graph> = Decomposer::empty();
    decomposer.with_full_simp();
    let driver = BssWithCatsDriver { random_t: false };

    TESTCASES
        .iter()
        .map(|(input, output)| {
            simulator::amplitude_variable(&GOAL_CIRCUIT, &GOAL_GRAPH, &mut decomposer, &driver, input, output)
        })
        .collect()
});*/

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

fn get_approximation_error_fidelity(
    circ_u: &Circuit,
    circ_v: &Circuit,
    parallel: Option<usize>
) -> i64 {

    let mut decomposer: Decomposer<Graph> = Decomposer::empty();
    decomposer.with_full_simp();

    let driver = BssWithCatsDriver { random_t: false };

    let mut graph_u = circ_u.to_graph::<Graph>();
    let graph_v_adjoint = circ_v.to_graph::<Graph>().to_adjoint();
    
    graph_u.plug(&graph_v_adjoint);
    graph_u.plug_inputs(&vec![BasisElem::Z0; circ_u.num_qubits()]);

    let scalar = simulator::decomp_graph(graph_u, &mut decomposer, &driver, parallel);
    let amp = scalar * scalar.conj();
    (amp.complex_value().re * 2147483648.0).round() as i64
}

fn get_approximation_error_testcases(graph: &Graph, circuit: &Circuit) -> i64 {
    //TODO https://github.com/Qiskit/qiskit-rs for simulation
    // maybe https://docs.rs/quantr/latest/quantr/#example
    // https://docs.rs/quizx/latest/quizx/cli/sim/struct.SimArgs.html

    let mut decomposer: Decomposer<Graph> = Decomposer::empty();
    decomposer.with_full_simp();

    let driver = BssWithCatsDriver { random_t: false };

    let mut total_error: f64 = 0.0;
    
    for (input, output) in TESTCASES.iter() {

        let test_result: String = simulator::sample_with_input(circuit, input, &mut decomposer, &driver, Some(4));
        let matched = test_result == *output;
        println!("{} matched goal {} / result {}", matched, output, test_result);

        total_error += if matched {
            0.0
        } else {
            1.0 / NUM_CASES as f64
        };
    }

    // Scale and convert to i64
    (total_error * -25000.0).round() as i64

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

    let approximation_error = get_approximation_error_testcases(graph, circuit);
    let depth = get_depth(graph, circuit);
    let complex_gates = get_complex_gates(graph, circuit);
    let oneq_gates = get_oneq_gates(graph, circuit);
    let input_encodings = get_input_encodings(graph, circuit);

    let fail_penalty = match population.extract_status[i] {
        ExtractStatus::Success => 0,
        ExtractStatus::Fail => -100000,
        ExtractStatus::Panic => -100000,
    };

    //println!("Getting fitness\nstats {:?}\n components DEP {} CMP {} INP {} approxError {}", circuit.stats(),depth, complex_gates, input_encodings, approximation_error);

    return
        approximation_error +
        -10 * depth +
        -3 * oneq_gates +
        -10 * complex_gates +
        -10 * input_encodings +
        fail_penalty;
}

pub fn set_fitness_values(population: &mut PopulationComponents) {
    for i in 0..population.graph.len() {
        population.fitness[i] = get_fitness(population, i);
    }
}
