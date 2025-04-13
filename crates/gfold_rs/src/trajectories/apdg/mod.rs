#![allow(non_snake_case)]
use bon::Builder;
use clarabel::algebra::*;
use clarabel::solver::traits::Solution;
use clarabel::solver::*;
use good_lp::solvers::clarabel::ClarabelProblem;
use models::{AlgorithmParams, SimulationParams};
use nalgebra::Vector3;
use thiserror::Error;

mod error;
use error::Error;
mod guess;
mod models;
mod sucessive;

#[derive(Debug, Clone, Builder)]
/// A single time step of the APDG solution
pub struct APDGSolutionTimeStep {
    /// Position [m]
    r: Vector3<f64>,
    /// Velocity [m/s]
    v: Vector3<f64>,
    /// Acceleration [m/s^2]
    a: Vector3<f64>,
    /// Mass [kg]
    m: f64,
    /// Thrust [N]
    t: Vector3<f64>,
}

/// A complete APDG solution
#[derive(Debug, Clone, Builder)]
pub struct APDGSolution {
    steps: Vec<APDGSolutionTimeStep>,
    /// The time step [s]
    dt: f64,
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

fn _solve(settings: &Settings) -> Result<Vec<APDGSolution>, Error> {
    // print settings
    println!("Settings: {settings:?}");

    // We start with the intitial convexification step
    let mut initial_guess = guess::problem::APDGProblem::new(
        settings.simulation_settings.clone(),
        settings.solver_settings.clone(),
    );

    Ok(vec![initial_guess.solve()?])
}
