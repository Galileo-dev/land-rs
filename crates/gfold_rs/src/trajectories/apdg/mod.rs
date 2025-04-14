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
    /// Thrust magnitude [N]
    gamma: f64,
}

/// A complete APDG solution
#[derive(Debug, Clone, Builder)]
pub struct APDGSolution {
    /// An array of optimal variables at each time step
    #[builder(getter)]
    steps: Vec<APDGSolutionTimeStep>,

    /// The time between each time step [s]
    #[builder(getter)]
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
    pub fn solve(&mut self, settings: &Settings) -> Result<APDGSolution, Error> {
        // bomb if landing site is unreachable

        // bomb if insufficient fuel

        _solve(&settings)
    }
}

fn _solve(settings: &Settings) -> Result<APDGSolution, Error> {
    println!("Settings: {settings:?}");

    // --- Step 1: Initial Guess (Problem 4) ---
    println!("Solving Initial Guess Problem...");
    let initial_problem = guess::problem::APDGProblem::new(
        settings.simulation_settings.clone(),
        settings.solver_settings.clone(),
    );
    let mut current_solution = initial_problem.solve()?;
    println!("Initial Guess Solved.");

    let n_sc = settings.solver_settings.n_sc;

    // Store convergence history
    let mut pos_diff_log_history: Vec<f64> = Vec::with_capacity(n_sc);
    let mut vel_diff_log_history: Vec<f64> = Vec::with_capacity(n_sc);
    let mut thrust_diff_log_history: Vec<f64> = Vec::with_capacity(n_sc);
    const LOG_EPSILON: f64 = 1e-10; // Prevent a log10(0) error

    for i in 0..n_sc {
        println!("Starting Iteration {}...", i + 1);

        // Use current solution as the previous trajectory
        let prev_trajectory: APDGSolution = current_solution;

        // Setup and solve the successive problem
        let successive_problem = sucessive::problem::APDGProblem::new(
            settings.simulation_settings.clone(),
            settings.solver_settings.clone(),
            prev_trajectory.clone(),
        );

        match successive_problem.solve() {
            Ok(new_solution) => {
                println!("Iteration {} Solved.", i + 1);

                // Check for convergence
                let solution_differences =
                    calculate_solution_differences(&prev_trajectory, &new_solution);
                println!(
                    "Iteration {}: Max Absolute Differences - Pos: {:.6e}, Vel: {:.6e}, Mass: {:.6e}, Thrust: {:.6e}, Combined Relative: {:.6e}",
                    i + 1,
                    solution_differences.pos,
                    solution_differences.vel,
                    solution_differences.mass,
                    solution_differences.thrust,
                    solution_differences.combined_relative
                );

                // Store log10 of differences for plotting later
                // LOG_EPSILON is used to prevent a log10(0) error
                pos_diff_log_history.push((solution_differences.pos + LOG_EPSILON).log10());
                vel_diff_log_history.push((solution_differences.vel + LOG_EPSILON).log10());
                thrust_diff_log_history.push((solution_differences.thrust + LOG_EPSILON).log10());

                // Promote new solution to current solution and iterate until convergence
                current_solution = new_solution;

                if solution_differences.combined_relative < settings.solver_settings.sc_tolerance {
                    println!(
                        "Converged after {} iterations (Tolerance: {:.1e}).",
                        i + 1,
                        settings.solver_settings.sc_tolerance
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
    println!("Iteration | Pos Diff (log10) | Vel Diff (log10) | Thrust Diff (log10)");
    println!("----------|------------------|------------------|---------------------");
    for i in 0..pos_diff_log_history.len() {
        println!(
            "{:>9} | {:>16.6e} | {:>16.6e} | {:>19.6e}",
            i + 1,
            pos_diff_log_history[i],
            vel_diff_log_history[i],
            thrust_diff_log_history[i]
        );
    }

    Ok(current_solution)
}

/// Holds the maximum absolute differences between two solutions across all time steps
#[derive(Debug, Clone, Copy)]
struct SolutionDifferences {
    pos: f64,               // max_k ||r_k^(i) - r_k^(i-1)||
    vel: f64,               // max_k ||v_k^(i) - v_k^(i-1)||
    mass: f64,              // max_k |m_k^(i) - m_k^(i-1)|
    thrust: f64,            // max_k ||t_k^(i) - t_k^(i-1)||
    combined_relative: f64, // Just a combination of the above
}

/// Calculates the maximum absolute differences between two trajectories,
/// and the combined relative difference for convergence checks.
fn calculate_solution_differences(sol1: &APDGSolution, sol2: &APDGSolution) -> SolutionDifferences {
    let steps1 = &sol1.steps;
    let steps2 = &sol2.steps;
    let n = steps1.len();
    let epsilon = 1e-9; // avoid division by zero

    let mut max_pos_diff: f64 = 0.0;
    let mut max_vel_diff: f64 = 0.0;
    let mut max_mass_diff: f64 = 0.0;
    let mut max_thrust_diff: f64 = 0.0;
    let mut max_combined_relative_diff: f64 = 0.0;

    for k in 0..n {
        let s1 = &steps1[k];
        let s2 = &steps2[k];

        // Absolute differences for history tracking
        let pos_diff_norm = (s2.r - s1.r).norm();
        max_pos_diff = max_pos_diff.max(pos_diff_norm);

        let vel_diff_norm = (s2.v - s1.v).norm();
        max_vel_diff = max_vel_diff.max(vel_diff_norm);

        let mass_diff_abs = (s2.m - s1.m).abs();
        max_mass_diff = max_mass_diff.max(mass_diff_abs);

        let thrust_diff_norm = (s2.t - s1.t).norm();
        max_thrust_diff = max_thrust_diff.max(thrust_diff_norm);

        // Combined Relative difference for convergence check
        let r1_norm = s1.r.norm();
        let r_rel_diff = pos_diff_norm / (r1_norm + epsilon);
        max_combined_relative_diff = max_combined_relative_diff.max(r_rel_diff);

        let v1_norm = s1.v.norm();
        let v_rel_diff = vel_diff_norm / (v1_norm + epsilon);
        max_combined_relative_diff = max_combined_relative_diff.max(v_rel_diff);

        let m1_abs = s1.m.abs();
        let m_rel_diff = mass_diff_abs / (m1_abs + epsilon);
        max_combined_relative_diff = max_combined_relative_diff.max(m_rel_diff);

        let t1_norm = s1.t.norm();
        let t_rel_diff = thrust_diff_norm / (t1_norm + epsilon);
        max_combined_relative_diff = max_combined_relative_diff.max(t_rel_diff);
    }

    SolutionDifferences {
        pos: max_pos_diff,
        vel: max_vel_diff,
        mass: max_mass_diff,
        thrust: max_thrust_diff,
        combined_relative: max_combined_relative_diff,
    }
}
