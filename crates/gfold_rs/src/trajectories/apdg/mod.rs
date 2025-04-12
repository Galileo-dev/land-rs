#![allow(non_snake_case)]
use bon::Builder;
use clarabel::algebra::*;
use clarabel::solver::traits::Solution;
use clarabel::solver::*;
use good_lp::solvers::clarabel::ClarabelProblem;
use models::{AlgorithmParams, SimulationParams};
use thiserror::Error;

use super::Trajectory;

mod guess;
mod models;
mod sucessive;

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

    // inherit error from guess
    #[error("Error in guess: {0}")]
    GuessError(#[from] guess::Error),
}

/// Required settings for a trajectory to be generated.
#[derive(Builder, Debug, Clone)]
pub struct Settings {
    #[builder(default = SimulationParams::builder().build())]
    simulation_settings: SimulationParams,
    #[builder(default = AlgorithmParams::builder().build())]
    solver_settings: AlgorithmParams,
}

#[derive(Debug, Clone, Default)]
pub struct APDGProblemSolver {}

impl APDGProblemSolver {
    /// Generate a trajectory to the landing site.
    pub fn solve(&mut self, settings: &Settings) -> Result<(), Error> {
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

const fn _new_apdg(settings: Settings) -> Result<APDGProblemSolver, Error> {
    Ok(APDGProblemSolver {})
}

fn _solve(settings: &Settings) -> Result<Vec<Trajectory>, Error> {
    // print settings
    println!("Settings: {settings:?}");

    // We start with the intitial convexification step
    let mut initial_guess = guess::problem::APDGProblem::<ClarabelProblem>::new(
        &settings.simulation_settings,
        &settings.solver_settings,
    );

    Ok(initial_guess.solve()?)
}
