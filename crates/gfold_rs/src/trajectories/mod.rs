//! Native Rust implementation of the APDG solver.

#[allow(clippy::module_inception)]
mod apdg;
mod convergence;

pub use apdg::models::{AlgorithmParams, SimulationParams};
pub use apdg::{APDGProblemSolver, APDGSolution, APDGSolutionTimeStep, Settings};
pub use convergence::ConvergenceHistory;
