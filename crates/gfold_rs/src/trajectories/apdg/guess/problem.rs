use good_lp::{
    clarabel, constraint, soc_constraint, variable, variables, Expression, ProblemVariables,
    SolverModel, Variable,
};
use nalgebra::Vector3;

use crate::trajectories::{
    apdg::models::{AlgorithmParams, SimulationParams},
    ThrustVector, Trajectory,
};

use super::Error;

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

pub struct APDGProblem<S: SolverModel> {
    solver: S,
}

struct DecisionVariables {
    /// Position
    /// [m]
    r: Vec<Vector3<Variable>>,
    /// Velocity
    /// [m/s]
    v: Vec<Vector3<Variable>>,
    /// Acceleration
    /// [m/s^2]
    a: Vec<Vector3<Variable>>,
    /// Mass
    /// [kg]
    m: Vec<Variable>,
    /// Thrust
    /// [N]
    t: Vec<Vector3<Variable>>,
    /// Angle of attack
    /// [rad]
    gamma: Vec<Variable>,
    /// Angle of attack rate
    /// [rad/s]
    aR: Vec<Vector3<Variable>>,
    /// Relaxation
    /// []
    kappa_aR: Vec<Variable>,
    /// Glide slope slack
    /// []
    z: Vec<Variable>,
}

impl DecisionVariables {
    fn fixed_vector(vars: &mut good_lp::ProblemVariables) -> Vector3<Variable> {
        Vector3::new(
            vars.add_variable(),
            vars.add_variable(),
            vars.add_variable(),
        )
    }

    fn new(vars: &mut good_lp::ProblemVariables, N: usize) -> Self {
        DecisionVariables {
            r: (0..N).map(|_| Self::fixed_vector(vars)).collect(),
            v: (0..N).map(|_| Self::fixed_vector(vars)).collect(),
            a: (0..N).map(|_| Self::fixed_vector(vars)).collect(),
            m: (0..N).map(|_| vars.add_variable()).collect(),
            t: (0..N).map(|_| Self::fixed_vector(vars)).collect(),
            gamma: (0..N).map(|_| vars.add_variable()).collect(),
            aR: (0..N).map(|_| Self::fixed_vector(vars)).collect(),
            kappa_aR: (0..N).map(|_| vars.add_variable()).collect(),
            z: (0..N).map(|_| vars.add_variable()).collect(),
        }
    }
}

impl<S: SolverModel> APDGProblem<S> {
    pub fn new(
        sim_params: &SimulationParams,
        algo_params: &AlgorithmParams,
    ) -> APDGProblem<impl SolverModel> {
        let (decision_vars, model) = setup_problem(sim_params, algo_params);
        let mut problem = APDGProblem { solver: model };

        // Add initial constraints
        add_initial_constraints(&mut problem.solver, &decision_vars, sim_params);

        // Pre-compute values for Problem 4
        let (mu, s) = pre_compute(sim_params, algo_params);

        // Add final constraints
        add_final_constraints(&mut problem.solver, &decision_vars, sim_params, algo_params);

        // Add dynamics constraints
        add_dynamics_constraints(
            &mut problem.solver,
            &decision_vars,
            sim_params,
            algo_params,
            mu,
            s,
        );

        // Add state constraints
        add_state_constraints(&mut problem.solver, &decision_vars, sim_params, algo_params);

        // Add slack constraints
        add_slack_constraints(&mut problem.solver, &decision_vars, sim_params, algo_params);

        problem
    }

    /// Solve the problem
    pub fn solve(mut self) -> Result<Vec<Trajectory>, Error> {
        // Run the solver
        self.solver.solve();

        Ok(vec![])
    }
}

/// Function to set up the problem with decision variables and objective function
fn setup_problem(
    params: &SimulationParams,
    algo: &AlgorithmParams,
) -> (DecisionVariables, impl SolverModel) {
    let N = algo.N;

    let mut vars = variables!();

    let mut decision_variables = DecisionVariables::new(&mut vars, N);

    let mut objective = Expression::default();

    objective += -algo.w_mf * decision_variables.m[N - 1];
    for k in 0..N {
        // Add “+ w_k_a_R * kappa[k]” to objective
        objective += algo.w_kappa_aR * decision_variables.kappa_aR[k];
    }

    let mut model = vars.minimise(objective).using(clarabel);

    model.settings().verbose(true).tol_feas(1e-8);

    (decision_variables, model)
}

