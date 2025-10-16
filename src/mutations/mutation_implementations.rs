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
pub fn local_complement(graph: &mut Graph, vertex_to_remove: Option<usize>) {
    let vertex = vertex_to_remove.unwrap_or_else(|| {
        let candidates: Vec<usize> = utilities::get_filtered_nonboundary_vertices(graph, graph.vertices());
        candidates.choose(&mut rng()).copied().expect("no valid candidates")
    });

    utilities::complement(graph, vertex);
}

pub fn pivot(graph: &mut Graph, edge_to_remove: &EdgeSpecified) -> () {

    if edge_to_remove.len() == 0 {

        let candidates = utilities::get_nonboundary_edges(graph);
        if candidates.len() == 0 {
            return;
        }

        match candidates.choose(&mut rng()) {
            Some(random_edge) => {
                edge_to_remove = random_edge;
            }
            None => {
                return;
            }
        }

    }

    utilities::complement(graph, edge_to_remove[0]);
    utilities::complement(graph, edge_to_remove[1]);
    utilities::complement(graph, edge_to_remove[0]);

    graph.remove_vertex(edge_to_remove[0]);
    graph.remove_vertex(edge_to_remove[1]);
}

pub fn flip_edge(graph: &mut Graph, edge: EdgeGeneral) {
    let test: Vec<EdgeSpecified> = graph.edges().collect();
}