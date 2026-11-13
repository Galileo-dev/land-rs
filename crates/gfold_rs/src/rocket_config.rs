//! Vehicle configuration for the landing problem.

use bon::Builder;
use nalgebra::Vector3;

use crate::trajectories::SimulationParams;

/// Physical vehicle parameters used to build the solver simulation settings.
#[derive(Builder, Debug, Clone)]
pub struct RocketConfig {
    // Mass
    /// Dry mass [kg]
    pub m_dry: f64,
    /// Fuel mass (propellant) [kg]
    pub m_fuel: f64,

    // Thrust
    /// Minimum vacuum thrust [N]
    pub t_min_vac: f64,
    /// Maximum vacuum thrust [N]
    pub t_max_vac: f64,
    /// Minimum thrust derivative [N s^-1]
    pub tdot_min: f64,
    /// Maximum thrust derivative [N s^-1]
    pub tdot_max: f64,

    // Thrust vector
    /// Specific impulse [s]
    pub i_sp: f64,
    /// Nozzle exit area [m^2]
    pub a_nozzle: f64,

    // Degrees of freedom
    /// Maximum tilt angle [deg]
    pub theta_max: f64,

    // Drag
    /// Reference area for drag [m^2]
    pub s_d: f64,
    /// Drag coefficient [-]
    pub c_d: f64,
}

impl RocketConfig {
    /// Initial total mass (dry + fuel) [kg]
    #[inline]
    pub fn m_0(&self) -> f64 {
        self.m_dry + self.m_fuel
    }

    /// Build the solver simulation parameters for the given initial state.
    pub fn to_sim_params(&self, r0: Vector3<f64>, v0: Vector3<f64>) -> SimulationParams {
        SimulationParams::builder()
            .r0(r0)
            .v0(v0)
            .m_dry(self.m_dry)
            .m_0(self.m_0())
            .i_sp(self.i_sp)
            .a_nozzle(self.a_nozzle)
            .t_min_vac(self.t_min_vac)
            .t_max_vac(self.t_max_vac)
            .tdot_min(self.tdot_min)
            .tdot_max(self.tdot_max)
            .theta_max(self.theta_max)
            .s_d(self.s_d)
            .c_d(self.c_d)
            .build()
    }
}
