#![allow(non_snake_case)]
use bon::Builder;
use clarabel::algebra::*;
use clarabel::solver::traits::Solution;
use clarabel::solver::*;
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
}

/// Required settings for a trajectory to be generated.
#[derive(Builder, Debug, Clone)]
pub struct Settings {
    simulation_settings: SimulationParams,
    solver_settings: AlgorithmParams,
}

pub struct APDGTrajectory {
    pub settings: Settings,
}

impl APDGTrajectory {
    /// Create a new APDG trajectory optimisation.
    pub fn new(settings: Settings) -> Result<Self, Error> {
        // Sanity check for landing site
        _new_apdg(settings)
    }

    /// Generate a trajectory to the landing site.
    pub fn solve(&mut self, settings: Settings) -> Result<(), Error> {
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

const fn _new_apdg(settings: Settings) -> Result<APDGTrajectory, Error> {
    Ok(APDGTrajectory { settings })
}

fn _solve(settings: &Settings) -> Result<DefaultSolution<Trajectory>, Error> {
    // print settings
    println!("Settings: {settings:?}");
    // Ok(DefaultSolution::new(1, 2))
    unimplemented!()
}
