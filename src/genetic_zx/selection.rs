use quizx::graph::GraphLike;
use rand::{rng, Rng};

use super::models::{Hyperparameters, PopulationComponents};
use super::crossover;

pub fn repopulate(
    population: &mut PopulationComponents,
    parameters: &Hyperparameters,
) {
    let population_size = population.graph.len();
    let mut rng = rng();

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

    for target_idx in target_indices {
        let parent1_idx: usize = tournament_select(population, parameters, &mut rng);
        
        let mut crossover_happened : bool = false;

        let child_graph = if rng.random::<f64>() < parameters.crossover_rate {
            crossover_happened = true;
            
            let parent2_idx: usize = tournament_select(population, parameters, &mut rng);
            crossover::crossover_subgraph(population, parent1_idx, parent2_idx)
        } else {
            population.graph[parent1_idx].clone()
        };

        population.graph[target_idx] = child_graph;

        /*if crossover_happened {
            println!(
                "Crossover: \n\tParent1 idx {}, fitness {} stats {}, \n\tParent2 idx {}, fitness {} stats {}, \n\tTarget index {} stats {}",
                parent1_idx,
                population.fitness[parent1_idx],
                population.graph[parent1_idx].num_vertices(),
                parent2_idx,
                population.fitness[parent2_idx],
                population.graph[parent2_idx].num_vertices(),
                target_idx,
                population.graph[target_idx].num_vertices(),
            );
        }*/
       
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