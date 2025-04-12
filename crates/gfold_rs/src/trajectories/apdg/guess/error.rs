use thiserror::Error;

/// Error codes returnable from incorrect guess inputs.
#[derive(Error, Debug)]
pub enum Error {
    /// Guess is not feasible.
    #[error("Guess is not feasible.")]
    InfeasibleGuess,
}
