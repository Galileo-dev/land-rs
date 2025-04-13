use good_lp::{ResolutionError, SolverModel};
use thiserror::Error;

/// Error codes returnable from APDG solver.
#[derive(Error, Debug)]
pub enum Error {
    /// Solver error.
    #[error("Solver error: {0}")]
    SolverError(String),
}
