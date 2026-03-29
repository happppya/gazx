
mod bar_styles;
mod fitness;
mod genetic_util;
mod output;
mod selection;
mod crossover;
mod constants;
mod simulator;

pub mod algorithm {
  pub use super::constants::init_goal_circuit;
  pub use super::genetic_util::build_population;
  pub use super::genetic_util::extract_population;
  pub use super::genetic_util::mutate_population;
  pub use super::genetic_util::mutate_and_extract;
  pub use super::fitness::set_fitness_values;
  pub use super::selection::repopulate;
}

pub mod results {
  pub use super::fitness::get_fitness_info;
  pub use super::output::benchmark;
  pub use super::output::print_population;
  pub use super::output::Logger;
}

pub mod models;