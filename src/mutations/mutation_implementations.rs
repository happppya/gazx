use quizx::circuit::*;
use quizx::extract::*;
use quizx::gate::GType::HAD;
use quizx::simplify::*;
use quizx::vec_graph::*;

use rand::seq::IndexedRandom;
use rand::{ rng };

use super::types::*;
use super::utilities;

pub fn full_reduce(graph: &mut Graph) -> () {
    full_simp(graph);
}

///
/// [mutation 1] Performs local complementation
/// If a vertex is not given, a random vertex is selected based on the criteria:
///   1. The vertex is not a boundary vertex
///   2. No neighbors of the vertex is a boundary vertex
///
pub fn local_complement(graph: &mut Graph, vertex_to_remove_option: Option<usize>) {
    
    let vertex_to_remove = vertex_to_remove_option.unwrap_or_else(|| {
        let candidates: Vec<usize> = utilities::get_filtered_nonboundary_vertices(graph, graph.vertices());
        candidates.choose(&mut rng()).copied().expect("no valid candidates")
    });

    utilities::complement(graph, vertex_to_remove);
}

pub fn pivot(graph: &mut Graph, edge_to_remove_option: Option<&EdgeSpecified>) -> () {

    let candidates : Vec<EdgeSpecified>;

    let edge_to_remove = match edge_to_remove_option {

        Some(edge) => edge,
        None => {
            candidates = utilities::get_nonboundary_edges(graph);
            let random_edge= candidates.choose(&mut rng());
            match random_edge {
                Some(edge) =>  edge,
                None => {
                    return;
                }
            }
        }
    };

    utilities::complement(graph, edge_to_remove.0);
    utilities::complement(graph, edge_to_remove.1);
    utilities::complement(graph, edge_to_remove.0);

    graph.remove_vertex(edge_to_remove.0);
    graph.remove_vertex(edge_to_remove.1);
    
}

pub fn flip_edge(graph: &mut Graph, edge: Option<&EdgeSpecified>) {
    let test: Vec<EdgeSpecified> = graph.edges().collect();
}