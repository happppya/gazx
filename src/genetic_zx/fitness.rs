
use quizx::{
    circuit::{ Circuit }, decompose::{BssWithCatsDriver, Decomposer}, graph::{ GraphLike }, vec_graph::Graph
};
use rand::{Rng};
use std::sync::LazyLock;

use super::models::{ PopulationComponents, ExtractStatus };
use super::constants::{ GOAL_CIRCUIT, GOAL_GRAPH, GOAL_CIRCUIT_STATS };
use super::simulator;

const NUM_CASES: usize = 32;

const CUSTOM_CASES: bool = false;

const CUSTOM_CASES_STR: &str = r#"
[([true, false, false, false, true], "10001"),
 ([true, true, false, true, false], "11010"),
 ([false, false, true, true, true], "00101"),
 ([true, true, false, false, true], "11001"),
 ([false, true, true, true, false], "01110"),
 ([false, false, false, false, false], "00000"),
 ([true, true, false, true, false], "11010"),
 ([false, true, false, false, true], "01001"),
 ([true, true, false, true, true], "11011"),
 ([true, false, true, false, false], "10100")]
"#;

static TESTCASES: LazyLock<Vec<(Vec<bool>, String)>> = LazyLock::new(|| {
    if CUSTOM_CASES {
        return parse_custom_cases(CUSTOM_CASES_STR);
    }

    let mut testcases = Vec::with_capacity(NUM_CASES);
    let mut rng = rand::rng();
    let num_qubits = GOAL_CIRCUIT.num_qubits();

    let mut decomposer: Decomposer<Graph> = Decomposer::empty();
    decomposer.with_full_simp();
    let driver = BssWithCatsDriver { random_t: false };

    for _ in 0..NUM_CASES {
        let input_bits: Vec<bool> = (0..num_qubits)
            .map(|_| rng.random_bool(0.5))
            .collect();

        let output_str = simulator::sample_with_input(
            &GOAL_CIRCUIT,
            &input_bits,
            &mut decomposer,
            &driver,
            Some(4),
        );

        testcases.push((input_bits, output_str));
    }

    testcases
});

fn parse_custom_cases(input: &str) -> Vec<(Vec<bool>, String)> {
    let trimmed = input.trim()
        .trim_start_matches('[')
        .trim_end_matches(']');

    let mut cases = Vec::new();

    for entry in trimmed.split("),") {
        let e = entry.trim()
            .trim_start_matches('(')
            .trim_end_matches(')');

        let mut parts = e.splitn(2, "],");

        // Parse boolean vector
        let bool_part = parts.next().unwrap()
            .trim()
            .trim_start_matches('[');

        let bools = bool_part
            .split(',')
            .map(|b| match b.trim() {
                "true" => true,
                "false" => false,
                _ => panic!("Invalid bool"),
            })
            .collect::<Vec<bool>>();

        // Parse output string
        let str_part = parts.next().unwrap()
            .trim()
            .trim_matches('"');

        cases.push((bools, str_part.to_string()));
    }

    cases
}

pub fn get_fitness_info() -> String {
    format!("NUM CASES: {}\n TESTCASES: {:?}\n GOAL CIRCUIT: {:?}\n GOAL GRAPH: {:?}\n GOAL CIRCUIT STATS: {:?}", NUM_CASES, *TESTCASES, *GOAL_CIRCUIT, *GOAL_GRAPH, *GOAL_CIRCUIT_STATS)
}

fn get_approximation_error_testcases(graph: &Graph, circuit: &Circuit) -> f64 {
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

        total_error += if matched {
            0.0
        } else {
            1.0 / NUM_CASES as f64
        };
    }

    if total_error > 0.45 {
        return 1.0;
    }

    total_error = f64::powf(total_error, 0.5);
    total_error

}

fn normalize_diff(actual: i64, target: i64) -> f64 {
    if target == 0 {
        return actual as f64;
    }
    (actual - target) as f64 / target.abs() as f64
}

fn get_depth(graph: &Graph, _circuit: &Circuit) -> f64 {
    normalize_diff(graph.depth() as i64, GOAL_GRAPH.depth() as i64)
}

fn get_oneq_gates(_graph: &Graph, circuit: &Circuit) -> f64 {
    let stats = circuit.stats();
    normalize_diff(stats.oneq as i64, GOAL_CIRCUIT_STATS.oneq as i64)
}

fn get_complex_gates(_graph: &Graph, circuit: &Circuit) -> f64 {
    let stats = circuit.stats();
    normalize_diff(stats.twoq as i64, GOAL_CIRCUIT_STATS.twoq as i64)
}

fn get_input_encodings(graph: &Graph, _circuit: &Circuit) -> f64 {
    // TODO the inputs dont really do anything, always 5
    normalize_diff(graph.inputs().len() as i64, GOAL_GRAPH.inputs().len() as i64)
}

pub fn get_fitness(population: &PopulationComponents, i: usize) -> f64 {

    let graph = &population.graph[i];
    let circuit = &population.circuit[i];

    let depth = get_depth(graph, circuit);
    let input_encodings = get_input_encodings(graph, circuit);

    let graph_fitness = 
        -1000.0 * depth + 
        -1000.0 * input_encodings;

    let circuit_fitness = match population.extract_status[i] {
        ExtractStatus::Success => {
            let approximation_error = get_approximation_error_testcases(graph, circuit);
            let complex_gates = get_complex_gates(graph, circuit);
            let oneq_gates = get_oneq_gates(graph, circuit);

            -10000.0 * approximation_error +
            -600.0 * oneq_gates +
            -1000.0 * complex_gates
        },
        ExtractStatus::Fail | ExtractStatus::Panic => {
            -15000.0
        }
    };

    let mut fitness = circuit_fitness + graph_fitness;
    if circuit.num_gates() == 0 {
        fitness -= 10000.0;
    }

    //println!("Getting fitness\nstats {:?}\n components DEP {} CMP {} INP {} approxError {}", circuit.stats(),depth, complex_gates, input_encodings, approximation_error);

    return fitness;

}

pub fn set_fitness_values(population: &mut PopulationComponents) {
    for i in 0..population.graph.len() {
        population.fitness[i] = get_fitness(population, i);
    }
}
