use std::collections::VecDeque;

use quizx::{ circuit::Circuit, extract::ToCircuit, graph::GraphLike, simplify::clifford_simp, vec_graph::Graph };
use rand::{Rng, rng, seq::IndexedRandom};

use super::models::PopulationComponents;

const MAX_TRIES : u32 = 1;

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

pub fn crossover_gate_list(population: &PopulationComponents, parent_a: usize, parent_b: usize) -> Graph {
    let circuit_a = &population.circuit[parent_a];
    let circuit_b = &population.circuit[parent_b];

    let gate_list_a = circuit_a.to_basic_gates().gates;
    let gate_list_b = circuit_b.to_basic_gates().gates;

    let mut rng = rng();

    // Generate random split points
    let split_point: usize = if gate_list_a.is_empty() { 
        0 
    } else {
        rng.random_range(0..=gate_list_a.len()) 
    };
    
    // Recombine gates
    let mut gate_vec = Vec::new();
    gate_vec.extend(gate_list_a.iter().take(split_point).cloned());
    gate_vec.extend(gate_list_b.iter().skip(split_point).cloned());

    let gate_vec_deque = VecDeque::from(gate_vec);

    let nqubits = usize::max(circuit_a.num_qubits(), circuit_b.num_qubits());
    let mut new_circuit = Circuit::new(nqubits);

    new_circuit.gates = gate_vec_deque;
    
    new_circuit.to_graph()
}

pub fn crossover_subgraph(population: &PopulationComponents, parent_a: usize, parent_b: usize) -> Graph {
    let mut rng = rand::rng();

    let graph_b = &population.graph[parent_b];
    let verts_b = graph_b.vertex_vec();

    for tries in 0..MAX_TRIES {
        let mut graph_a = population.graph[parent_a].clone();

        // Select and remove random vertices from Graph A
        let verts_a = graph_a.vertex_vec();
        let num_remove = rng.random_range(1..=std::cmp::max(1, verts_a.len() / 3)); 
        let to_remove: Vec<_> = verts_a.choose_multiple(&mut rng, num_remove).cloned().collect();
        
        for v in to_remove {
            graph_a.remove_vertex(v);
        }

        // Cache the remaining vertices in A, the valid targets for stitching
        let remaining_verts_a = graph_a.vertex_vec();

        // Select random vertices to extract from Graph B
        let num_extract = rng.random_range(1..=std::cmp::max(1, verts_b.len() / 3));
        let to_extract: Vec<_> = verts_b.choose_multiple(&mut rng, num_extract).cloned().collect();

        let target_degrees: Vec<usize> = to_extract.iter().map(|&v| graph_b.degree(v)).collect();

        let subgraph_b = graph_b.subgraph_from_vertices(to_extract);
        let vertex_map = graph_a.append_graph(&subgraph_b);

        let new_verts: Vec<_> = vertex_map.values().cloned().collect();
        
        // Stitch the new floating vertices
        for (i, &v_new) in new_verts.iter().enumerate() {

            let target_degree = *target_degrees.get(i).unwrap_or(&0);
            
            // Add random edges until the current degree matches the target degree
            while graph_a.degree(v_new) < target_degree {

                if let Some(&target_v) = remaining_verts_a.choose(&mut rng) {
                    // Prevent self-loop
                    if v_new != target_v {
                        graph_a.add_edge(v_new, target_v);
                    }
                } else {
                    break; 
                }
            }
        }

        if tries == MAX_TRIES - 1 {
            return graph_a;
        }

        let mut extract_graph = graph_a.clone();
        if let Some(_new_circuit) = extract(&mut extract_graph) {
            return extract_graph;
        }
        
    }

    population.graph[parent_a].clone()
}