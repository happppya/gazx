use std::collections::HashSet;

use quizx::circuit::*;
use quizx::extract::*;
use quizx::gate::GType::HAD;
use quizx::simplify::*;
use quizx::vec_graph::*;

use rand::seq::IndexedRandom;
use rand::{rng, seq::SliceRandom};

type EdgeSpecified = (usize, usize, EType);
type EdgeGeneral = (usize, usize);

fn complement(graph : &mut Graph, vertex : usize) -> () {

  let neighbors : Vec<usize> = graph.neighbors(vertex).collect();
  let mut done_neighbors : Vec<usize> = Vec::new();

  let edge_iterator = graph.edges();
  let mut edge_set : HashSet<EdgeGeneral> = HashSet::new();
  
  for (n1, n2, _) in edge_iterator {
    edge_set.insert((n1, n2));
  }

  for &n1 in &neighbors {
    done_neighbors.push(n1);

    for &n2 in &neighbors {

      if (done_neighbors.contains(&n2)) {
        continue;
      }

      let edge : EdgeGeneral = (n1, n2);

      if (edge_set.contains(&edge)) {
        graph.remove_edge(n1, n2);
      } else {
        graph.add_edge_with_type(n1, n2, EType::H);
      }

    }

  }

}

pub fn full_reduce(graph : &mut Graph) -> () {
  full_simp(graph);
}

/// 
/// [mutation 1] Performs local complementation
/// If a vertex is not given, a random vertex is selected based on the criteria:
///   1. The vertex is not a boundary vertex
///   2. No neighbors of the vertex is a boundary vertex
/// 
pub fn local_complement(graph : &mut Graph, vertex_to_remove : Option<usize>) {

  let vertex = vertex_to_remove.unwrap_or_else(|| {

    let mut candidates : Vec<usize> = Vec::new();

    for candidate_vertex in graph.vertices() {

      if (graph.vertex_type(candidate_vertex) == VType::B) {continue;}

      let mut neighbor_valid = true;

      for neighbor_vertex in graph.neighbors(candidate_vertex) {
        if graph.vertex_type(neighbor_vertex) == VType::B {
          neighbor_valid = false;
          break;
        }
      }

      if !neighbor_valid {continue;}

      candidates.push(candidate_vertex);

    }
    
    let mut rng = rng();
    candidates.choose(&mut rng).copied().expect("no valid candidates")

  });

  complement(graph, vertex);

}