/// Add initial condition constraints to the problem
fn add_initial_constraints(
    model: &mut impl SolverModel,
    vars: &DecisionVariables,
    params: &SimulationParams,
) {
    // Initial Mass m[0] = m_0
    model.add_constraint(constraint!(vars.m[0] == params.m_0));

    // Initial Position r[0] = r_0
    for (var, &r0) in vars.r[0].iter().zip(params.r0.iter()) {
        model.add_constraint(constraint!(*var == r0));
    }

    // Initial Velocity v[0] = v_0

    for (var, &v0) in vars.v[0].iter().zip(params.v0.iter()) {
        model.add_constraint(constraint!(*var == v0));
    }

    // T[0] = Gamma_0 * n_hat0
    // n_hat0 is the initial normal vector
    for (i, var) in vars.t[0].iter().enumerate() {
        model.add_constraint(constraint!(*var == params.gamma_0_vac * params.n_hat0[i]));
    }

    // Gamma[0] = Gamma_0_vac
    model.add_constraint(constraint!(vars.gamma[0] == params.gamma_0_vac));
}

/// Add final condition constraints to the problem
fn add_final_constraints(
    model: &mut impl SolverModel,
    vars: &DecisionVariables,
    params: &SimulationParams,
    algo: &AlgorithmParams,
) {
    let k_end = algo.N - 1;

    // Final position r[N-1] = rf
    for (var, &rf_val) in vars.r[k_end].iter().zip(params.rf.iter()) {
        model.add_constraint(constraint!(*var == rf_val));
    }

    // Final velocity v[N-1] = vf
    for (var, &vf_val) in vars.v[k_end].iter().zip(params.vf.iter()) {
        model.add_constraint(constraint!(*var == vf_val));
    }

    // Final thrust direction
    // T[N-1] = Gamma[N-1] * n_hatf
    for (i, tf) in vars.t[k_end].iter().enumerate() {
        model.add_constraint(constraint!(*tf == vars.gamma[k_end] * params.n_hatf[i]));
    }
}

fn pre_compute(params: &SimulationParams, settings: &AlgorithmParams) -> (Vec<f64>, Vec<f64>) {
    // Pre-computed values for Problem 4
    // mu[k] = ((k_n - k)/k_n)*m_0 + (k/k_n)*m_dry
    let mu = |k: usize| {
        let kn = settings.N as f64;
        let k = k as f64;
        ((kn - k) / kn) * params.m_0 + (k / kn) * params.m_dry
    };

    // s[k] = (k_n - k/k_n) ||v_0|| + (k/k_n) ||v_f||
    let s = |k: usize| {
        let kn = settings.N as f64;
        let k = k as f64;
        let v0_norm = (params.v0.iter().map(|&x| x * x).sum::<f64>()).sqrt();
        let v_f_norm = (params.vf.iter().map(|&x| x * x).sum::<f64>()).sqrt();
        ((kn - k) / kn) * v0_norm + (k / kn) * v_f_norm
    };

    (
        (0..settings.N).map(mu).collect(),
        (0..settings.N).map(s).collect(),
    )
}

/// Add the discretized dynamics contraints
fn add_dynamics_constraints(
    model: &mut impl SolverModel,
    vars: &DecisionVariables,
    params: &SimulationParams,
    settings: &AlgorithmParams,
    mu: Vec<f64>,
    s: Vec<f64>,
) {
    let N = settings.N;

    // Some relationships
    let alpha = 1.0 / (params.i_sp * params.g_0); //  relates thrust to mass flow rate
    let m_dot_bp = (params.p_amb * params.a_nozzle) / (params.i_sp * params.g_0); // Mass flow rate

    for k in 0..N - 1 {
        // Mass dynamics
        // m[k+1] = m[k] - [alpha/2 * (gamma[k] + gamma[k+1]) + m_dot_bp] * dt
        model.add_constraint(constraint!(
            vars.m[k + 1]
                == vars.m[k]
                    - (alpha / 2.0 * (vars.gamma[k] + vars.gamma[k + 1]) * settings.dt)
                    - (m_dot_bp * settings.dt)
        ));

        // Position dynamics
        // r[k+1] = r[k] + v[k] * dt + 1/3 * (a[k] + 1/2*a[k+1]) * dt^2
        for i in 0..3 {
            model.add_constraint(constraint!(
                vars.r[k + 1][i]
                    == vars.r[k][i]
                        + vars.v[k][i] * settings.dt
                        + (1.0 / 3.0)
                            * (vars.a[k][i] + 0.5 * vars.a[k + 1][i])
                            * settings.dt.powi(2)
            ));
        }

        // Velocity dynamics
        // v[k+1] = v[k] + 1/2 * (a[k] + a[k+1]) * dt
        for i in 0..3 {
            model.add_constraint(constraint!(
                vars.v[k + 1][i]
                    == vars.v[k][i] + 0.5 * (vars.a[k][i] + vars.a[k + 1][i]) * settings.dt
            ));
        }

        // Acceleration dynamics
        //a[k] = 1/mu[k] * (T[k] - 1/2 * rho * S_D * C_D * s[k] * v[k]) + a_R[k] + g
        for i in 0..3 {
            model.add_constraint(constraint!(
                vars.a[k][i]
                    == 1.0 / mu[k]
                        * (vars.t[k][i]
                            - 0.5 * params.rho * params.s_d * params.c_d * s[k] * vars.v[k][i])
                        + vars.aR[k][i]
                        + params.g_vec[i]
            ));
        }
    }
}

