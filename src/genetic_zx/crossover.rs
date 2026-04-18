use std::collections::{ HashMap, HashSet, VecDeque };

use quizx::{
    circuit::Circuit,
    extract::ToCircuit,
    graph::{ self, EType, GraphLike, VType },
    simplify::{ self, clifford_simp },
    vec_graph::Graph,
};
use rand::{ Rng, rng, seq::{ IndexedRandom, SliceRandom } };

use super::models::PopulationComponents;

const MAX_TRIES: u32 = 5;

#[inline]
fn has_nonboundary_neighbors(graph: &Graph, vertex: usize) -> bool {
    for neighbor_vertex in graph.neighbors(vertex) {
        if
            graph.vertex_type(neighbor_vertex) != VType::Z &&
            graph.vertex_type(neighbor_vertex) != VType::X
        {
            return false;
        }
    }

    return true;
}

fn push_nonboundary_vertices(candidates: &mut Vec<usize>, graph: &Graph) -> () {
    for vertex in graph.vertices() {
        if graph.vertex_type(vertex) != VType::Z && graph.vertex_type(vertex) != VType::X {
            continue;
        }

        if !has_nonboundary_neighbors(graph, vertex) {
            continue;
        }

        candidates.push(vertex);
    }
}

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

pub fn crossover_gate_list(
    population: &PopulationComponents,
    parent_a: usize,
    parent_b: usize
) -> Graph {
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

pub fn crossover_subgraph(
    population: &PopulationComponents,
    parent_a: usize,
    parent_b: usize
) -> Graph {
    let mut rng = rand::rng();

    let graph_b = &population.graph[parent_b];
    let mut verts_b: Vec<usize> = Vec::new();
    push_nonboundary_vertices(&mut verts_b, &graph_b);

    for tries in 0..MAX_TRIES {
        let mut graph_a = population.graph[parent_a].clone();

        // Select and remove random vertices from Graph A
        let mut verts_a: Vec<usize> = Vec::new();
        push_nonboundary_vertices(&mut verts_a, &graph_a);

        let num_remove = rng.random_range(1..=std::cmp::max(1, verts_a.len() / 3));
        let to_remove: Vec<_> = verts_a.choose_multiple(&mut rng, num_remove).cloned().collect();

        for v in to_remove {
            graph_a.remove_vertex(v);
        }

        // Cache the remaining vertices in A, the valid targets for stitching
        let mut remaining_verts_a: Vec<usize> = Vec::new();
        push_nonboundary_vertices(&mut remaining_verts_a, &graph_a);

        // Select random vertices to extract from Graph B
        let num_extract = rng.random_range(1..=std::cmp::max(1, verts_b.len() / 3));
        let to_extract: Vec<_> = verts_b.choose_multiple(&mut rng, num_extract).cloned().collect();

        let target_degrees: Vec<usize> = to_extract
            .iter()
            .map(|&v| graph_b.degree(v))
            .collect();

        let subgraph_b = graph_b.subgraph_from_vertices(to_extract);
        let vertex_map = graph_a.append_graph(&subgraph_b);

        let new_verts: Vec<_> = vertex_map.values().cloned().collect();

        for (i, &v_new) in new_verts.iter().enumerate() {
            let target_degree = *target_degrees.get(i).unwrap_or(&0);
            let current_degree = graph_a.degree(v_new);

            if current_degree < target_degree {
                let edges_needed = (target_degree - current_degree).min(2);

                // Sample without replacement to avoid duplicates
                let unique_targets: Vec<_> = remaining_verts_a
                    .iter()
                    .filter(
                        |&v|
                            graph_a.vertex_type(*v) == VType::Z ||
                            graph_a.vertex_type(*v) == VType::X
                    )
                    .collect::<Vec<&usize>>()
                    .choose_multiple(&mut rng, edges_needed)
                    .cloned()
                    .collect();

                for target_v in unique_targets {
                    if v_new != *target_v {
                        graph_a.add_edge(v_new, *target_v);
                    }
                }
            }
        }

        println!(
            "graph A stats with vertices {} and edges {} ",
            graph_a.num_vertices(),
            graph_a.num_edges()
        );

        graph_a.pack(true);

        println!(
            "packed stats with vertices {} and edges {} ",
            graph_a.num_vertices(),
            graph_a.num_edges()
        );

        if tries == MAX_TRIES - 1 {
            let mut result = graph_a.copy(false);
            result.inputs_mut().clone_from(&graph_a.inputs());
            result.outputs_mut().clone_from(&graph_a.outputs());

            println!(
                "result stats with vertices {} and edges {} ",
                graph_a.num_vertices(),
                graph_a.num_edges()
            );

            return result;
        }

        let mut extract_graph = graph_a.clone();
        if let Some(_new_circuit) = extract(&mut extract_graph) {
            let mut result = graph_a.copy(false);
            result.inputs_mut().clone_from(&graph_a.inputs());
            result.outputs_mut().clone_from(&graph_a.outputs());
            return result;
        }
    }

    population.graph[parent_a].clone()
}

fn compute_layers(graph: &Graph) -> HashMap<usize, usize> {
    // Ideally: derive from gflow (layer = measurement order)
    // Fallback: BFS from inputs / boundaries

    let mut depth = HashMap::new();
    let mut queue = std::collections::VecDeque::new();

    for &input in graph.inputs() {
        depth.insert(input, 0);
        queue.push_back(input);
    }

    while let Some(v) = queue.pop_front() {
        let d = depth[&v];

        for n in graph.neighbors(v) {
            if !depth.contains_key(&n) {
                depth.insert(n, d + 1);
                queue.push_back(n);
            }
        }
    }

    depth
}

/// Performs patch crossover between two graphs
pub fn crossover_patch_replacement(
    population: &PopulationComponents,
    parent_a: usize,
    parent_b: usize
) -> Graph {
    let mut rng = rand::rng();
    let graph_b = &population.graph[parent_b];

    let mut graph_a = Graph::new();

    for tries in 0..MAX_TRIES {
        graph_a = population.graph[parent_a].clone();

        let patch_b = match find_random_patch(graph_b, &mut rng) {
            Some(p) => p,
            None => {
                continue;
            }
        };

        let b_boundary_size = patch_b.external_edges.len();

        // A patch with no external connections is an isolated island which doesn't do anything
        if b_boundary_size == 0 {
            continue;
        }

        // Look for a subgraph in A that has the exact same boundary size as the patch in B
        let cavity_a = match find_matching_cavity(&graph_a, b_boundary_size, &mut rng) {
            Some(c) => c,
            None => {
                continue;
            }
        };

        // Delete interior nodes of cavity from A to make room for patch
        for &v in &cavity_a.interior {
            graph_a.remove_vertex(v);
        }

        // Memory copy will cause conflicting node IDs. Must instantiate new nodes in A that mimic the ones from B
        let mut vertex_map = HashMap::new();
        for (idx, &old_v_b) in patch_b.interior.iter().enumerate() {
            let v_type = graph_b.vertex_type(old_v_b);
            let phase = graph_b.phase(old_v_b);

            // Add the cloned node into A and map the old B-index to the new A-index
            let new_v_a = graph_a.add_vertex_with_phase(v_type, phase);
            vertex_map.insert(idx, new_v_a);
        }

        // Reconnect the interior edges
        for i in 0..patch_b.interior.len() {
            for j in i + 1..patch_b.interior.len() {
                let u_b = patch_b.interior[i];
                let v_b = patch_b.interior[j];

                if let Some(et) = graph_b.edge_type_opt(u_b, v_b) {
                    let u_a = *vertex_map.get(&i).unwrap();
                    let v_a = *vertex_map.get(&j).unwrap();
                    graph_a.add_edge_with_type(u_a, v_a, et);
                }
            }
        }

        // Patch from B is now floating inside A
        // Must connect the dangling edges to boundary anchors on the perimeter of A's cavity
        let mut anchors_a = cavity_a.boundary_anchors.clone();

        anchors_a.shuffle(&mut rng);

        for (i, b_ext_edge) in patch_b.external_edges.iter().enumerate() {
            // Assign each dangling edge to an anchor in A
            let anchor_a = anchors_a[i % anchors_a.len()];
            let new_v_in_a = *vertex_map
                .get(&b_ext_edge.interior_idx)
                .expect("Vertex index mapping failed during stitch");

            graph_a.add_edge_with_type(new_v_in_a, anchor_a, b_ext_edge.edge_type);
        }

        // Return whatever result without checking extract if we hit max limit of retries
        if tries == MAX_TRIES - 1 {
            break;
        }

        graph_a.pack(true);

        let mut test_extract = graph_a.clone();
        if let Some(_) = extract(&mut test_extract) {
            // println!("SUCCESS!!!!!! Patch crossover produced extractable graph on try {}", tries);

            // Copy is necessary to get rid of wasted space. Memory will rise exponentially otherwise
            let mut result = graph_a.copy(false);

            // IO has to be copied over manually after copy method
            result.inputs_mut().clone_from(&graph_a.inputs());
            result.outputs_mut().clone_from(&graph_a.outputs());
            return result;
        } else {
            // println!("Try {}: Extraction failed for resulting graph.", tries);
        }
    }

    graph_a.pack(true);

    let mut result = graph_a.copy(false);
    result.inputs_mut().clone_from(&graph_a.inputs());
    result.outputs_mut().clone_from(&graph_a.outputs());

    // println!(
    //     "DEBUG: All tries exhausted. Returning last attempt with vertices {} and edges {} ",
    //     graph_a.num_vertices(),
    //     graph_a.num_edges()
    // );

    return result;
}

/// Finds a random, connected cluster of internal nodes in a graph
fn find_random_patch(graph: &Graph, rng: &mut impl Rng) -> Option<Patch> {
    let mut non_boundary = Vec::new();
    push_nonboundary_vertices(&mut non_boundary, graph);

    // Should not select IO nodes at the start
    let seed = *non_boundary.choose(rng)?;

    let mut interior = Vec::new();
    let mut q = VecDeque::new();
    let mut visited = HashSet::new();

    q.push_back(seed);
    visited.insert(seed);
    interior.push(seed);

    let target_size = rng.random_range(2..6);

    // BFS to grow the patch from seed node
    while let Some(v) = q.pop_front() {
        if interior.len() >= target_size {
            break;
        }
        for n in graph.neighbors(v) {
            if !visited.contains(&n) {
                let vt = graph.vertex_type(n);
                if vt == VType::Z || vt == VType::X {
                    visited.insert(n);
                    interior.push(n);
                    q.push_back(n);
                }
            }
        }
    }

    // Identify the edges connecting nodes inside the patch to outside the patch
    let mut external_edges = Vec::new();
    for (idx, &v) in interior.iter().enumerate() {
        for n in graph.neighbors(v) {
            if !interior.contains(&n) {
                external_edges.push(ExternalEdge {
                    interior_idx: idx,
                    edge_type: graph.edge_type(v, n),
                });
            }
        }
    }

    Some(Patch { interior, external_edges })
}

/// Searches the graph for a connected cluster of nodes (a cavity) whose number of external connections matches `target_boundary`
fn find_matching_cavity(
    graph: &Graph,
    target_boundary: usize,
    rng: &mut impl Rng
) -> Option<Cavity> {
    let mut non_boundary = Vec::new();
    push_nonboundary_vertices(&mut non_boundary, graph);

    // Select 10 random starting points to look for holes instead of searching entire space
    let seeds: Vec<_> = non_boundary.choose_multiple(rng, 10).collect();

    for &seed in seeds {
        let mut interior = Vec::new();
        let mut q = VecDeque::new();
        let mut visited = HashSet::new();

        q.push_back(seed);
        visited.insert(seed);
        interior.push(seed);

        // Iteratively grow a cavity with BFS
        for _ in 0..15 {
            let mut boundary_anchors = Vec::new();
            for &v in &interior {
                for n in graph.neighbors(v) {
                    if !interior.contains(&n) {
                        boundary_anchors.push(n);
                    }
                }
            }

            if boundary_anchors.len() == target_boundary {
                if
                    interior
                        .iter()
                        .any(
                            |&v|
                                graph.vertex_type(v) != VType::Z && graph.vertex_type(v) != VType::X
                        )
                {
                    break; // Invalid cavity containing boundary nodes
                }
                return Some(Cavity { interior, boundary_anchors });
            }

            // Abandon if the hole is too big
            if boundary_anchors.len() > target_boundary + 2 {
                break;
            }

            // Expand the cavity by adding one neighboring valid spider
            let mut expanded = false;
            let current_neighbors: Vec<_> = graph.neighbors(*interior.last().unwrap()).collect();
            for n in current_neighbors {
                if !visited.contains(&n) {
                    let vt = graph.vertex_type(n);
                    if vt == VType::Z || vt == VType::X {
                        visited.insert(n);
                        interior.push(n);
                        q.push_back(n);
                        expanded = true;
                        break; // Only expand by one node per iteration
                    }
                }
            }
            if !expanded {
                break;
            } // no more valid neighbors to expand to
        }
    }
    None
}

/// A donor chunk taken from a graph
struct Patch {
    /// The local IDs of the spiders in this chunk
    interior: Vec<usize>,
    /// The "dangling" edges that connected this chunk to its original graph
    external_edges: Vec<ExternalEdge>,
}

/// Represents a single connection passing through the boundary of a Patch
struct ExternalEdge {
    /// The specific node inside the patch that this edge is attached to
    interior_idx: usize,
    edge_type: EType,
}

/// A hollowed-out section in the recipient graph ready to receive a Patch
struct Cavity {
    /// The nodes that will be deleted to make room for the new patch
    interior: Vec<usize>,
    /// The nodes on the perimeter that the new patch will be stitched into
    boundary_anchors: Vec<usize>,
}
