use genevo::{operator::prelude::*, population::*, prelude::*, types::fmt::Display};
use quizx::{circuit::Circuit, gate::GType::InitAncilla, random_graph::EquatorialStabilizerStateBuilder, vec_graph::Graph};
use rand::{SeedableRng, rngs::StdRng};

type Selection = Vec<bool>;

#[derive(Debug)]
struct GraphPopulation {
  graphs: Vec<Graph>
}

#[derive(Debug)]
struct Problem {

}

impl Problem {
  pub fn new() -> Self {
    Self {}
  }
}

impl<'a> FitnessFunction<Selection, i64> for &'a Problem {
  fn fitness_of(&self, selection : &Selection) -> i64 {
    5
  }
  fn average(&self, values: &[i64]) -> i64 {
      (values.iter().sum::<i64>() as f32 / values.len() as f32 + 0.5).floor() as i64
  }

  fn highest_possible_fitness(&self) -> i64 {
      100
  }

  fn lowest_possible_fitness(&self) -> i64 {
      0
  }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

  let population_size: u32 = 100;
  let num_qubits : usize = 4usize;

  let circuit = &Circuit::from_file("circuits/small/grover_5.qasm")?.to_basic_gates();

  let population : GraphPopulation = GraphPopulation { graphs: ({
    
    let mut random_population : Vec<Graph> = Vec::new();
    
    for i in 0..population_size {
      let mut graph = EquatorialStabilizerStateBuilder { rng: (StdRng::from_os_rng()), qubits: (num_qubits) };
      random_population.push(graph.build());

      let percentage_population_build = (i+1) as f64 / population_size as f64 * 100.0;
      if (percentage_population_build.floor() as i32) % 5 == 0 {
        println!("Building population. {:?}%", percentage_population_build);
      }
      
    }

    random_population

  }) };

  let initial_population: Population<Selection> = build_population()
        .with_genome_builder(BinaryEncodedGenomeBuilder::new(
            100
        ))
        .of_size(400)
        .uniform_at_random();

  let problem = Problem::new();

  let mut simulation = simulate(
    genetic_algorithm()
      .with_evaluation(&problem)
      .with_selection(MaximizeSelector::new(0.85, 12))
      .with_crossover(SinglePointCrossBreeder::new())
      .with_mutation(RandomValueMutator::new(0.2, false, true))
      .with_reinsertion(ElitistReinserter::new(&problem, false, 0.85))
      .with_initial_population(initial_population)
      .build(),
  )
  .until(GenerationLimit::new(20))
  .build();

  'sim: loop {
    
        let result = simulation.step();

        match result {
            Ok(SimResult::Intermediate(step)) => {
                let evaluated_population = step.result.evaluated_population;
                let best_solution = step.result.best_solution;
                println!(
                    "step: generation: {}, average_fitness: {}, \
                     best fitness: {}, duration: {}, processing_time: {}",
                    step.iteration,
                    evaluated_population.average_fitness(),
                    best_solution.solution.fitness,
                    step.duration.fmt(),
                    step.processing_time.fmt(),
                );
                let solution = best_solution
                    .solution
                    .genome;
            },
            Ok(SimResult::Final(step, processing_time, duration, stop_reason)) => {
                let best_solution = step.result.best_solution;
                println!("{}", stop_reason);
                println!(
                    "Final result after {}: generation: {}, \
                     best solution with fitness {} found in generation {}, processing_time: {}",
                    duration.fmt(),
                    step.iteration,
                    best_solution.solution.fitness,
                    best_solution.generation,
                    processing_time.fmt(),
                );
                break 'sim;
            },
            Err(error) => {
                println!("{}", error);
                break 'sim;
            },
        }
    }

  Ok(())

}