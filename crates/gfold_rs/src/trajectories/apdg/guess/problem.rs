use good_lp::{
    clarabel, constraint, soc_constraint, variable, variables, Constraint, Expression,
    ProblemVariables, Solution, SolutionStatus, SolverModel, Variable,
};
use nalgebra::Vector3;

use crate::trajectories::{
    apdg::models::{AlgorithmParams, SimulationParams},
    APDGSolution, APDGSolutionTimeStep,
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

pub struct APDGProblem {
    sim_params: SimulationParams,
    algo_params: AlgorithmParams,
}

// Store all decision variables for a single time step
#[derive(Clone, Debug)]
pub struct TimeStepVariables {
    /// Position [m]
    r: Vector3<Variable>,
    /// Velocity [m/s]
    v: Vector3<Variable>,
    /// Acceleration [m/s^2]
    a: Vector3<Variable>,
    /// Mass [kg]
    m: Variable,
    /// Thrust [N]
    t: Vector3<Variable>,
    /// Thrust magnitude scalar [N]
    gamma: Variable,
    /// Acceleration relaxation term [m/s^2]
    aR: Vector3<Variable>,
    /// Relaxation slack []
    kappa_aR: Variable,
}

// Store all decision variables for all time steps
// Basically the global state
struct DecisionVariables {
    steps: Vec<TimeStepVariables>,
    N: usize,
}

impl DecisionVariables {
    fn fixed_vector(vars: &mut good_lp::ProblemVariables) -> Vector3<Variable> {
        Vector3::new(
            vars.add_variable(),
            vars.add_variable(),
            vars.add_variable(),
        )
    }

    // Create variables in an interleaved order by time step
    fn new(vars: &mut good_lp::ProblemVariables, N: usize) -> Self {
        let mut steps = Vec::with_capacity(N);
        for _k in 0..N {
            let r_k = Self::fixed_vector(vars);
            let v_k = Self::fixed_vector(vars);
            let a_k = Self::fixed_vector(vars);
            let m_k = vars.add_variable();
            let t_k = Self::fixed_vector(vars);
            let gamma_k = vars.add_variable();
            let aR_k = Self::fixed_vector(vars);
            let kappa_aR_k = vars.add_variable();

            steps.push(TimeStepVariables {
                r: r_k,
                v: v_k,
                a: a_k,
                m: m_k,
                t: t_k,
                gamma: gamma_k,
                aR: aR_k,
                kappa_aR: kappa_aR_k,
            });
        }
        DecisionVariables { steps, N }
    }
}

impl APDGProblem {
    pub fn new(sim_params: SimulationParams, algo_params: AlgorithmParams) -> APDGProblem {
        APDGProblem {
            sim_params,
            algo_params,
        }
    }

    /// Solve the problem
    pub fn solve(self) -> Result<APDGSolution, Error> {
        // Setup the problem inside solve
        let (decision_vars, mut model) = setup_problem(&self.sim_params, &self.algo_params);

        // Add initial constraints
        add_initial_constraints(&mut model, &decision_vars, &self.sim_params);

        // Add final constraints
        add_final_constraints(
            &mut model,
            &decision_vars,
            &self.sim_params,
            &self.algo_params,
        );

        // Pre-compute values for Problem 4
        let (mu, s) = pre_compute(&self.sim_params, &self.algo_params);

        // Add dynamics constraints
        add_dynamics_constraints(
            &mut model,
            &decision_vars,
            &self.sim_params,
            &self.algo_params,
            &mu,
            &s,
        );

        // Add state constraints
        add_state_constraints(
            &mut model,
            &decision_vars,
            &self.sim_params,
            &self.algo_params,
        );

        // Add slack constraints
        add_slack_constraints(
            &mut model,
            &decision_vars,
            &self.sim_params,
            &self.algo_params,
        );

        // Run the solver
        let solution = model
            .solve()
            .map_err(|e| Error::SolverError(format!("{:?}", e)))?;

        match solution.status() {
            SolutionStatus::Optimal => {
                let N = self.algo_params.N;
                let dt = self.algo_params.dt;
                let mut steps_solution = Vec::with_capacity(N);

                for k in 0..N {
                    let step_vars = &decision_vars.steps[k];

                    let r_sol = Vector3::new(
                        solution.value(step_vars.r[0]),
                        solution.value(step_vars.r[1]),
                        solution.value(step_vars.r[2]),
                    );
                    let v_sol = Vector3::new(
                        solution.value(step_vars.v[0]),
                        solution.value(step_vars.v[1]),
                        solution.value(step_vars.v[2]),
                    );
                    let a_sol = Vector3::new(
                        solution.value(step_vars.a[0]),
                        solution.value(step_vars.a[1]),
                        solution.value(step_vars.a[2]),
                    );
                    let m_sol = solution.value(step_vars.m);
                    let t_sol = Vector3::new(
                        solution.value(step_vars.t[0]),
                        solution.value(step_vars.t[1]),
                        solution.value(step_vars.t[2]),
                    );
                    let gamma_sol = solution.value(step_vars.gamma);
                    let aR_sol = Vector3::new(
                        solution.value(step_vars.aR[0]),
                        solution.value(step_vars.aR[1]),
                        solution.value(step_vars.aR[2]),
                    );

                    steps_solution.push(APDGSolutionTimeStep {
                        r: r_sol,
                        v: v_sol,
                        a: a_sol,
                        m: m_sol,
                        t: t_sol,
                        gamma: gamma_sol,
                        aR: aR_sol,
                    });
                }
                Ok(APDGSolution {
                    steps: steps_solution,
                    dt,
                })
            }
            _ => Err(Error::SolverError(format!(
                "Solver did not find an optimal solution. Status: {:?}",
                solution.status()
            ))),
        }
    }
}

/// Function to set up the problem with decision variables and objective function
fn setup_problem(
    params: &SimulationParams,
    algo: &AlgorithmParams,
) -> (DecisionVariables, impl SolverModel) {
    let N = algo.N;

    let mut vars = variables!();

    let decision_variables = DecisionVariables::new(&mut vars, N);

    let norm_kappa_aR_var = vars.add_variable();

    let mut objective = Expression::default();
    // Minimize: -w_mf * m[N-1] + w_kappa_aR * ||kappa_aR||
    objective += -algo.w_mf * decision_variables.steps[N - 1].m;
    objective += algo.w_kappa_aR * norm_kappa_aR_var;

    let mut model = vars.minimise(objective).using(clarabel);

    // Add SOC constraint: ||kappa_aR|| <= norm_kappa_aR
    let kappa_aR_vars: Vec<Variable> = decision_variables
        .steps
        .iter()
        .map(|s| s.kappa_aR)
        .collect();
    model.add_constraint(constraint!(norm_kappa_aR_var >= 0.0));
    model.add_constraint(soc_constraint!(
        norm2_vec(kappa_aR_vars) <= norm_kappa_aR_var
    ));

    model.settings();

    (decision_variables, model)
}

/// Add initial condition constraints to the problem
fn add_initial_constraints(
    model: &mut impl SolverModel,
    vars: &DecisionVariables,
    params: &SimulationParams,
) {
    // Initial Mass m[0] = m_0
    model.add_constraint(constraint!(vars.steps[0].m == params.m_0));

    // Initial Position r[0] = r_0
    for (var, &r0) in vars.steps[0].r.iter().zip(params.r0.iter()) {
        model.add_constraint(constraint!(*var == r0));
    }

    // Initial Velocity v[0] = v_0
    for (var, &v0) in vars.steps[0].v.iter().zip(params.v0.iter()) {
        model.add_constraint(constraint!(*var == v0));
    }

    // T[0] = Gamma_0 * n_hat0
    // n_hat0 is the initial normal vector
    for (i, var) in vars.steps[0].t.iter().enumerate() {
        model.add_constraint(constraint!(*var == params.gamma_0_vac * params.n_hat0[i]));
    }

    // Gamma[0] = Gamma_0_vac
    model.add_constraint(constraint!(vars.steps[0].gamma == params.gamma_0_vac));
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
    for (var, &rf_val) in vars.steps[k_end].r.iter().zip(params.rf.iter()) {
        model.add_constraint(constraint!(*var == rf_val));
    }

    // Final velocity v[N-1] = vf
    for (var, &vf_val) in vars.steps[k_end].v.iter().zip(params.vf.iter()) {
        model.add_constraint(constraint!(*var == vf_val));
    }

    // Final thrust direction
    // T[N-1] = Gamma[N-1] * n_hatf
    for (i, tf) in vars.steps[k_end].t.iter().enumerate() {
        model.add_constraint(constraint!(
            *tf == vars.steps[k_end].gamma * params.n_hatf[i]
        ));
    }
}

fn pre_compute(params: &SimulationParams, settings: &AlgorithmParams) -> (Vec<f64>, Vec<f64>) {
    // Pre-computed values for Problem 4
    // mu[k] = ((k_n - k)/k_n)*m_0 + (k/k_n)*m_dry
    let mu_fn = |k: usize| {
        let kn = settings.N as f64;
        let k_f64 = k as f64;
        ((kn - k_f64) / kn) * params.m_0 + (k_f64 / kn) * params.m_dry
    };

    // s[k] = (k_n - k/k_n) ||v_0|| + (k/k_n) ||v_f||
    let s_fn = |k: usize| {
        let kn = settings.N as f64;
        let k_f64 = k as f64;
        let v0_norm = params.v0.norm();
        let v_f_norm = params.vf.norm();
        ((kn - k_f64) / kn) * v0_norm + (k_f64 / kn) * v_f_norm
    };

    (
        (0..settings.N).map(mu_fn).collect(),
        (0..settings.N).map(s_fn).collect(),
    )
}

/// Add the discretized dynamics contraints
fn add_dynamics_constraints(
    model: &mut impl SolverModel,
    vars: &DecisionVariables,
    params: &SimulationParams,
    settings: &AlgorithmParams,
    mu: &[f64],
    s: &[f64],
) {
    let N = settings.N;

    // Some relationships
    let alpha = 1.0 / (params.i_sp * params.g_0); //  relates thrust to mass flow rate
    let m_dot_bp = (params.p_amb * params.a_nozzle) / (params.i_sp * params.g_0); // Mass flow rate

    for k in 0..N - 1 {
        // Mass dynamics
        // m[k+1] = m[k] - [alpha/2 * (gamma[k] + gamma[k+1]) + m_dot_bp] * dt
        model.add_constraint(constraint!(
            vars.steps[k + 1].m
                == vars.steps[k].m
                    - (alpha / 2.0 * (vars.steps[k].gamma + vars.steps[k + 1].gamma) * settings.dt)
                    - (m_dot_bp * settings.dt)
        ));

        // Position dynamics
        // r[k+1] = r[k] + v[k] * dt + 1/3 * (a[k] + 1/2*a[k+1]) * dt^2
        for i in 0..3 {
            model.add_constraint(constraint!(
                vars.steps[k + 1].r[i]
                    == vars.steps[k].r[i]
                        + vars.steps[k].v[i] * settings.dt
                        + (1.0 / 3.0)
                            * (vars.steps[k].a[i] + 0.5 * vars.steps[k + 1].a[i])
                            * settings.dt.powi(2)
            ));
        }

        // Velocity dynamics
        // v[k+1] = v[k] + 1/2 * (a[k] + a[k+1]) * dt
        for i in 0..3 {
            model.add_constraint(constraint!(
                vars.steps[k + 1].v[i]
                    == vars.steps[k].v[i]
                        + 0.5 * (vars.steps[k].a[i] + vars.steps[k + 1].a[i]) * settings.dt
            ));
        }
    }

    // Acceleration dynamics
    //a[k] = 1/mu[k] * (T[k] - 1/2 * rho * S_D * C_D * s[k] * v[k]) + a_R[k] + g
    for k in 0..N {
        if k < mu.len() && k < s.len() {
            for i in 0..3 {
                model.add_constraint(constraint!(
                    vars.steps[k].a[i]
                        == 1.0 / mu[k]
                            * (vars.steps[k].t[i]
                                - 0.5
                                    * params.rho
                                    * params.s_d
                                    * params.c_d
                                    * s[k]
                                    * vars.steps[k].v[i])
                            + vars.steps[k].aR[i]
                            + params.g_vec[i]
                ));
            }
        } else {
            eprintln!(
                "Warning: Index {} out of bounds for mu/s arrays in dynamics constraints.",
                k
            );
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
        model.add_constraint(constraint!(vars.steps[k].m >= params.m_dry));
    }

    // Glide-slope constraint
    // ||r[k]|| cos(gamma_gs) <= e_u^T * r[k]
    let sec_gs = 1.0 / f64::to_radians(params.gamma_gs).cos();
    for k in 0..N {
        let t_expr = sec_gs
            * (params.e_hat_up.x * vars.steps[k].r[0]
                + params.e_hat_up.y * vars.steps[k].r[1]
                + params.e_hat_up.z * vars.steps[k].r[2]);
        // Use the expression directly
        model.add_constraint(soc_constraint!(
            norm2(vars.steps[k].r[0], vars.steps[k].r[1], vars.steps[k].r[2]) <= t_expr
        ));
    }

    // Thrust (Equation 70)
    // ||T[k]|| <= Gamma[k]
    for k in 0..N {
        model.add_constraint(soc_constraint!(
            norm2(vars.steps[k].t[0], vars.steps[k].t[1], vars.steps[k].t[2])
                <= vars.steps[k].gamma
        ));
    }

    // Max/Min thrust (Equation 71)
    // T_min <= Gamma[k] <= T_max
    for k in 0..N {
        model.add_constraint(constraint!(vars.steps[k].gamma >= params.t_min_vac));
        model.add_constraint(constraint!(vars.steps[k].gamma <= params.t_max_vac));
    }

    // Tilt constraint (Equation 72):
    // Gamma[k] * cos(theta_max) <= e^T T[k].
    // e_hat_up dot T[k] - Gamma[k]*cos(...) >= 0
    let cos_th = f64::to_radians(params.theta_max).cos();
    for k in 0..N {
        let up_dot_t = params.e_hat_up.x * vars.steps[k].t[0]
            + params.e_hat_up.y * vars.steps[k].t[1]
            + params.e_hat_up.z * vars.steps[k].t[2];

        model.add_constraint(constraint!(up_dot_t >= cos_th * vars.steps[k].gamma));
    }

    // Rate of change of thrust (Equation 73):
    // dot_min*dt <= Gamma[k+1] - Gamma[k] <= Tdot_max*dt
    for k in 0..N - 1 {
        model.add_constraint(constraint!(
            vars.steps[k + 1].gamma - vars.steps[k].gamma >= params.tdot_min * settings.dt
        ));
        model.add_constraint(constraint!(
            vars.steps[k + 1].gamma - vars.steps[k].gamma <= params.tdot_max * settings.dt
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
            norm2(
                vars.steps[k].aR[0],
                vars.steps[k].aR[1],
                vars.steps[k].aR[2]
            ) <= vars.steps[k].kappa_aR
        ));
    }
}
