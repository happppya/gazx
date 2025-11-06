use std::sync::LazyLock;

use ndarray::MathCell;
use quizx::circuit::*;
use quizx::extract::*;
use quizx::gate::GType::HAD;
use quizx::graph;
use quizx::simplify::*;
use quizx::vec_graph::*;

use rand::Rng;
use rand::random;
use rand::seq::IndexedRandom;
use rand::seq::SliceRandom;
use rand_distr::{ Poisson, Distribution };
use rand::{ rng };

use super::types::*;
use super::utilities;

static RNG_POISSON: LazyLock<Poisson<f64>> = std::sync::LazyLock::new(||
    Poisson::new(3.0).unwrap()
);

#[inline]
fn get_add_edge_candidates(graph: &Graph, first_vertex_candidate: usize) -> Vec<usize> {
    let neighbor_set = utilities::get_neighbor_set(graph, first_vertex_candidate);

    graph
        .vertices()
        .filter(move |vertex| {
            if graph.vertex_type(first_vertex_candidate) == VType::B {
                graph.vertex_type(*vertex) != VType::B && !neighbor_set.contains(vertex)
            } else {
                *vertex != first_vertex_candidate && !neighbor_set.contains(vertex)
            }
        })
        .collect()
}

pub fn add_edge(
    graph: &mut Graph,
    first_vertex_option: Option<usize>,
    second_vertex_option: Option<usize>
) {
    let (first_vertex, second_vertex) = match first_vertex_option {
        Some(first) => {
            let second: usize = match second_vertex_option {
                Some(vertex) => { vertex }
                None => {
                    let candidates = get_add_edge_candidates(graph, first);
                    *candidates.choose(&mut rng()).unwrap()
                }
            };

            (first, second)
        }
        None => {
            let mut first_vertex_candidates = graph.vertex_vec();
            loop {
                if first_vertex_candidates.is_empty() {
                    panic!("No valid vertex found");
                }
                let random_index = rng().random_range(0..first_vertex_candidates.len());
                let candidates = get_add_edge_candidates(
                    graph,
                    first_vertex_candidates[random_index]
                );
                if !candidates.is_empty() {
                    let second_vertex = *candidates.choose(&mut rng()).unwrap();
                    break (first_vertex_candidates[random_index], second_vertex);
                } else {
                    first_vertex_candidates.remove(random_index);
                }
            }
        }
    };

    match graph.edge_type_opt(first_vertex, second_vertex) {
        Some(_) => {
            return;
        }
        None => {
            let edge_type = match rng().random_range(0..=1) {
                0 => EType::N,
                1 => EType::H,
                _ => unreachable!(),
            };
            graph.add_edge_with_type(first_vertex, second_vertex, edge_type);
        }
    }
}

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

pub fn switch_edge(
    graph: &mut Graph,
    edge_to_remove_option: Option<&EdgeSpecified>,
    edge_add_first_vertex_option: Option<usize>,
    edge_add_second_vertex_option: Option<usize>
) {
    remove_edge(graph, edge_to_remove_option);
    add_edge(graph, edge_add_first_vertex_option, edge_add_second_vertex_option);
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

    let random_input_phase = utilities::get_random_input_phase(graph);
    let new_vertex = graph.add_vertex_with_phase(VType::Z, random_input_phase);

    graph.add_edge_with_type(edge_to_split.0, new_vertex, edge_to_split.2);
    graph.add_edge_with_type(new_vertex, edge_to_split.1, edge_to_split.2);
}

pub fn inverse_local_complement(graph: &mut Graph, vertices_to_attach_option: Option<&Vec<usize>>) {
    let mut nonboundary_vertices: Vec<usize>;

    let vertices_to_attach: &Vec<usize> = match vertices_to_attach_option {
        Some(_) => { vertices_to_attach_option.unwrap() }
        None => {
            nonboundary_vertices = utilities::get_vertices_nonboundary(graph).collect();
            let n_vertices_to_attach = std::cmp::min(
                (RNG_POISSON.sample(&mut rng()) as usize) + 1usize,
                nonboundary_vertices.len()
            );

            nonboundary_vertices.shuffle(&mut rng());
            nonboundary_vertices.truncate(n_vertices_to_attach);

            &nonboundary_vertices
        }
    };

    let random_input_phase = utilities::get_random_input_phase(graph);
    let new_vertex = graph.add_vertex_with_phase(VType::Z, random_input_phase);
    let edge_set = utilities::get_edge_set(graph);

    for i in 0..vertices_to_attach.len() {
        let n1 = vertices_to_attach[i];
        graph.add_edge_with_type(n1, new_vertex, EType::H);

        for j in i + 1..vertices_to_attach.len() {
            let n2 = vertices_to_attach[j];

            let edge: EdgeGeneral = (n1, n2);

            if edge_set.contains(&edge) {
                graph.remove_edge(n1, n2);
            } else {
                graph.add_edge_with_type(n1, n2, EType::H);
            }
        }
    }
}
