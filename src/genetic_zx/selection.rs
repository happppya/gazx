use std::collections::HashSet;
use rand::{ rng, seq::SliceRandom };

use crate::genetic_zx::models::PopulationComponents;

pub fn worst_individuals_iter(
    population: &PopulationComponents,
    individuals: usize,
) -> impl Iterator<Item = usize> {
    let mut indices: Vec<usize> = (0..population.fitness.len()).collect();

    // sort indices by fitness (worst first)
    indices.sort_by(|&a, &b| {
        population.fitness[a]
            .partial_cmp(&population.fitness[b])
            .unwrap()
    });

    let worst_count = (individuals as f32 * 0.9).round() as usize;
    let random_count = individuals - worst_count;

    // split worst vs remaining
    let (worst, rest) = indices.split_at(worst_count.min(indices.len()));

    // pick random individuals from the remaining population
    let mut random_selection: Vec<usize> = rest.to_vec();
    random_selection.shuffle(&mut rng());

    // combine results
    let mut selected = Vec::with_capacity(individuals);
    selected.extend_from_slice(worst);
    selected.extend(random_selection.into_iter().take(random_count));

    selected.into_iter()
}


pub fn repopulate(
  population: &mut PopulationComponents,
  worst_individuals_iter: impl Iterator<Item = usize>
) {

  let worst_set: HashSet<usize> = worst_individuals_iter.collect();
  let population_size = population.graph.len();

  for individual in worst_set.clone() {
    println!("removing {}", individual);
  }

  let mut good_indices: Vec<usize> = (0..population_size).filter(|i| !worst_set.contains(i)).collect();

  good_indices.shuffle(&mut rng());

  // replace each worst individual with a randomly chosen good one
  for (target_idx, &replacement_idx) in worst_set.iter().zip(good_indices.iter()) {
    population.graph[*target_idx] = population.graph[replacement_idx].clone();
  }
}
