use quizx::{circuit::Circuit, vec_graph::Graph};

use crate::mutation::mutation_runner::MutationType;

#[derive(Clone, Copy, Debug)]
pub enum ExtractStatus {
    Success,
    Fail,
    Panic,
}

#[derive(Debug)]
pub struct GraphPopulation {
    pub graphs: Vec<Graph>,
    pub circuits: Vec<Circuit>,
    pub last_mutations: Vec<MutationType>,
    pub extract_statuses: Vec<ExtractStatus>,
    pub fitness_values: Vec<i64>,
}
