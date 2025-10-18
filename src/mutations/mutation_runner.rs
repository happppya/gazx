#[derive(Debug)]
pub enum MutationType {
    LocalComplement,
    FullReduce,
    Pivot,
    FlipEdge,
}

pub const MUTATIONS_ALL: &[MutationType] = &[
    MutationType::LocalComplement,
    MutationType::FullReduce,
    MutationType::Pivot,
    MutationType::FlipEdge,
];

pub const MUTATIONS_EDGES: &[MutationType] = &[
    MutationType::Pivot,
    MutationType::FlipEdge,
];
