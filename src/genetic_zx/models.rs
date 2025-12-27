use quizx::{circuit::Circuit, vec_graph::Graph};

use crate::mutation::mutation_runner::MutationType;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ExtractStatus {
    Success,
    Fail,
    Panic,
}

#[derive(Debug)]
pub struct PopulationComponents {
    pub graph: Vec<Graph>,
    pub circuit: Vec<Circuit>,
    pub last_mutation: Vec<MutationType>,
    pub mutation_retries : Vec<u32>,
    
    pub extract_status: Vec<ExtractStatus>,
    pub fitness: Vec<i64>,
}
