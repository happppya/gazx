use std::collections::HashSet;
use rand::{rng, seq::SliceRandom, Rng};

use crate::genetic_zx::models::{PopulationComponents, Hyperparameters};

pub fn repopulate(
    population: &mut PopulationComponents,
    parameters: &Hyperparameters,
) {
    let population_size = population.graph.len();

    let mut indices: Vec<usize> = (0..population_size).collect();
    let elitism_count = (population_size as f64 * parameters.elitism_rate) as usize;

    // Sort descending to put the best individuals at the start of the array.
    let (_, pivot, non_elites) = indices.select_nth_unstable_by(elitism_count, |&a, &b| {
        population.fitness[b]
            .partial_cmp(&population.fitness[a])
            .unwrap()
    });

    // The individuals to replace are the pivot and everything after it
    let mut target_indices = vec![*pivot];
    target_indices.extend_from_slice(non_elites);

    let mut rng = rng();

    for target_idx in target_indices {
        let mut best_parent_idx = 0;
        let mut best_fitness = population.fitness[0];

        // Tournament selection
        for i in 0..parameters.tournament_size {
            let competitor_idx = rng.random_range(0..population_size);
            let fitness = population.fitness[competitor_idx];

            if i == 0 || fitness > best_fitness {
                best_fitness = fitness;
                best_parent_idx = competitor_idx;
            }
        }

        // Overwrite the individual with the tournament winner
        population.graph[target_idx] = population.graph[best_parent_idx].clone();
        population.fitness[target_idx] = population.fitness[best_parent_idx];
    }
}