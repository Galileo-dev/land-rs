#![allow(non_snake_case)]
use bon::Builder;
use clarabel::algebra::*;
use clarabel::solver::traits::Solution;
use clarabel::solver::*;
use thiserror::Error;

use super::Trajectory;

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
pub struct APDGSettings<T: FloatT> {
    /// Landing site latitude and longitude.
    #[builder(default = [T::zero(), T::zero(), T::zero()])]
    pub landing_site: Vector3<T>,

    /// Num of discretised time steps
    #[builder(default = T::from(10.0).unwrap_or_else(|| T::zero()))]
    pub N: T,

    /// time step duration (seconds)
    #[builder(default = T::from(2.0).unwrap_or_else(|| T::zero()))]
    pub dt: T,

    /// Initial mass [kg]
    #[builder(default = T::from(2.0).unwrap_or_else(|| T::zero()))]
    pub m0: T,

    /// Gravity vector [m/s^2]
    #[builder(default = [T::zero(), T::zero(), T::from(-9.81).unwrap_or_else(|| T::zero())])]
    pub g: Vector3<T>,

    /// Minimum thrust [N].
    #[builder(default = T::from(10.0).unwrap_or_else(|| T::zero()))]
    pub T_min: T,

    /// Maximum thrust [N].
    #[builder(default = T::from(1000.0).unwrap_or_else(|| T::zero()))]
    pub T_max: T,

    /// Maximum gimbal angle (radians).
    #[builder(default = T::from(15.0_f64.to_radians()).unwrap_or_else(|| T::zero()))]
    pub theta_T_max: T,

    /// Maximum glide slope angle (radians).
    #[builder(default = T::from(5.0_f64.to_radians()).unwrap_or_else(|| T::zero()))]
    pub theta_gs: T,

    /// Specific impulse [s].
    #[builder(default = T::from(300.0).unwrap_or_else(|| T::zero()))]
    pub I_sp: T,

    /// Maximum allowable control (used for trust region constraints).
    #[builder(default = T::from(100.0).unwrap_or_else(|| T::zero()))]
    pub max_control: T,
}

/// Performs APDG trajectory optimisation using the solver.
#[derive(Debug)]
pub struct APDGTrajectory<T = f64>
where
    T: FloatT,
{
    /// Settings for the trajectory optimisation.
    pub settings: APDGSettings<T>,
    /// Computed solution (state vectors over time).
    pub solution: Option<Trajectory>,
    /// Number of computation iterations required for convergence.
    pub iteration_count: Option<usize>,
    /// Final convergance error norm.
    pub convergence_error: Option<T>,
}

impl<T> APDGTrajectory<T>
where
    T: FloatT,
{
    /// Create a new APDG trajectory optimisation.
    pub fn new(settings: APDGSettings<T>) -> Result<Self, Error> {
        // Sanity check for landing site
        _new_apdg(settings)
    }

    /// Generate a trajectory to the landing site.
    pub fn solve(&mut self, settings: APDGSettings<T>) -> Result<(), Error> {
        // bomb if landing site is unreachable

        // bomb if insufficient fuel

        match _solve(&settings) {
            Ok(trajectory) => {
                // Store the trajectory
                self.solution = Some(trajectory.0);

                // Store the number of iterations

                // Store the convergence error
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

const fn _new_apdg<T: FloatT>(settings: APDGSettings<T>) -> Result<APDGTrajectory<T>, Error> {
    Ok(APDGTrajectory {
        settings,
        solution: None,
        iteration_count: None,
        convergence_error: None,
    })
}

fn _solve<T: FloatT>(settings: &APDGSettings<T>) -> Result<DefaultSolution<Trajectory>, Error> {
    // print settings
    println!("Settings: {settings:?}");
    Ok(DefaultSolution::new(1, 2))
}
