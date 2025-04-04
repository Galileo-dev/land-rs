#![allow(non_snake_case)]
use bon::Builder;
use clarabel::algebra::*;
use clarabel::solver::traits::Solution;
use clarabel::solver::*;
use thiserror::Error;

use super::Trajectory;

mod variables;
mod models;
mod problem;


/// 3D vector
type Vector3<T> = [T; 3];

/// Error codes returnable from incorrect trajectory inputs.
#[derive(Error, Debug)]
pub enum Error {
    /// Landing site is not reachable.
    #[error("Landing site is not reachable.")]
    UnreachableLandingSite,

    /// Insufficient fuel to reach landing site.
    #[error("Insufficient fuel to reach landing site.")]
    InsufficientFuel,
}

/// Required settings for a trajectory to be generated.
#[derive(Builder, Debug, Clone)]
pub struct Settings<T = f64> {
    simulation_settings: SimulationSettings;
    solver_settings: SolverSettings;
}

/// Performs APDG trajectory optimisation using the solver.
#[derive(Debug)]
pub struct Trajectory<T = f64>
where
    T: FloatT,
{
    /// Settings for the trajectory optimisation.
    pub settings: Settings<T>,
    /// Computed solution (state vectors over time).
    pub solution: Option<Trajectory>,
    /// Number of computation iterations required for convergence.
    pub iteration_count: Option<usize>,
    /// Final convergance error norm.
    pub convergence_error: Option<T>,
}

impl<T> Trajectory<T>
where
    T: FloatT,
{
    /// Create a new APDG trajectory optimisation.
    pub fn new(settings: Settings<T>) -> Result<Self, Error> {
        // Sanity check for landing site
        _new_apdg(settings)
    }

    /// Generate a trajectory to the landing site.
    pub fn solve(&mut self, settings: Settings<T>) -> Result<(), Error> {
        // bomb if landing site is unreachable

        // bomb if insufficient fuel

        match _solve(&settings) {
            Ok(trajectory) => {
                // Store the number of iterations

                // Store the convergence error
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

const fn _new_apdg<T: FloatT>(settings: Settings<T>) -> Result<APDGTrajectory<T>, Error> {
    Ok(APDGTrajectory {
        settings,
        solution: None,
        iteration_count: None,
        convergence_error: None,
    })
}

fn _solve<T: FloatT>(settings: &Settings<T>) -> Result<DefaultSolution<Trajectory>, Error> {
    // print settings
    println!("Settings: {settings:?}");
    // Ok(DefaultSolution::new(1, 2))
    unimplemented!()
}
