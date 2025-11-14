use quizx::vec_graph::Graph;
use rand::rng;
use rand::seq::IndexedRandom;

use super::models::GraphPopulation;

use super::output;
use crate::mutation;
use crate::mutation::mutation_runner;

pub fn step_population(population : &mut GraphPopulation) {

  for (i, graph) in population.graphs.iter_mut().enumerate() {

    let mutation = mutation_runner::MUTATIONS_ALL.choose(&mut rng()).unwrap();
    population.last_mutations[i] = mutation;
    
    mutation_runner::run_mutation(graph, mutation);
    
  }

  let (_, extract_millis) = output::benchmark(|| {output::print_population(population)});

  println!("Population extraction and printing time: {:?}", extract_millis);

}