use good_lp::{ResolutionError, SolverModel};
use thiserror::Error;

/// Error codes returnable from APDG solver.
#[derive(Error, Debug)]
pub enum Error {
    /// Solver error.
    #[error("Solver error: {0}")]
    SolverError(String),

    /// SC loop did not converge.
    #[error("SC loop did not converge after {0} iterations")]
    SCNotConverged(usize),

    /// SC loop reached maximum iterations.
    #[error("SC loop reached maximum iterations ({0})")]
    SCMaxIterations(usize),

    /// Error in SC iteration {0}: {1}
    #[error("Error in SC iteration {0}: {1}")]
    SCError(usize, Box<Error>),

    /// Numeric error.
    #[error("Numeric error: {0}")]
    NumericError(String),
}
