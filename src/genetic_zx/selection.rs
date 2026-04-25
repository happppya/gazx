use quizx::graph::GraphLike;
use quizx::vec_graph::Graph;
use rand::{rng, Rng};

use super::models::{Hyperparameters, PopulationComponents};
use super::crossover;
use super::fitness;
use super::genetic_util;

pub fn repopulate(
    population: &mut PopulationComponents,
    parameters: &Hyperparameters,
) {
    let population_size = population.graph.len();
    let mut rng = rng();

    let mut indices: Vec<usize> = (0..population_size).collect();
    let elitism_count = (population_size as f64 * parameters.elitism_rate) as usize;

    // Sort descending to put the best individuals at the start of the array
    let (_, pivot, non_elites) = indices.select_nth_unstable_by(elitism_count, |&a, &b| {
        population.fitness[b]
            .partial_cmp(&population.fitness[a])
            .unwrap()
    });

    let mut target_indices = vec![*pivot];
    target_indices.extend_from_slice(non_elites);

    enum Child {
        CrossedOver(Graph),
        Cloned(usize),
    }

    // Selection and generation
    let mut pending_children = Vec::with_capacity(target_indices.len());

    for _ in 0..target_indices.len() {
        let parent1_idx = tournament_select(population, parameters, &mut rng);
        
        if rng.random::<f64>() < parameters.crossover_rate {
            let parent2_idx = tournament_select(population, parameters, &mut rng);
            let new_graph = crossover::crossover_gate_list(population, parent1_idx, parent2_idx);
            pending_children.push(Child::CrossedOver(new_graph));
        } else {
            pending_children.push(Child::Cloned(parent1_idx));
        }
    }

    // Apply new individuals
    // New fitness and circuits do not have to be applied in this step
    for (target_idx, child) in target_indices.into_iter().zip(pending_children) {
        match child {
            Child::CrossedOver(new_graph) => {
                // Apply the new graph
                population.graph[target_idx] = new_graph;
            }
            Child::Cloned(parent_idx) => {
                population.graph[target_idx] = population.graph[parent_idx].clone();
            }
        }
    }
}

fn tournament_select(
    population: &PopulationComponents,
    parameters: &Hyperparameters,
    rng: &mut impl Rng,
) -> usize {
    let population_size = population.graph.len();

    let mut best_idx = rng.random_range(0..population_size);
    let mut best_fitness = population.fitness[best_idx];

    for _ in 1..parameters.tournament_size {
        let competitor_idx = rng.random_range(0..population_size);
        let fitness = population.fitness[competitor_idx];

        if fitness > best_fitness {
            best_fitness = fitness;
            best_idx = competitor_idx;
        }
    }

    best_idx
}