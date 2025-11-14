use quizx::vec_graph::Graph;

use crate::mutation::mutation_runner::MutationType;

#[derive(Debug)]
pub struct GraphPopulation<'a> {
    pub graphs: Vec<Graph>,
    pub last_mutations : Vec<&'a MutationType>,
}