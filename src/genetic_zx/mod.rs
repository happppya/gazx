
mod bar_styles;
mod fitness;
mod genetic_util;
mod output;
mod selection;

pub mod algorithm {
  pub use super::genetic_util::build_population;
  pub use super::genetic_util::extract_population;
  pub use super::genetic_util::mutate_population;
  pub use super::genetic_util::mutate_and_extract;
  pub use super::fitness::set_fitness_values;
  pub use super::output::benchmark;
  pub use super::output::print_population;
  pub use super::selection::repopulate;
  pub use super::selection::worst_individuals_iter;
}

pub mod models;