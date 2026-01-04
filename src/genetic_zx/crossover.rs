use std::collections::VecDeque;

use quizx::{ circuit::Circuit, extract::ToCircuit, simplify::clifford_simp, vec_graph::Graph };

fn extract(graph: &mut Graph) -> Option<Circuit> {
    let extract_result = std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(
            || -> Result<_, _> {
                let mut clone = graph.clone();
                clifford_simp(&mut clone);
                clone.extractor().gflow().up_to_perm().extract()
            }
        )
    );

    match extract_result {
        Ok(Ok(circuit)) => {
            return Some(circuit);
        }
        Ok(Err(_)) => {
            return None;
        }
        Err(_) => {
            return None;
        }
    }
}

pub fn crossover_gate_list(parentA: &mut Graph, parentB: &mut Graph) -> Graph {
    let circuit_a_option = extract(parentA);
    let circuit_b_option = extract(parentB);

    match (circuit_a_option, circuit_b_option) {
        (Some(circuit_a), Some(circuit_b)) => {
            let gate_list_a = circuit_a.to_basic_gates().gates;
            let gate_list_b = circuit_b.to_basic_gates().gates;

            let split_point_a = gate_list_a.len() / 2;
            let split_point_b = gate_list_b.len() / 2;

            let mut gate_vec = Vec::new();

            gate_vec.extend(gate_list_a.iter().take(split_point_a).cloned());
            gate_vec.extend(gate_list_b.iter().skip(split_point_b).cloned());

            let gate_vec_deque = VecDeque::from(gate_vec);

            let nqubits = usize::max(circuit_a.num_qubits(), circuit_b.num_qubits());
            let mut new_circuit = Circuit::new(nqubits);

            new_circuit.gates = gate_vec_deque;

            return new_circuit.to_graph();
        }
        _ => {
            // if extraction fails return the parent
            return parentA.clone();
        }
    }
}
