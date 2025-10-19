use quizx::vec_graph::Graph;

use super::mutation_implementations;

#[derive(Debug)]
pub enum MutationType {
    LocalComplement,
    FullReduce,
    Pivot,
    FlipEdge,
    RemoveEdge,
    RemoveVertex,
}

pub const MUTATIONS_ALL: &[MutationType] = &[
    MutationType::LocalComplement,
    MutationType::FullReduce,
    MutationType::Pivot,
    MutationType::FlipEdge,
    MutationType::RemoveEdge,
    MutationType::RemoveVertex,
];

pub const MUTATIONS_EDGES: &[MutationType] = &[
    MutationType::Pivot,
    MutationType::FlipEdge,
    MutationType::RemoveEdge,
];

#[inline]
pub fn run_mutation(graph: &mut Graph, mutation: &MutationType) {
    match mutation {
        MutationType::LocalComplement => mutation_implementations::local_complement(graph, None),
        MutationType::FullReduce => mutation_implementations::full_reduce(graph),
        MutationType::Pivot => mutation_implementations::pivot(graph, None),
        MutationType::FlipEdge => mutation_implementations::flip_edge(graph, None),
        MutationType::RemoveEdge => mutation_implementations::remove_edge(graph, None),
        MutationType::RemoveVertex => mutation_implementations::remove_vertex(graph, None),
    }
}
