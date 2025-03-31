use good_lp::{Expression, Variable};

// -------------------------------------------------------
// Problem 4: Rocket Landing Optimal Control Problem
// First Solve Convexification Step
//
// Problem formulation:
//      min  final mass => -m[N-1] in the linear term
//
// Penalise a large aR by adding w_aR * sum( kappa_{a,R}[k] )
//
// This makes our cost function:
//      min  - w_mf * m[N-1] + kappa_aR * sum( kappa_{a,R}[k] ).
//
// s.t.
//    Boundary Conditions, Dynamics, SOC Constraints
// -------------------------------------------------

struct DecisionVariables {
    /// Position
    /// [m]
    r: Vec<Variable>,
    /// Velocity
    /// [m/s]
    v: Vec<Variable>,
    /// Acceleration
    /// [m/s^2]
    a: Vec<Variable>,
    /// Mass
    /// [kg]
    m: Vec<Variable>,
    /// Thrust
    /// [N]
    t: Vec<Variable>,
    /// Angle of attack
    /// [rad]
    gamma: Vec<Variable>,
    /// Angle of attack rate
    /// [rad/s]
    aR: Vec<Variable>,
    /// Relaxation
    /// []
    kappa_aR: Vec<Variable>,
    /// Glide slope slack
    /// []
    z: Vec<Variable>,
}

fn maximise_final_mass(m_f: Variable) -> Expression {
    mass_cost(energy)
}

fn mass_cost(energy: Variable) -> Expression {
    let price = fetch_energy_price(energy);
    energy * price
}
