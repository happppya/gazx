use quizx::{circuit::Circuit, vec_graph::Graph};

use crate::mutation::mutation_runner::MutationType;

#[derive(Debug)]
pub enum ExtractStatus {
    Success,
    Fail,
    Panic,
}

#[derive(Debug)]
pub struct GraphPopulation<'a> {
    pub graphs: Vec<Graph>,
    pub circuits: Vec<Circuit>,

    pub last_mutations: Vec<&'a MutationType>,
    pub extract_statuses: Vec<&'a ExtractStatus>,
    pub fitness_values: Vec<i64>,
}
