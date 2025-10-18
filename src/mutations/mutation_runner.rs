#[derive(Debug)]
pub enum MutationType {
    LocalComplement,
    FullReduce,
    Pivot,
    FlipEdge,
}

pub const MUTATIONS_TO_RUN: &[MutationType] = &[
    MutationType::LocalComplement,
    MutationType::FullReduce,
    MutationType::Pivot,
    MutationType::FlipEdge,
];
