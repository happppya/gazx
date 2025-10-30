use quizx::vec_graph::Graph;

use super::mutation_implementations;

#[derive(Debug)]
pub enum MutationType {
    LocalComplement,
    InverseLocalComplement,
    FullReduce,
    Pivot,
    FlipEdge,
    RemoveEdge,
    RemoveVertex,
    SplitEdge,
    AddEdge,
}

pub const MUTATIONS_ALL: &[MutationType] = &[
    MutationType::LocalComplement,
    MutationType::InverseLocalComplement,
    MutationType::FullReduce,
    MutationType::Pivot,
    MutationType::FlipEdge,
    MutationType::RemoveEdge,
    MutationType::RemoveVertex,
    MutationType::SplitEdge,
    MutationType::AddEdge,
];

pub const MUTATIONS_EDGES: &[MutationType] = &[
    MutationType::Pivot,
    MutationType::FlipEdge,
    MutationType::RemoveEdge,
    MutationType::SplitEdge,
];

#[inline]
pub fn run_mutation(graph: &mut Graph, mutation: &MutationType) {
    match mutation {
        MutationType::LocalComplement => mutation_implementations::local_complement(graph, None),
        MutationType::InverseLocalComplement => mutation_implementations::inverse_local_complement(graph, None),
        MutationType::FullReduce => mutation_implementations::full_reduce(graph),
        MutationType::Pivot => mutation_implementations::pivot(graph, None),
        MutationType::FlipEdge => mutation_implementations::flip_edge(graph, None),
        MutationType::RemoveEdge => mutation_implementations::remove_edge(graph, None),
        MutationType::RemoveVertex => mutation_implementations::remove_vertex(graph, None),
        MutationType::SplitEdge => mutation_implementations::split_edge(graph, None),
        MutationType::AddEdge => mutation_implementations::add_edge(graph, None, None),
    }
}
