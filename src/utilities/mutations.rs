use quizx::circuit::*;
use quizx::extract::*;
use quizx::simplify::*;
use quizx::vec_graph::*;

pub fn complement(graph : Graph, vertex : usize) {

  let neighbors : Vec<usize> = graph.neighbors(vertex).collect();
  let mut done_neighbors : Vec<usize> = Vec::new();

  let edges = graph.edges();
  
  for (node, idk, asdf) in edges {
    println!("{:?}, {:?}, {:?}", node, idk, asdf);
  }

  for &n1 in &neighbors {
    done_neighbors.push(n1);

    for &n2 in &neighbors {

      if (done_neighbors.contains(&n2)) {
        continue;
      }

    }

  }

}