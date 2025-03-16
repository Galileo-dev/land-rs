#![allow(non_snake_case)]
use bon::Builder;
use clarabel::algebra::*;
use clarabel::solver::*;
use thiserror::Error;

/// 3D vector
type Vector3<T> = [T; 3];

/// Error codes returnable from incorrect trajectory inputs.
#[derive(Error, Debug)]
pub enum APDGError {
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
    settings: APDGSettings<T>,
    /// Computed solution (state vectors over time).
    pub solution: Option<Vec<T>>,
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
    pub fn new(settings: APDGSettings<T>) -> Result<Self, APDGError> {
        // Sanity check for landing site
        _new_apdg(settings)
    }

    /// Generate a trajectory to the landing site.
    pub fn solve(&mut self, settings: APDGSettings<T>) {
        // bomb if landing site is unreachable

        // bomb if insufficient fuel

        _solve(&settings);
    }
}

fn _new_apdg<T: FloatT>(settings: APDGSettings<T>) -> Result<APDGTrajectory<T>, APDGError> {
    Ok(APDGTrajectory {
        settings,
        solution: None,
        iteration_count: None,
        convergence_error: None,
    })
}

fn _solve<T: FloatT>(settings: &APDGSettings<T>) {}
