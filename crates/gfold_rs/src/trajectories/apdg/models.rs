use bon::{builder, Builder};

/// Simulation parameters (Table 1).
#[derive(Debug, Builder, Clone)]
pub struct SimulationParams {
    /// Ambient fluid density
    /// [kg/m^3]
    #[builder(default = 1.0)]
    pub rho: f64,

    /// Ambient pressure
    /// [Pa]
    #[builder(default = 100_000.0)]
    pub p_amb: f64,

    /// Standard gravity
    /// [m/s^2]
    #[builder(default = 9.807)]
    pub g_0: f64,

    /// Gravity vector
    /// [m/s^2]
    #[builder(default = [-9.807, 0.0, 0.0])]
    pub g_vec: [f64; 3],

    /// Dry mass of the vehicle
    /// [kg]
    #[builder(default = 10_000.0)]
    pub m_dry: f64,

    /// Initial total mass (dry mass + propellant)
    /// [kg]
    #[builder(default = 15_000.0)]
    pub m_0: f64,

    /// Initial position vector
    /// [m]
    #[builder(default = [500.0, 500.0, 0.0])]
    pub r0: [f64; 3],

    /// Final position vector
    /// [m]
    #[builder(default = [0.0, 0.0, 0.0])]
    pub rf: [f64; 3],

    /// Initial velocity vector
    /// [m/s]
    #[builder(default = [-50.0, 0.0, 50.0])]
    pub v0: [f64; 3],

    /// Final velocity vector
    /// [m/s]
    #[builder(default = [0.0, 0.0, 0.0])]
    pub vf: [f64; 3],

    /// Initial thrust guess
    /// [N]
    #[builder(default = 175_000.0)]
    pub gamma_0_vac: f64,

    /// Initial thrust direction unit vector
    #[builder(default = [1.0, 0.0, 0.0])]
    pub n_hat0: [f64; 3],

    /// Final thrust direction unit vector
    #[builder(default = [1.0, 0.0, 0.0])]
    pub n_hatf: [f64; 3],

    /// Up pointing unit vector
    #[builder(default = [1.0, 0.0, 0.0])]
    pub e_hat_up: [f64; 3],

    /// Nozzle exit area
    /// [m^2]
    #[builder(default = 0.5)]
    pub a_nozzle: f64,

    /// Specific impulse
    /// [s]
    #[builder(default = 300.0)]
    pub i_sp: f64,

    /// Minimum vacuum thrust
    /// [N]
    #[builder(default = 100_000.0)]
    pub t_min_vac: f64,

    /// Maximum vacuum thrust
    /// [N]
    #[builder(default = 250_000.0)]
    pub t_max_vac: f64,

    /// Minimum thrust derivative
    /// [N/s]
    #[builder(default = -100_000.0)]
    pub tdot_min: f64,

    /// Maximum thrust derivative
    /// [N/s]
    #[builder(default = 100_000.0)]
    pub tdot_max: f64,

    /// Maximum tilt (pitch) angle
    /// [deg]
    #[builder(default = 15.0)]
    pub theta_max: f64,

    /// Glide slope angle
    /// [deg]
    #[builder(default = 80.0)]
    pub gamma_gs: f64,

    /// Reference area for drag
    /// [m^2]
    #[builder(default = 10.0)]
    pub s_d: f64,

    /// Drag coefficient
    #[builder(default = 1.0)]
    pub c_d: f64,
}

/// Boundary Conditions and Algorithm parameters
#[derive(Debug, Builder, Clone)]
pub struct AlgorithmParams {
    /// t_f,s [s] – Initial guess for total flight time
    /// (Table 2: tf,s = 15 s)
    #[builder(default = 15.0)]
    pub tf_guess: f64,

    /// Number of discretization points in the trajectory
    #[builder(default = 30)]
    pub N: usize,

    /// Time step for the simulation
    #[builder(default = tf_guess / (N as f64))]
    pub dt: f64,

    /// Number of successive convexification iterations
    #[builder(default = 10)]
    pub n_sc: usize,

    /// Weight on final mass in the cost function
    #[builder(default = 1.0)]
    pub w_mf: f64,

    /// Weight on time penalty
    #[builder(default = 0.0001)]
    pub w_eta_dt: f64,

    /// Weight on thrust penalty
    #[builder(default = 0.0001)]
    pub w_eta_T: f64,

    /// Weight on angle-of-attack regularization
    #[builder(default = 100.0)]
    pub w_kappa_aR: f64,
}
