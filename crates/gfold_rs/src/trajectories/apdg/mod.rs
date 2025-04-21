#![allow(non_snake_case)]
use crate::trajectories::ConvergenceHistory;
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
mod sucessive;

// Settings
pub mod models;

#[derive(Debug, Clone, Builder)]
/// A single time step of the APDG solution
pub struct APDGSolutionTimeStep {
    /// Position [m]
    pub r: Vector3<f64>,
    /// Velocity [m/s]
    pub v: Vector3<f64>,
    /// Acceleration [m/s^2]
    pub a: Vector3<f64>,
    /// Mass [kg]
    pub m: f64,
    /// Thrust [N]
    pub t: Vector3<f64>,
    /// Thrust magnitude [N]
    pub gamma: f64,
    /// Acceleration relaxation term [m/s^2]
    pub aR: Vector3<f64>,
}

/// A complete APDG solution
#[derive(Debug, Clone, Builder)]
pub struct APDGSolution {
    /// An array of optimal variables at each time step
    steps: Vec<APDGSolutionTimeStep>,

    /// The time between each time step [s]
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

impl Settings {
    /// Returns a reference to the simulation parameters.
    pub fn simulation_settings(&self) -> &SimulationParams {
        &self.simulation_settings
    }

    /// Returns a reference to the algorithm parameters.
    pub fn solver_settings(&self) -> &AlgorithmParams {
        &self.solver_settings
    }
}

impl APDGSolution {
    /// The number of time steps in the solution
    pub fn num_steps(&self) -> usize {
        self.steps.len()
    }

    /// The time between each time step [s]
    pub fn dt(&self) -> f64 {
        self.dt
    }

