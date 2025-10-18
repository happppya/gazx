use quizx::circuit::*;
use quizx::extract::*;
use quizx::gate::GType::HAD;
use quizx::simplify::*;
use quizx::vec_graph::*;

use rand::seq::IndexedRandom;
use rand::{ rng };

use crate::mutations::utilities::get_default_or_random_edge;
use crate::mutations::utilities::get_default_or_random_edge_optimized;

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
    
    let vertex_to_remove = match vertex_to_remove_option {
        Some(vertex) => vertex,
        None => {
            let candidates: Vec<usize> = utilities::get_filtered_nonboundary_vertices(graph, graph.vertices());
            let random_vertex = candidates.choose(&mut rng());
            match random_vertex {
                Some(vertex) => *vertex,
                None => {
                    return;
                }
            }
        }
    };

    utilities::complement(graph, vertex_to_remove);

}

pub fn pivot(graph: &mut Graph, edge_to_remove_option: Option<&EdgeSpecified>) -> () {

    let edge_to_remove = utilities::get_default_or_random_edge_optimized(graph, edge_to_remove_option);

    utilities::complement(graph, edge_to_remove.0);
    utilities::complement(graph, edge_to_remove.1);
    utilities::complement(graph, edge_to_remove.0);

    graph.remove_vertex(edge_to_remove.0);
    graph.remove_vertex(edge_to_remove.1);
    
}

pub fn flip_edge(graph: &mut Graph, edge_to_flip_option: Option<&EdgeSpecified>) {

    let edge_to_flip = get_default_or_random_edge_optimized(graph, edge_to_flip_option);

    match graph.edge_type(edge_to_flip.0, edge_to_flip.1) {
        EType::H => graph.set_edge_type(edge_to_flip.0, edge_to_flip.1, EType::N),
        EType::N => graph.set_edge_type(edge_to_flip.0, edge_to_flip.1, EType::H),
        EType::Wio => return,
    }

}