use super::Error;
use crate::trajectories::{
    apdg::models::{AlgorithmParams, SimulationParams},
    APDGSolution, APDGSolutionTimeStep,
};
use autodiff::F;
use good_lp::{
    clarabel, constraint, soc_constraint, variable, variables, Constraint, Expression,
    ProblemVariables, Solution, SolutionStatus, SolverModel, Variable,
};
use nalgebra::Vector3;

// -------------------------------------------------------
// Problem 5: Rocket Landing Optimal Control Problem
// Successive Convexification Step
//
//     min -w_mf * m[kf] + w_eta_dt * eta_dt + w_eta_T * ||eta_T|| + w_kappa_aR * ||kappa_aR||
//
// s.t.
//    Boundary Conditions, Dynamics, SOC Constraints
// -------------------------------------------------

pub struct APDGProblem {
    sim_params: SimulationParams,
    algo_params: AlgorithmParams,
    prev_trajectory: APDGSolution,
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
    /// Trust region slack for thrust T[k]
    eta_T: Variable, // Represents ηT[k] from Eq. 93
}

// Store all decision variables for all time steps
// Basically the global state
struct DecisionVariables {
    steps: Vec<TimeStepVariables>,
    N: usize,
    /// Time step duration [s]
    dt: Variable,
    /// Trust region slack variable for dt
    eta_dt: Variable,
    /// Auxiliary variable for L2 norm of eta_T (||ηT|| from Eq. 58)
    norm_eta_T: Variable,
    /// Auxiliary variable for L2 norm of kappa_aR (||κa,R|| from Eq. 59)
    norm_kappa_aR: Variable,
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
            let eta_T_k = vars.add_variable();

            steps.push(TimeStepVariables {
                r: r_k,
                v: v_k,
                a: a_k,
                m: m_k,
                t: t_k,
                gamma: gamma_k,
                aR: aR_k,
                kappa_aR: kappa_aR_k,
                eta_T: eta_T_k,
            });
        }

        // Add the global variables
        let dt_var = vars.add_variable();
        let eta_dt_var = vars.add_variable();
        let norm_eta_T_var = vars.add_variable();
        let norm_kappa_aR_var = vars.add_variable();

        DecisionVariables {
            steps,
            N,
            dt: dt_var,
            eta_dt: eta_dt_var,
            norm_eta_T: norm_eta_T_var,
            norm_kappa_aR: norm_kappa_aR_var,
        }
    }
}

impl APDGProblem {
    pub fn new(
        sim_params: SimulationParams,
        algo_params: AlgorithmParams,
        prev_trajectory: APDGSolution,
    ) -> APDGProblem {
        APDGProblem {
            sim_params,
            algo_params,
            prev_trajectory,
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

        // Add dynamics constraints
        add_linearised_dynamics_constraints(
            &mut model,
            &decision_vars,
            &self.sim_params,
            &self.algo_params,
            &self.prev_trajectory,
        );

        // Add state constraints
        add_state_constraints(
            &mut model,
            &decision_vars,
            &self.sim_params,
            &self.algo_params,
            &self.prev_trajectory,
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
                // Get the optimized dt from the solution
                let dt_sol = solution.value(decision_vars.dt);
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

                    // Populate using the expected APDGSolutionTimeStep struct
                    steps_solution.push(APDGSolutionTimeStep {
                        r: r_sol,
                        v: v_sol,
                        a: a_sol,
                        m: m_sol,
                        t: t_sol,
                        gamma: gamma_sol,
                    });
                }
                Ok(APDGSolution {
                    steps: steps_solution,
                    dt: dt_sol,
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

    let mut objective = Expression::default();
    // Minimise: -w_mf * m[kf] + w_eta_dt * eta_dt + w_eta_T * ||eta_T|| + w_kappa_aR * ||kappa_aR||
    objective += -algo.w_mf * decision_variables.steps[N - 1].m;
    objective += algo.w_eta_dt * decision_variables.eta_dt;
    objective += algo.w_eta_T * decision_variables.norm_eta_T;
    objective += algo.w_kappa_aR * decision_variables.norm_kappa_aR;

    let mut model = vars.minimise(objective).using(clarabel);

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

/// Add the linearised dynamics contraints
fn add_linearised_dynamics_constraints(
    model: &mut impl SolverModel,
    vars: &DecisionVariables,
    params: &SimulationParams,
    settings: &AlgorithmParams,
    prev_trajectory: &APDGSolution,
) {
    todo!("Add the linearised dynamics constraints");
}

/// Add the state constraints
fn add_state_constraints(
    model: &mut impl SolverModel,
    vars: &DecisionVariables,
    params: &SimulationParams,
    settings: &AlgorithmParams,
    prev_trajectory: &APDGSolution,
) {
    let N = settings.N;
    let dt_bar = prev_trajectory.dt;
    let dt_i = settings.dt;

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

    // Thrust Trust Region ||T[k] - T_bar[k]|| <= eta_T[k]
    for k in 0..N {
        let step_k_bar = &prev_trajectory.steps[k];
        let t_bar_k = step_k_bar.t;
        let current_t_k = vars.steps[k].t;
        let delta_t_x = current_t_k[0] - t_bar_k.x;
        let delta_t_y = current_t_k[1] - t_bar_k.y;
        let delta_t_z = current_t_k[2] - t_bar_k.z;

        let eta_T_k_var = vars.steps[k].eta_T;

        model.add_constraint(soc_constraint!(
            norm2(delta_t_x, delta_t_y, delta_t_z) <= eta_T_k_var
        ));
    }

    // Max/Min thrust (Equation 71)
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

    // Rate of change of thrust (Equation 73/91):
    // dot_min*dt_i <= Gamma[k+1] - Gamma[k] <= Tdot_max*dt_i
    for k in 0..N - 1 {
        model.add_constraint(constraint!(
            vars.steps[k + 1].gamma - vars.steps[k].gamma >= params.tdot_min * dt_i
        ));
        model.add_constraint(constraint!(
            vars.steps[k + 1].gamma - vars.steps[k].gamma <= params.tdot_max * dt_i
        ));
    }

    // Time Step Trust Region
    // (dt - dt_bar)^2 <= eta_dt
    model.add_constraint(constraint!(vars.eta_dt >= 0.0)); // Ensure slack is non-negative
    model.add_constraint(constraint!(vars.dt - dt_bar <= vars.eta_dt));
    model.add_constraint(constraint!(vars.dt - dt_bar >= -vars.eta_dt));

    // Trust Region Constraints
    // ||eta_T|| <= norm_eta_T
    let eta_T_vars: Vec<Variable> = vars.steps.iter().map(|s| s.eta_T).collect();
    model.add_constraint(constraint!(vars.norm_eta_T >= 0.0));
    model.add_constraint(soc_constraint!(norm2_vec(eta_T_vars) <= vars.norm_eta_T));

    // Constraint for ||kappa_aR|| <= norm_kappa_aR
    let kappa_aR_vars: Vec<Variable> = vars.steps.iter().map(|s| s.kappa_aR).collect();
    model.add_constraint(constraint!(vars.norm_kappa_aR >= 0.0));
    model.add_constraint(soc_constraint!(
        norm2_vec(kappa_aR_vars) <= vars.norm_kappa_aR
    ));
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
