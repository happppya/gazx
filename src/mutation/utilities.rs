use std::{collections::HashSet, hash::Hash};
use quizx::{phase::Phase, vec_graph::*};
use rand::{ Rng, random, rng, seq::{IndexedRandom, IteratorRandom} };

use super::types::*;

pub fn get_random_input_phase(graph : &Graph) -> Phase {
    graph.phase(
        *graph.inputs().choose(&mut rng()).expect("Graph should have at least one input.")
    )
}

fn has_nonboundary_neighbors(graph: &Graph, vertex: usize) -> bool {

    for neighbor_vertex in graph.neighbors(vertex) {
        if graph.vertex_type(neighbor_vertex) == VType::B {
            return false;
        }
    }

    return true;
}

pub fn get_vertices_nonboundary<'a>(graph : &'a Graph) -> impl Iterator<Item=usize> + 'a {

    graph.vertices().filter(move |v| graph.vertex_type(*v) != VType::B)
    
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
) -> Option<usize> {

    match vertex_option {
        Some(vertex) => vertex,
        None => {
            
            let mut candidates: Vec<usize> = Vec::new();
            push_nonboundary_vertices(&mut candidates, graph);

            if candidates.is_empty() {
                return None;
            }

            return candidates.choose(&mut rng()).copied();
            
        }
    };

    None

}

pub fn default_edge_nth(
    graph: &Graph,
    edge_option: Option<&EdgeSpecified>
) -> EdgeSpecified {
    match edge_option {
        Some(edge) => *edge,
        None => {

            let mut i: usize = 0;
            let target_index = rng().random_range(0..graph.num_edges());

            let result: Option<EdgeSpecified> = graph.edges().nth(target_index);

            return result.expect("Graph should have at least one edge")
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

            return result.expect("Graph should have at least one edge")
        }
    }
}

pub fn default_edge_remove(
    graph : &Graph,
    edge_option: Option<&EdgeSpecified>
) -> Option<EdgeSpecified> {

    match edge_option {
        Some(edge) => Some(*edge),
        None => {

            let candidates : Vec<EdgeSpecified> = 
                graph
                    .edges()
                    .filter(|e| graph.degree(e.0) > 1 && graph.degree(e.1) > 1)
                    .collect();

            return candidates.choose(&mut rng()).copied()
            
        }
    }

}

pub fn default_pivot_vertex_pair(
    graph : &mut Graph,
    vertex_pair_option : Option<&(usize, usize)>
) -> Option<(usize, usize)> {
    match vertex_pair_option {
        Some(pair) => {Some(*pair)},
        None => {

            let mut valid_edges : Vec<EdgeSpecified> = Vec::new();
            
            for edge in graph.edges() {
                if 
                graph.vertex_type(edge.0) != VType::B &&
                graph.vertex_type(edge.1) != VType::B &&
                has_nonboundary_neighbors(graph, edge.0) &&
                has_nonboundary_neighbors(graph, edge.1) {
                    valid_edges.push(edge);
                }
            }

            let random_edge = valid_edges.choose(&mut rng());

            match random_edge {
                Some(edge) => return Some((edge.0, edge.1)),
                None => return None,
            }

        }
    }
}

#[inline]
pub fn get_edge_set(graph : &Graph) -> HashSet<EdgeGeneral> {

    graph.edges()
        .map(|(n1, n2, _)| (n1, n2)) 
        .collect()
    
}

#[inline]
pub fn get_neighbor_set(graph : &Graph, vertex : usize) -> HashSet<usize> {
    HashSet::from_iter(graph.neighbors(vertex))
}

///
/// Performs a complement
///
/// # Parameters
/// * `graph` - Graph
/// * `vertex` - The pivotal vertex
pub fn complement(graph: &mut Graph, vertex: usize) -> () {
    
    let neighbors: Vec<usize> = graph.neighbor_vec(vertex);
    let edge_set = get_edge_set(graph);

    for i in 0..neighbors.len() {

        let n1 = neighbors[i];

        for j in i + 1..neighbors.len() {
            
            let n2 = neighbors[j];

            let edge: EdgeGeneral = (n1, n2);

            if edge_set.contains(&edge) {
                graph.remove_edge(n1, n2);
            } else {

                println!("Complement adding edge between vertices {:?} and {:?}", graph.vertex_type(n1), graph.vertex_type(n2));
                graph.add_edge_with_type(n1, n2, EType::H);
                
            }
        }
    }
}
