//! Main crate error module

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic: {0}")]
    Generic(String),

    #[error(transparent)]
    Apdg(#[from] crate::trajectories::Error),
}
