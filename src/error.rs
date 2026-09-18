use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid configuration: {0}")]
    InvalidConfig(&'static str),

    #[error("Reed-Solomon operation failed: {0}")]
    ReedSolomon(#[from] reed_solomon_erasure::Error),
}
