use std::panic;

use itertools::Itertools;
use quizx::circuit::Circuit;
use quizx::decompose::{BssWithCatsDriver, Decomposer, Driver};

use quizx::graph::{BasisElem, GraphLike, VType};
use quizx::scalar::Scalar4;
use quizx::{cli, simplify};
use quizx::vec_graph::Graph;
use rand::{Rng, rng};

use num::rational::Ratio;

#[derive(Clone, Copy, Debug)]
enum Pauli {
    I,
    X,
    Y,
    Z,
}

type PauliString = Vec<Pauli>;

/// Computes the amplitude of an output given an input
pub fn amplitude_variable(
    circuit: &Circuit,
    graph_original : &Graph,
    decomposer: &mut Decomposer<Graph>,
    driver: &impl Driver,
    input: &[bool],
    output: &[bool],
) -> f64 {

    let mut graph = graph_original.clone();

    let qs = circuit.num_qubits();

    if qs == 0 {
        return 0.0; // TODO why is qs 0
    }
    
    assert_eq!(input.len(), qs);
    assert_eq!(output.len(), qs);

    //println!("stats {} ", circuit.stats());
    //println!("length of input and output should be {}, got {}", qs, input.len());

    graph.plug_inputs(
    &input
        .iter()
        .map(|b| if *b { BasisElem::Z1 } else { BasisElem::Z0 })
        .collect::<Vec<_>>(),
    );

    graph.plug_outputs(
        &output
            .iter()
            .map(|x| if *x { BasisElem::Z1 } else { BasisElem::Z0 })
            .collect::<Vec<_>>(),
    );
    
    let simplify_result = panic::catch_unwind(
        panic::AssertUnwindSafe(|| {
            simplify::full_simp(&mut graph);
            decomposer.set_target(graph);
            decomposer.decompose_parallel(driver).scalar()
        })
    );

    //TODO normalize the graph, dividing by the scalar?

    match simplify_result {
        Ok(scalar) => {
            let amp = scalar * scalar.conj();
            return amp.complex_value().re;
        },
        Err(_) => return 0.0,
    }
    
}

/// Run the provided decomposer on a graph.
pub fn decomp_graph(
    g: Graph,
    decomposer: &mut Decomposer<Graph>,
    driver: &impl Driver,
    parallel: Option<usize>
) -> Scalar4 {

    decomposer.set_target(g);
    if let Some(_depth) = parallel {
        decomposer.decompose_parallel(driver).scalar()
    } else {
        decomposer.decompose(driver).scalar()
    }

}

/// Sample with zero inputs
pub fn sample(
    circ: &Circuit,
    decomposer: &mut Decomposer<Graph>,
    driver: &impl Driver,
    parallel: Option<usize>,
) -> String {
    sample_with_input(circ, &vec![false; circ.num_qubits()], decomposer, driver, parallel)
}

/// Sample from a circuit by computing marginals via doubling of the diagram.
pub fn sample_with_input(
    circ: &Circuit,
    input_bits: &[bool], // Added this
    decomposer: &mut Decomposer<Graph>,
    driver: &impl Driver,
    parallel: Option<usize>,
) -> String {
    let qs = circ.num_qubits();
    let mut xs: Vec<bool> = vec![];
    let mut rng = rand::rng();

    let input_basis: Vec<BasisElem> = input_bits
        .iter()
        .map(|&b| if b { BasisElem::Z1 } else { BasisElem::Z0 })
        .collect();

    let mut original_graph: Graph = circ.to_graph();
    original_graph.plug_inputs(&input_basis); 
    
    for _ in 0..qs {
        
        let mut g = original_graph.clone();
        
        for x in &xs {
            g.plug_output(0, if *x { BasisElem::Z1 } else { BasisElem::Z0 });
        }
        g.plug_output(0, BasisElem::Z1);

        g.plug(&g.to_adjoint());
        simplify::full_simp(&mut g);

        let scalar = decomp_graph(g, decomposer, driver, parallel);
        xs.push(rng.random_bool(scalar.complex_value().re.clamp(0.0, 1.0)));
    }
    xs.iter().map(|x| if *x { '1' } else { '0' }).join("")
}

/// Compute an amplitude.
fn amplitude(
    circ: &Circuit,
    decomposer: &mut Decomposer<Graph>,
    driver: &impl Driver,
    bit_str: &Vec<bool>,
    parallel: Option<usize>,
) -> f64 {
    let qs = circ.num_qubits();
    let bit_str = match bit_str.as_slice() {
        [b] => &vec![*b; qs],
        bs if bs.len() == qs => bit_str,
        _ => {
            panic!("Bit string length does not match number of qubits");
        }
    };

    let mut g: Graph = circ.to_graph();
    g.plug_inputs(&vec![BasisElem::Z0; qs]);
    g.plug_outputs(
        &bit_str
            .iter()
            .map(|x| if *x { BasisElem::Z1 } else { BasisElem::Z0 })
            .collect_vec(),
    );

    let scalar = decomp_graph(g, decomposer, driver, parallel);
    let amp = scalar * scalar.conj();
    amp.complex_value().re
}

/// Computes an expectation value by doubling the diagram.
fn expectation_value(
    circ: &Circuit,
    decomposer: &mut Decomposer<Graph>,
    driver: &impl Driver,
    pauli_str: &PauliString,
    parallel: Option<usize>,
) -> f64 {
    let qs = circ.num_qubits();
    let pauli_str = match pauli_str.as_slice() {
        [p] => &vec![*p; qs],
        ps if ps.len() == qs => pauli_str,
        _ => {
            panic!("Pauli string length does not match number of qubits");
        }
    };

    let mut g: Graph = circ.to_graph();
    g.plug_inputs(&vec![BasisElem::Z0; qs]);
    let g_adj = g.to_adjoint();
    for (i, p) in pauli_str.iter().enumerate() {
        let b = g.outputs()[i];
        let [(v, _)] = g.incident_edge_vec(b).try_into().unwrap();
        match p {
            Pauli::I => {}
            Pauli::X => {
                let x = g.add_vertex_with_phase(VType::X, 1);
                g.remove_edge(v, b);
                g.add_edge(v, x);
                g.add_edge(x, b);
            }
            Pauli::Y => {
                let x = g.add_vertex_with_phase(VType::X, 1);
                let z = g.add_vertex_with_phase(VType::Z, 1);
                g.remove_edge(v, b);
                g.add_edge(v, z);
                g.add_edge(z, x);
                g.add_edge(x, b);
                g.scalar_mut().mul_phase(Ratio::new(1, 2));
            }
            Pauli::Z => {
                let z = g.add_vertex_with_phase(VType::Z, 1);
                g.remove_edge(v, b);
                g.add_edge(v, z);
                g.add_edge(z, b);
            }
        }
    }
    g.plug(&g_adj);

    let scalar = decomp_graph(g, decomposer, driver, parallel);
    scalar.complex_value().re
}