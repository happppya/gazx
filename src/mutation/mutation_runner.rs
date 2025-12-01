use quizx::vec_graph::Graph;

use super::mutations;

#[derive(Clone, Copy, Debug)]
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
    SwitchEdge,
    AddPhaseGadget,

    NoMutation,

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
    MutationType::SwitchEdge,
    MutationType::AddPhaseGadget,
];

pub const MUTATIONS_EDGES: &[MutationType] = &[
    MutationType::Pivot,
    MutationType::FlipEdge,
    MutationType::RemoveEdge,
    MutationType::SplitEdge,
    MutationType::SwitchEdge,
];

#[inline]
pub fn run_mutation(graph: &mut Graph, mutation: &MutationType) {
    match mutation {
        MutationType::LocalComplement => mutations::local_complement(graph, None),
        MutationType::InverseLocalComplement => mutations::inverse_local_complement(graph, None),
        MutationType::FullReduce => mutations::full_reduce(graph),
        MutationType::Pivot => mutations::pivot(graph, None),
        MutationType::FlipEdge => mutations::flip_edge(graph, None),
        MutationType::RemoveEdge => mutations::remove_edge(graph, None),
        MutationType::RemoveVertex => mutations::remove_vertex(graph, None),
        MutationType::SplitEdge => mutations::split_edge(graph, None),
        MutationType::AddEdge => mutations::add_edge(graph, None, None),
        MutationType::SwitchEdge => mutations::switch_edge(graph, None, None, None),
        MutationType::AddPhaseGadget => mutations::add_phase_gadget(graph, None),
        MutationType::NoMutation => {},
    }
}
