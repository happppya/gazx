use std::collections::HashSet;

use quizx::circuit::*;
use quizx::extract::*;
use quizx::gate::GType::HAD;
use quizx::simplify::*;
use quizx::vec_graph::*;

type EdgeSpecified = (usize, usize, EType);
type EdgeGeneral = (usize, usize);

pub fn complement(graph : &mut Graph, vertex : usize) {

  let neighbors : Vec<usize> = graph.neighbors(vertex).collect();
  let mut done_neighbors : Vec<usize> = Vec::new();

  let edge_list = graph.edges();
  let mut edge_set : HashSet<EdgeGeneral> = HashSet::new();
  
  for (n1, n2, _) in edge_list {
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