/// Add the state constraints
fn add_state_constraints(
    model: &mut impl SolverModel,
    vars: &DecisionVariables,
    params: &SimulationParams,
    settings: &AlgorithmParams,
) {
    let N = settings.N;

    // Add SOC constraints
    for k in 0..N {
        // Mass lowerbound constraint
        // m[k] >= m_dry
        model.add_constraint(constraint!(vars.m[k] >= params.m_dry));
    }

    // Glide-slope constraint
    // ||r[k]|| cos(gamma_gs) <= e_u^T * r[k]
    // For Clarabel we need to use an aux variable z[k]
    // ||r[k]|| <= z[k]
    // z[k] = sec_gs * ( e_u^T * r[k] )
    let sec_gs = 1.0 / f64::to_radians(params.gamma_gs).cos();
    for k in 0..N {
        let dot_e_r = params.e_hat_up.x * vars.r[k][0]
            + params.e_hat_up.y * vars.r[k][1]
            + params.e_hat_up.z * vars.r[k][2];

        // z[k] = sec_gs * ( e_hat_up dot r[k] )
        model.add_constraint(constraint!(vars.z[k] == sec_gs * dot_e_r));

        // SoC: norm2(r[k]) <= z[k]
        model.add_constraint(soc_constraint!(
            norm2(vars.r[k][0], vars.r[k][1], vars.r[k][2]) <= vars.z[k]
        ));
    }

    // Thrust (Equation 70)
    // ||T[k]|| <= Gamma[k]
    for k in 0..N {
        model.add_constraint(soc_constraint!(
            norm2(vars.t[k][0], vars.t[k][1], vars.t[k][2]) <= vars.gamma[k]
        ));
    }

    // Max/Min thrust (Equation 71)
    // T_min <= Gamma[k] <= T_max
    for k in 0..N {
        model.add_constraint(constraint!(vars.gamma[k] >= params.t_min_vac));
        model.add_constraint(constraint!(vars.gamma[k] <= params.t_max_vac));
    }

    // Tilt constraint (Equation 72):
    // Gamma[k] * cos(theta_max) <= e^T T[k].
    // e_hat_up dot T[k] - Gamma[k]*cos(...) >= 0
    let cos_th = f64::to_radians(params.theta_max).cos();
    for k in 0..N {
        let up_dot_t = params.e_hat_up.x * vars.t[k][0]
            + params.e_hat_up.y * vars.t[k][1]
            + params.e_hat_up.z * vars.t[k][2];

        model.add_constraint(constraint!(up_dot_t >= cos_th * vars.gamma[k]));
    }

    // Rate of change of thrust (Equation 73):
    // dot_min*dt <= Gamma[k+1] - Gamma[k] <= Tdot_max*dt
    for k in 0..N - 1 {
        model.add_constraint(constraint!(
            vars.gamma[k + 1] - vars.gamma[k] >= params.tdot_min * settings.dt
        ));
        model.add_constraint(constraint!(
            vars.gamma[k + 1] - vars.gamma[k] <= params.tdot_max * settings.dt
        ));
    }
}

/// Add the slack variable constraints
fn add_slack_constraints(
    model: &mut impl SolverModel,
    vars: &DecisionVariables,
    params: &SimulationParams,
    settings: &AlgorithmParams,
) {
    for k in 0..settings.N {
        // SC Modifications
        // ||a_R[k]|| <= k_aR[k] }
        model.add_constraint(soc_constraint!(
            norm2(vars.aR[k][0], vars.aR[k][1], vars.aR[k][2]) <= vars.kappa_aR[k]
        ));
    }
}
