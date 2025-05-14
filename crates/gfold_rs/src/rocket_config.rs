use bon::Builder;
use nalgebra::Vector3;

use crate::trajectories::SimulationParams;

#[derive(Builder, Debug, Clone)]
pub struct RocketConfig {
    // Mass
    pub m_dry: f64,  // [kg]
    pub m_fuel: f64, // [kg]

    // Thrust
    pub t_min_vac: f64, // [N]
    pub t_max_vac: f64, // [N]
    pub tdot_min: f64,  // [N s⁻¹]
    pub tdot_max: f64,  // [N s⁻¹]

    // Thrust vector
    pub i_sp: f64,     // [s]
    pub a_nozzle: f64, // [m²]

    // Degrees of freedom
    pub theta_max: f64, // [deg]

    // Drag
    pub s_d: f64, // [m²]
    pub c_d: f64, // [-]
}

impl RocketConfig {
    #[inline]
    pub fn m_0(&self) -> f64 {
        self.m_dry + self.m_fuel
    }

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
