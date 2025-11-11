use quizx::vec_graph::Graph;
use rand::rng;
use rand::seq::IndexedRandom;

use super::models::GraphPopulation;

use super::output;
use crate::mutation;
use crate::mutation::mutation_runner;

fn mutate_graph(graph : &mut Graph) {

  let mutation = mutation_runner::MUTATIONS_ALL.choose(&mut rng()).unwrap();
  mutation_runner::run_mutation(graph, mutation);

}

pub fn step_population(population : &mut GraphPopulation) {

  for graph in &mut population.graphs {
    mutate_graph(graph);
  }

  let (_, extract_millis) = output::benchmark(|| {output::print_population(population)});

  println!("Population extraction and printing time: {:?}", extract_millis);

}