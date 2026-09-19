use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid configuration: {0}")]
    InvalidConfig(&'static str),
    #[error("Reed-Solomon operation failed: {0}")]
    ReedSolomon(#[from] reed_solomon_erasure::Error),
    #[error("invalid group number in shred")]
    InvalidGroupIndex,
    #[error("invalid shred index")]
    InvalidShredIndex,
    #[error("invalid shred size")]
    InvalidShredSize,
    #[error("conflicting shred {index} in group {group_index}")]
    ConflictingShred { group_index: usize, index: usize },
    #[error("missing data shred {index} in group {group_index}")]
    MissingDataShred { group_index: usize, index: usize },
}
