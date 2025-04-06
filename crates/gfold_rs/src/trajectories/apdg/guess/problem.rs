use good_lp::{
    clarabel, constraint, variable, variables, Expression, ProblemVariables, SolverModel, Variable,
};
use nalgebra::Vector3;

use crate::trajectories::apdg::models::{AlgorithmParams, SimulationParams};

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

struct APDGProblem<S: SolverModel> {
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
    fn new(
        sim_params: &SimulationParams,
        algo_params: &AlgorithmParams,
    ) -> APDGProblem<impl SolverModel> {
        let (decision_vars, model) = setup_problem(sim_params, algo_params);
        let mut problem = APDGProblem { solver: model };

        // Some relationships
        let alpha = 1.0 / (sim_params.i_sp * sim_params.g_0); //  relates thrust to mass flow rate
        let m_dot_bp =
            (sim_params.p_amb * sim_params.a_nozzle) / (sim_params.i_sp * sim_params.g_0); // Mass flow rate

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
            m_dot_bp,
            alpha,
            mu,
            s,
        );

        problem
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

    let mut model = vars
        .minimise(
            -algo.w_mf * decision_variables.m[N - 1]
                + algo.w_kappa_aR * decision_variables.kappa_aR.iter().sum::<Expression>(),
        )
        .using(clarabel);

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
    settings: &AlgorithmParams,
) {
    // Final position r[N-1] = r0
    for (var, &r0) in vars.r[settings.N - 1].iter().zip(params.r0.iter()) {
        model.add_constraint(constraint!(*var == r0));
    }

    // Final velocity v[N-1] = v0
    for (var, &v0) in vars.v[settings.N - 1].iter().zip(params.v0.iter()) {
        model.add_constraint(constraint!(*var == v0));
    }

    // Final thrust direction
    // T[N-1] = Gamma[N-1] * n_hatf
    for (var, &n_hatf) in vars.t[settings.N - 1].iter().zip(params.n_hatf.iter()) {
        model.add_constraint(constraint!(*var == params.gamma_0_vac * n_hatf));
    }
}

fn pre_compute(params: &SimulationParams, settings: &AlgorithmParams) -> (Vec<f64>, Vec<f64>) {
    // Pre-computed values for Problem 4
    // mu[k] = ((k_n - k)/k_n)*m_0 + (k/k_n)*m_dry
    let mu = |k: usize| {
        let k_n = settings.N as f64;
        let k = k as f64;
        ((k_n - k) / k_n) * params.m_0 + (k / k_n) * params.m_dry
    };

    // s[k] = (k_n - k/k_n) ||v_0|| + (k/k_n) ||v_f||
    let s = |k: usize| {
        let k_n = settings.N as f64;
        let k = k as f64;
        let v0_norm = (params.v0.iter().map(|&x| x * x).sum::<f64>()).sqrt();
        let v_f_norm = (params.vf.iter().map(|&x| x * x).sum::<f64>()).sqrt();
        ((k_n - k) / k_n) * v0_norm + (k / k_n) * v_f_norm
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
    m_dot_bp: f64,
    alpha: f64,
    mu: Vec<f64>,
    s: Vec<f64>,
) {
    let N = settings.N;

    for k in 0..N - 1 {
        // Mass dynamics
        // m[k+1] = m[k] - [alpha/2 * (gamma[k] + gamma[k+1]) + m_dot_bp] * dt
        model.add_constraint(constraint!(
            vars.m[k + 1]
                == vars.m[k] - alpha / 2.0 * (vars.gamma[k] + vars.gamma[k + 1])
                    + m_dot_bp * settings.dt
        ));

        // Position dynamics
        // r[k+1] = r[k] + v[k] * dt + 1/3 * (a[k] + 1/2*a[k+1]) * dt^2
        for i in 0..3 {
            model.add_constraint(constraint!(
                vars.r[k + 1][i]
                    == vars.r[k][i]
                        + vars.v[k][i] * settings.dt
                        + 1.0 / 3.0 * (vars.a[k][i] + 0.5 * vars.a[k + 1][i]) * settings.dt.powi(2)
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
        //a[k] = 1/mu[k] * (T[k] - 1/2 * p_amb * S_D * C_D * s[k] * v[k]) + a_R[k] + g
        for i in 0..3 {
            model.add_constraint(constraint!(
                vars.a[k][i]
                    == 1.0 / mu[k]
                        * (vars.t[k][i]
                            - 0.5 * params.p_amb * params.s_d * params.c_d * s[k] * vars.v[k][i])
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
    for k in 0..N - 1 {
        // Mass lowerbound constraint
        // m[k] >= m_dry
        model.add_constraint(constraint!(vars.m[k] >= params.m_dry));
    }

    // Glide-slope constraint
    // ||r[k]|| cos(gamma_gs) <= e_u^T * r[k]
    // let sec_gamma_gs = 1.0 / f64::to_radians(gamma_gs).cos();
}
