use quizx::circuit::*;
use quizx::extract::*;
use quizx::gate::GType::HAD;
use quizx::graph;
use quizx::simplify::*;
use quizx::vec_graph::*;

use rand::seq::IndexedRandom;
use rand::{ rng };

use super::types::*;
use super::utilities;

pub fn full_reduce(graph: &mut Graph) -> () {
    full_simp(graph);
}

pub fn remove_edge(graph: &mut Graph, edge_to_remove_option: Option<&EdgeSpecified>) -> () {
    let edge_to_remove = utilities::default_edge_old(graph, edge_to_remove_option);
    graph.remove_edge(edge_to_remove.0, edge_to_remove.1);
}

pub fn remove_vertex(graph: &mut Graph, vertex_to_remove_option: Option<usize>) -> () {
    let vertex_to_remove = utilities::default_vertex(graph, vertex_to_remove_option);
    graph.remove_vertex(vertex_to_remove);
}

///
/// [mutation 1] Performs local complementation
/// If a vertex is not given, a random vertex is selected based on the criteria:
///   1. The vertex is not a boundary vertex
///   2. No neighbors of the vertex is a boundary vertex
///
pub fn local_complement(graph: &mut Graph, vertex_to_remove_option: Option<usize>) {
    let vertex_to_remove = utilities::default_vertex(graph, vertex_to_remove_option);

    utilities::complement(graph, vertex_to_remove);
}

pub fn pivot(graph: &mut Graph, edge_to_remove_option: Option<&EdgeSpecified>) -> () {
    let edge_to_remove = utilities::default_edge_old(graph, edge_to_remove_option);

    utilities::complement(graph, edge_to_remove.0);
    utilities::complement(graph, edge_to_remove.1);
    utilities::complement(graph, edge_to_remove.0);

    graph.remove_vertex(edge_to_remove.0);
    graph.remove_vertex(edge_to_remove.1);
}

pub fn flip_edge(graph: &mut Graph, edge_to_flip_option: Option<&EdgeSpecified>) {
    let edge_to_flip = utilities::default_edge_old(graph, edge_to_flip_option);

    match graph.edge_type(edge_to_flip.0, edge_to_flip.1) {
        EType::H => graph.set_edge_type(edge_to_flip.0, edge_to_flip.1, EType::N),
        EType::N => graph.set_edge_type(edge_to_flip.0, edge_to_flip.1, EType::H),
        EType::Wio => {
            return;
        }
    }
}

pub fn split_edge(graph: &mut Graph, edge_to_split_option: Option<&EdgeSpecified>) {
    let edge_to_split = utilities::default_edge(graph, edge_to_split_option);

    graph.remove_edge(edge_to_split.0, edge_to_split.1);

    let random_input_phase = graph.phase(
        *graph.inputs().choose(&mut rng()).expect("Graph should have at least one input.")
    );
    let new_vertex = graph.add_vertex_with_phase(VType::Z, random_input_phase);

    graph.add_edge_with_type(edge_to_split.0, new_vertex, edge_to_split.2);
    graph.add_edge_with_type(new_vertex, edge_to_split.1, edge_to_split.2);
    
}
