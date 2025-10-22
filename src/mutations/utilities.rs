use std::collections::HashSet;
use quizx::{vec_graph::*};
use rand::{ rng, seq::IndexedRandom, Rng };

use super::types::*;

fn has_nonboundary_neighbors(graph: &Graph, vertex: usize) -> bool {

    for neighbor_vertex in graph.neighbors(vertex) {
        if graph.vertex_type(neighbor_vertex) == VType::B {
            return false;
        }
    }

    return true;
}

pub fn get_vertices_nonboundary<'a>(graph : &'a Graph) -> impl Iterator<Item=usize> + 'a {

    graph.vertices().filter(move |v| graph.vertex_type(*v) == VType::B)
    
}

///
/// Returns vertices such that the vertex is non-boundary and all neighboring vertices are non-boundary
///
/// # Parameters
/// * `graph` - Graph
/// * `vertices` - Candidate vertices
/// 
#[inline]
fn push_nonboundary_vertices(
    
    candidates : &mut Vec<usize>,
    graph: &Graph

) -> () {

    for vertex in graph.vertices() {

        if graph.vertex_type(vertex) == VType::B {
            continue;
        }

        if !has_nonboundary_neighbors(graph, vertex) {
            continue;
        }

        candidates.push(vertex);
    }
}

pub fn default_vertex(
    graph : &Graph,
    vertex_option : Option<usize>
) -> usize {

    match vertex_option {
        Some(vertex) => vertex,
        None => {
            
            let mut candidates = Vec::new();
            push_nonboundary_vertices(&mut candidates, graph);
            
            *candidates.choose(&mut rng()).unwrap()
            
        }
    };

    0usize

}

pub fn default_edge(
    graph: &Graph,
    edge_option: Option<&EdgeSpecified>
) -> EdgeSpecified {

    match edge_option {
        Some(edge) => *edge,
        None => {
            *graph.edge_vec().choose(&mut rng()).unwrap()
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
