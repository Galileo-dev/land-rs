use good_lp::{
    clarabel, constraint, variable, variables, Expression, ProblemVariables, SolverModel, Variable,
};
use nalgebra::Vector3;

use crate::{
    trajectories::apdg::models::{AlgorithmParams, SimulationParams},
    utils::lpvector3::LpVector3,
};

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
    r: Vec<LpVector3>,
    /// Velocity
    /// [m/s]
    v: Vec<LpVector3>,
    /// Acceleration
    /// [m/s^2]
    a: Vec<LpVector3>,
    /// Mass
    /// [kg]
    m: Vec<Variable>,
    /// Thrust
    /// [N]
    t: Vec<LpVector3>,
    /// Angle of attack
    /// [rad]
    gamma: Vec<Variable>,
    /// Angle of attack rate
    /// [rad/s]
    aR: Vec<LpVector3>,
    /// Relaxation
    /// []
    kappa_aR: Vec<Variable>,
    /// Glide slope slack
    /// []
    z: Vec<Variable>,
}

impl DecisionVariables {
    fn fixed_vector(vars: &mut good_lp::ProblemVariables) -> LpVector3 {
        LpVector3::from_vec(vars.add_vector(variable(), 3))
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
    fn new(params: &SimulationParams, algo: &AlgorithmParams) -> APDGProblem<impl SolverModel> {
        let (decision_vars, model) = setup_problem(params, algo);
        let mut problem = APDGProblem { solver: model };

        // Add initial constraints
        add_initial_constraints(&mut problem.solver, &decision_vars, algo);

        // Add final constraints
        add_final_constraints(&mut problem.solver, &decision_vars, algo);

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

    let mut decision_vars = DecisionVariables::new(&mut vars, N);

    let mut model = vars
        .minimise(
            -algo.w_mf * decision_vars.m[N - 1]
                + algo.w_kappa_aR * decision_vars.kappa_aR.iter().sum::<Expression>(),
        )
        .using(clarabel);

    (decision_vars, model)
}

/// Add initial condition constraints to the problem
fn add_initial_constraints(
    model: &mut impl SolverModel,
    decision_vars: &DecisionVariables,
    algo: &AlgorithmParams,
) {
    // Initial Mass m[0] = m_0
    model.add_constraint(constraint!(decision_vars.m[0] == algo.m_0));

    // Initial Position r[0] = r_0
    model.add_constraint(constraint!(decision_vars.r[0] == algo.r0));

    // Initial Velocity v[0] = v_0
    for (var, &v0) in decision_vars.v[0].iter().zip(algo.v0.iter()) {
        model.add_constraint(constraint!(*var == v0));
    }

    // T[0] = Gamma_0 * n_hat0
    // n_hat0 is the initial normal vector
    for (i, var) in decision_vars.t[0].iter().enumerate() {
        model.add_constraint(constraint!(*var == algo.gamma_0_vac * algo.n_hat0[i]));
    }

    // Gamma[0] = Gamma_0_vac
    model.add_constraint(constraint!(decision_vars.gamma[0] == algo.gamma_0_vac));
}

/// Add final condition constraints to the problem
fn add_final_constraints(
    model: &mut impl SolverModel,
    decision_vars: &DecisionVariables,
    algo: &AlgorithmParams,
) {
    // Final position r[N-1] = r0
    for (var, &r0) in decision_vars.r[algo.N - 1].iter().zip(algo.r0.iter()) {
        model.add_constraint(constraint!(*var == r0));
    }

    // Final velocity v[N-1] = v0
    for (var, &v0) in decision_vars.v[algo.N - 1].iter().zip(algo.v0.iter()) {
        model.add_constraint(constraint!(*var == v0));
    }

    // Final thrust direction
    // T[N-1] = Gamma[N-1] * n_hatf
    // for (var, &v0) in decision_vars.t[algo.N - 1].iter().zip(algo.v0.iter()) {
    //     model.add_constraint(constraint!(*var == v0));
    // }
    // model.add_constraint(constraint!(
    //     decision_vars.t[algo.N - 1] == decision_vars.gamma[algo.N - 1] * algo.n_hatf
    // ));
}