    /// Individual steps of the solution
    pub fn steps(&self) -> &[APDGSolutionTimeStep] {
        &self.steps
    }
}

#[derive(Debug, Clone, Default)]
pub struct APDGProblemSolver {}

impl APDGProblemSolver {
    /// Generate a trajectory and convergence history.
    pub fn solve(
        &mut self,
        settings: &Settings,
    ) -> Result<(APDGSolution, ConvergenceHistory), Error> {
        _solve(settings)
    }
}

fn _solve(settings: &Settings) -> Result<(APDGSolution, ConvergenceHistory), Error> {
    println!("Settings: {settings:?}");

    // --- Step 1: Initial Guess (Problem 4) ---
    println!("Solving Initial Guess Problem...");
    let initial_problem = guess::problem::APDGProblem::new(
        settings.simulation_settings().clone(),
        settings.solver_settings().clone(),
    );
    let mut current_solution = initial_problem.solve()?;
    println!("Initial Guess Solved.");

    let n_sc = settings.solver_settings().n_sc;

    // Store convergence history
    let mut pos_log: Vec<f64> = Vec::with_capacity(n_sc);
    let mut vel_log: Vec<f64> = Vec::with_capacity(n_sc);
    let mut thrust_log: Vec<f64> = Vec::with_capacity(n_sc);
    let mut aR_log: Vec<f64> = Vec::with_capacity(n_sc);
    const LOG_EPSILON: f64 = 1e-10; // Prevent a log10(0) error
    for i in 0..n_sc {
        println!("Starting Iteration {}...", i + 1);

        // Use current solution as the previous trajectory
        let prev_trajectory: APDGSolution = current_solution;

        // Setup and solve the successive problem
        let successive_problem = sucessive::problem::APDGProblem::new(
            settings.simulation_settings().clone(),
            settings.solver_settings().clone(),
            prev_trajectory.clone(),
        );

        match successive_problem.solve() {
            Ok(new_solution) => {
                println!("Iteration {} Solved.", i + 1);

                // Check for convergence
                let solution_differences =
                    calculate_solution_differences(&prev_trajectory, &new_solution);
                println!(
                    "Iteration {}: Max Absolute Differences - Pos: {:.6e}, Vel: {:.6e}, Mass: {:.6e}, Thrust: {:.6e}, Max Relative: {:.6e}",
                    i + 1,
                    solution_differences.abs_pos,
                    solution_differences.abs_vel,
                    solution_differences.abs_mass,
                    solution_differences.abs_thrust,
                    solution_differences.max_relative
                );

                // Store log10 of differences for plotting later
                pos_log.push((solution_differences.abs_pos + LOG_EPSILON).log10());
                vel_log.push((solution_differences.abs_vel + LOG_EPSILON).log10());
                thrust_log.push((solution_differences.abs_thrust + LOG_EPSILON).log10());
                aR_log.push((solution_differences.abs_aR + LOG_EPSILON).log10());
                // Promote new solution to current solution and iterate until convergence
                current_solution = new_solution;

                if solution_differences.max_relative < settings.solver_settings().sc_tolerance {
                    println!(
                        "Converged after {} iterations (Tolerance: {:.1e}).",
                        i + 1,
                        settings.solver_settings().sc_tolerance
                    );
                    break;
                }

                if i == n_sc - 1 {
                    println!("Reached maximum iterations ({}).", n_sc);
                }
            }
            Err(e) => {
                return Err(Error::SCError(i + 1, Box::new(e)));
            }
        }
    }

    println!("Successive Convexification Finished.");
    println!("\nConvergence History (Log10 Max Differences):");
    println!(
        "Iteration | Pos Diff (log10) | Vel Diff (log10) | Thrust Diff (log10) | aR Diff (log10)"
    );
    println!(
        "----------|------------------|------------------|---------------------|------------------|"
    );
    for i in 0..pos_log.len() {
        println!(
            "{:>9} | {:>16.6e} | {:>16.6e} | {:>19.6e} | {:>16.6e}",
            i + 1,
            pos_log[i],
            vel_log[i],
            thrust_log[i],
            aR_log[i]
        );
    }

    Ok((
        current_solution,
        ConvergenceHistory {
            pos: pos_log,
            vel: vel_log,
            thrust: thrust_log,
            aR: aR_log,
        },
    ))
}

/// Holds the maximum absolute differences between two solutions across all time steps
#[derive(Debug, Clone, Copy)]
struct SolutionDifferences {
    abs_pos: f64,      // max_k ||r2[k] - r1[k]|| for all k
    abs_vel: f64,      // max_k ||v2[k] - v1[k]|| for all k
    abs_mass: f64,     // max_k |m2[k] - m1[k]| for all k
    abs_thrust: f64,   // max_k ||t2[k] - t1[k]|| for all k
    abs_aR: f64,       // max_k ||aR2[k] - aR1[k]|| for all k
    rel_pos: f64,      // max_k ||r2[k] - r1[k]|| / (||r1[k]|| + epsilon) for all k
    rel_vel: f64,      // max_k ||v2[k] - v1[k]|| / (||v1[k]|| + epsilon) for all k
    rel_mass: f64,     // max_k |m2[k] - m1[k]| / (|m1[k]| + epsilon) for all k
    rel_thrust: f64,   // max_k ||t2[k] - t1[k]|| / (||t1[k]|| + epsilon) for all k
    rel_aR: f64,       // max_k ||aR2[k] - aR1[k]|| / (||aR1[k]|| + epsilon) for all k
    max_relative: f64, // Maximum of all relative (rel_pos, rel_vel, rel_mass, rel_thrust, rel_aR).
}

/// Calculates the maximum absolute differences between two trajectories,
/// and the combined relative difference for convergence checks.
fn calculate_solution_differences(sol1: &APDGSolution, sol2: &APDGSolution) -> SolutionDifferences {
    let steps1 = &sol1.steps;
    let steps2 = &sol2.steps;

    assert_eq!(
        steps1.len(),
        steps2.len(),
        "Cannot compare solutions with different numbers of steps."
    );

    let epsilon = 1e-9; // avoid division by zero

    let mut max_abs_pos = 0.0_f64;
    let mut max_abs_vel = 0.0_f64;
    let mut max_abs_mass = 0.0_f64;
    let mut max_abs_thrust = 0.0_f64;
    let mut max_abs_aR = 0.0_f64;

    let mut max_rel_pos = 0.0_f64;
    let mut max_rel_vel = 0.0_f64;
    let mut max_rel_mass = 0.0_f64;
    let mut max_rel_thrust = 0.0_f64;
    let mut max_rel_aR = 0.0_f64;

    for (s1, s2) in steps1.iter().zip(steps2.iter()) {
        // Absolute differences for history tracking
        let abs_pos_diff = (s2.r - s1.r).norm();
        max_abs_pos = max_abs_pos.max(abs_pos_diff);

        let abs_vel_diff = (s2.v - s1.v).norm();
        max_abs_vel = max_abs_vel.max(abs_vel_diff);

        let abs_mass_diff = (s2.m - s1.m).abs();
        max_abs_mass = max_abs_mass.max(abs_mass_diff);

        let abs_thrust_diff = (s2.t - s1.t).norm();
        max_abs_thrust = max_abs_thrust.max(abs_thrust_diff);

        let r1_norm = s1.r.norm();
        let v1_norm = s1.v.norm();
        let m1_abs = s1.m.abs();
        let t1_norm = s1.t.norm();
        let aR1_norm = s1.aR.norm();

        let rel_pos_diff = abs_pos_diff / (r1_norm + epsilon);
        max_rel_pos = max_rel_pos.max(rel_pos_diff);

        let rel_vel_diff = abs_vel_diff / (v1_norm + epsilon);
        max_rel_vel = max_rel_vel.max(rel_vel_diff);

        let rel_mass_diff = abs_mass_diff / (m1_abs + epsilon);
        max_rel_mass = max_rel_mass.max(rel_mass_diff);

        let rel_thrust_diff = abs_thrust_diff / (t1_norm + epsilon);
        max_rel_thrust = max_rel_thrust.max(rel_thrust_diff);

        let abs_aR_diff = (s2.aR - s1.aR).norm();
        max_abs_aR = max_abs_aR.max(abs_aR_diff);

        let rel_aR_diff = abs_aR_diff / (s1.aR.norm() + epsilon);
        max_rel_aR = max_rel_aR.max(rel_aR_diff);
    }

    // Find the maximum among all the calculated maximum relative differences
    let overall_max_relative = [
        max_rel_pos,
        max_rel_vel,
        max_rel_mass,
        max_rel_thrust,
        max_rel_aR,
    ]
    .into_iter()
    .fold(0.0f64, f64::max);

    SolutionDifferences {
        abs_pos: max_abs_pos,
        abs_vel: max_abs_vel,
        abs_mass: max_abs_mass,
        abs_thrust: max_abs_thrust,
        abs_aR: max_abs_aR,
        rel_pos: max_rel_pos,
        rel_vel: max_rel_vel,
        rel_mass: max_rel_mass,
        rel_thrust: max_rel_thrust,
        rel_aR: max_rel_aR,
        max_relative: overall_max_relative,
    }
}
