use std::collections::HashSet;
use quizx::{vec_graph::*};
use rand::{ rng, seq::IndexedRandom, Rng };

use super::types::*;

fn has_nonboundary_neighbors(graph: &Graph, vertex: usize) -> bool {
    for neighbor_vertex in graph.neighbor_vec(vertex) {
        if graph.vertex_type(neighbor_vertex) == VType::B {
            return false;
        }
    }

    return true;
}

fn get_nonboundary_edges(graph: &Graph) -> Vec<EdgeSpecified> {
    let mut candidates: Vec<EdgeSpecified> = Vec::new();

    for edge in graph.edges() {
        if graph.vertex_type(edge.0) == VType::B {
            continue;
        }
        if graph.vertex_type(edge.1) == VType::B {
            continue;
        }

        if !has_nonboundary_neighbors(graph, edge.0) {
            continue;
        }
        if !has_nonboundary_neighbors(graph, edge.1) {
            continue;
        }

        candidates.push(edge);
    }

    return candidates;
}

///
/// Returns vertices such that the vertex is non-boundary and all neighboring vertices are non-boundary
///
/// # Parameters
/// * `graph` - Graph
/// * `vertices` - Candidate vertices
pub fn get_filtered_nonboundary_vertices(
    graph: &Graph,
    vertices: impl Iterator<Item = usize>
) -> Vec<usize> {
    let mut candidates: Vec<usize> = Vec::new();

    for vertex in vertices {
        if graph.vertex_type(vertex) == VType::B {
            continue;
        }

        if !has_nonboundary_neighbors(graph, vertex) {
            continue;
        }

        candidates.push(vertex);
    }

    return candidates;
}

pub fn default_edge(
    graph: &Graph,
    edge_option: Option<&EdgeSpecified>
) -> Box<EdgeSpecified> {


    //TODO something is wrong with this. Maybe overhead from vector copying. Try hash graph.
    match edge_option {
        Some(&edge) => Box::new(edge),
        None => {
            let target_index = rng().random_range(0..graph.num_edges());
            Box::new(graph.edge_vec()[target_index])
        }
    }
}

pub fn default_edge_old(
    graph: &Graph,
    edge_option: Option<&EdgeSpecified>
) -> EdgeSpecified {
    match edge_option {
        Some(edge) => *edge,
        None => {
            let mut i: usize = 0;
            let target_index = rng().random_range(0..graph.num_edges());

            let mut result: Option<EdgeSpecified> = None;

            for edge in graph.edges() {
                if i == target_index {
                    result = Some(edge);
                    break;
                }
                i += 1;
            }

            result.expect("Graph should have at least one edge")
        }
    }
}

///
/// Performs a complement
///
/// # Parameters
/// * `graph` - Graph
/// * `vertex` - The pivotal vertex
pub fn complement(graph: &mut Graph, vertex: usize) -> () {
    let neighbors: Vec<usize> = graph.neighbor_vec(vertex);

    let mut edge_set: HashSet<EdgeGeneral> = HashSet::new();

    for (n1, n2, _) in graph.edges() {
        edge_set.insert((n1, n2));
    }

    //todo just iterate from i to j where j starts at i, no need for hashset or vector

    for i in 0..neighbors.len() {
        for j in i + 1..neighbors.len() {
            let n1 = neighbors[i];
            let n2 = neighbors[j];

            let edge: EdgeGeneral = (n1, n2);

            if edge_set.contains(&edge) {
                graph.remove_edge(n1, n2);
            } else {
                graph.add_edge_with_type(n1, n2, EType::H);
            }
        }
    }
}
