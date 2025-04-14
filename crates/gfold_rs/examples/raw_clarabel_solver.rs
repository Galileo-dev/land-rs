#![allow(non_snake_case)]
#![allow(clippy::too_many_lines)]

use clarabel::algebra::*;
use clarabel::solver::*;
use plotters::prelude::*;

fn main() {
    // -------------------------------------------------------
    // 1) Specify vehicle and environmental parameters
    // -------------------------------------------------------

    // Environmental parameters
    let g_0 = 9.807; // [m/s^2]      gravity
    let g_vec = [-g_0, 0.0, 0.0]; // [m/s^2]  gravity vector

    // Vehicle parameters
    let m_0 = 15_000.0; // [kg]         initial mass
    let m_dry = 10_000.0; // [kg]         dry mass

    // Thrust parameters
    let I_sp = 300.0; // [s]          specific impulse
    let T_min = 100_000.0; // [N]          minimum thrust
    let T_max = 250_000.0; // [N]          maximum thrust
    let Tdot_min = -100_000.0; // [N/s]        minimum thrust rate
    let Tdot_max = 100_000.0; // [N/s]        maximum thrust rate
    let Gamma_0 = 175_000.0; // [N]        initial thrust

    // Drag parameters
    let S_D = 10.0; // [m^2]        vehicle drag reference area
    let C_D = 1.0; //              drag coefficient
    let rho = 1.0; // [kg/m^3]     air density
    let A_nozzle = 0.5; // [m^2]        exit area of the rocket nozzle
    let P_amb = 100_000.0; // [Pa]         ambient pressure

    // Directional parameters
    let n_hat0 = [1.0, 0.0, 0.0]; //              initial normal vector
    let n_hatf = [1.0, 0.0, 0.0]; //              final normal vector
    let e_hat_Tu = [1.0, 0.0, 0.0]; //              Up pointing unit vector

    // Safety parameters
    let theta_max = 15.0; // [°] max tilt angle from normal
    let gamma_gs = 80.0; // [°] glide slope angle

    // Supose we have the follow inital conditions
    let r0 = [500.0, 500.0, 0.0]; // [m] initial position
    let v0 = [-50.0, 0.0, 50.0]; // [m/s] initial velocity
    let v_f = [0.0, 0.0, 0.0]; // [m/s] desired final velocity

    // Some relationships
    let alpha = 1.0 / (I_sp * g_0); //  relates thrust to mass flow rate
    let m_dot_bp = (P_amb * A_nozzle) / (I_sp * g_0); // Mass flow rate

    // Objective weights
    let w_mf = 1.0; // weight for final mass in cost
    let w_kappa_aR = 100.0; // penalty for large relaxations

    // -------------------------------------------------------
    // 2) Specify a time of flight guess, tf,s, and compute dt
    //    using Eq. 55.
    // -------------------------------------------------------

    // Pick a final time guess tf and number of steps N
    let N = 30; // number of steps
    let tf_guess = 15.0; // [s] guess of total time of flight
    let dt = tf_guess / (N as f64); // uniform time step

    // Pre-computed values for Problem 4
    // mu[k] = ((k_n - k)/k_n)*m_0 + (k/k_n)*m_dry
    let mu = |k: usize| {
        let k_n = N as f64;
        let k = k as f64;
        ((k_n - k) / k_n) * m_0 + (k / k_n) * m_dry
    };

    // s[k] = (k_n - k/k_n) ||v_0|| + (k/k_n) ||v_f||
    let s = |k: usize| {
        let k_n = N as f64;
        let k = k as f64;
        let v0_norm = (v0.iter().map(|&x| x * x).sum::<f64>()).sqrt();
        let v_f_norm = (v_f.iter().map(|&x| x * x).sum::<f64>()).sqrt();
        ((k_n - k) / k_n) * v0_norm + (k / k_n) * v_f_norm
    };

    // -------------------------------------------------------
    // Decision variables:
    // --------------------------------------------------------
    // Group 15 variables per step
    //
    // Order:
    //   r_x, r_y, r_z,
    //   v_x, v_y, v_z,
    //   m,
    //   T_x, T_y, T_z,
    //   Gamma,
    //   aR_x, aR_y, aR_z,
    //   kappa_{a,R}.
    //
    // [r(3) + v(3) + a(3) + m(1) + T(3),+ Gamma(1), + a_R(3), + kappa_{a,R}(1) + z(1)]
    let num_vars_per_step = 19;
    let n = num_vars_per_step * N;

    // Helpers for indexing
    let idx_r = |k, i| k * num_vars_per_step + i; // [0..2]
    let idx_v = |k, i| k * num_vars_per_step + 3 + i; // [3..5]
    let idx_a = |k, i| k * num_vars_per_step + 6 + i; // [6..8]
    let idx_m = |k| k * num_vars_per_step + 9; // [9]
    let idx_T = |k, i| k * num_vars_per_step + 10 + i; // [10..12]
    let idx_Gamma = |k| k * num_vars_per_step + 13; // [13]
    let idx_aR = |k, i| k * num_vars_per_step + 14 + i; // [14..16]
    let idx_kappa_aR = |k| k * num_vars_per_step + 17; // [17]
    let idx_z = |k| k * num_vars_per_step + 18; // [18]

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

    // No quadratic term in the objective
    let P = CscMatrix::spalloc((n, n), 0);

    // Linear term in the objective
    let mut q = vec![0.; n];
    q[idx_m(N - 1)] = -w_mf; // Maximise the final mass

    for k in 0..N {
        q[idx_kappa_aR(k)] = w_kappa_aR; // Penalise relaxation
    }

    // ---------------------------------------------------
    // Constraints: Ax + s = b, s ∈ K
    // ---------------------------------------------------

    let (mut Ai, mut Aj, mut Av, mut b, mut cones) =
        (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());
    let mut row_count = 0;

    /// Equality constraint
    ///
    /// Standard form:
    /// sum (coefficients × variables) <= rhs
    macro_rules! eq {
        ($cols:expr, $rhs:expr) => {{
            for &(c, v) in $cols.iter() {
                Ai.push(row_count);
                Aj.push(c);
                Av.push(v);
            }
            b.push($rhs);
            cones.push(ZeroConeT(1));
            row_count += 1;
        }};
    }

    /// Inequality constraint
    ///
    /// Standard form:
    /// sum (coefficients × variables) <= rhs
    macro_rules! leq {
        ($cols:expr, $rhs:expr) => {{
            for &(c, v) in $cols.iter() {
                Ai.push(row_count);
                Aj.push(c);
                Av.push(v);
            }
            b.push($rhs);
            cones.push(NonnegativeConeT(1)); // Means " <= 0 " in standard form
            row_count += 1;
        }};
    }

    /// Second order cone constraint
    ///
    /// Standard form:
    /// sqrt( x^2 + y^2 + z^2 ) <= t
    macro_rules! soc4 {
        ($xyz:expr, $tcol:expr) => {{
            let (tc, tv) = $tcol;

            Ai.push(row_count);
            Aj.push(tc);
            Av.push(tv);
            b.push(0.0);

            for (i, &(c, v)) in $xyz.iter().enumerate() {
                Ai.push(row_count + 1 + i);
                Aj.push(c);
                Av.push(v);
                b.push(0.0);
            }
            cones.push(SecondOrderConeT(4));
            row_count += 4;
        }};
    }

    // ---------------------------------------------------
    // Initial conditions
    // ---------------------------------------------------

    // Initial Mass m[0] = m_0
    eq!(&[(idx_m(0), 1.0)], m_0);

    // Initial Position r[0] = r_0
    for i in 0..3 {
        eq!(&[(idx_r(0, i), 1.0)], r0[i]);
    }

    // Initial Velocity v[0] = v_0
    for i in 0..3 {
        eq!(&[(idx_v(0, i), 1.0)], v0[i]);
    }

    // T[0] = Gamma_0 * n_hat0
    // n_hat0 is the initial normal vector
    for i in 0..3 {
        eq!(&[(idx_T(0, i), 1.0)], Gamma_0 * n_hat0[i]);
    }

    // Gamma[0] = Gamma_0
    eq!(&[(idx_Gamma(0), 1.0)], Gamma_0);

    // ---------------------------------------------------
    // Final conditions
    // ---------------------------------------------------

    // Final position r[N] = 0
    for i in 0..3 {
        eq!(&[(idx_r(N - 1, i), 1.0)], 0.0);
    }
    // Final velocity v[N] = 0
    for i in 0..3 {
        eq!(&[(idx_v(N - 1, i), 1.0)], 0.0);
    }

    // Original:
    //      T[N-1] = Gamma[N-1] * n_hatf
    // Rearranged:
    //      T[N-1] - Gamma[N-1] * n_hatf = 0
    for i in 0..3 {
        eq!(
            &[(idx_T(N - 1, i), 1.0), (idx_Gamma(N - 1), -n_hatf[i])],
            0.0
        );
    }

    // ---------------------------------------------------
    // Dynamics
    // ---------------------------------------------------

    // Mass dynamics (Equation 64)
    for k in 0..N - 1 {
        // Original:
        //      m[k+1] = m[k] - [alpha/2 * (gamma[k] + gamma[k+1]) + m_dot_bp] * dt
        // Rearranged:
        //      m[k+1] - m[k] + [alpha/2 * (gamma[k] + gamma[k+1])] * dt = -m_dot_bp * dt
        eq!(
            &[
                (idx_m(k + 1), 1.0),
                (idx_m(k), -1.0),
                (idx_Gamma(k), alpha / 2.0 * dt),
                (idx_Gamma(k + 1), alpha / 2.0 * dt),
            ],
            -m_dot_bp * dt
        );
    }

    // Position dynamics (Equation 65)
    for k in 0..N - 1 {
        for i in 0..3 {
            // Original:
            //      r[k+1] = r[k] + v[k] * dt + 1/3 * (a[k] + 1/2*a[k+1]) * dt^2
            // Rearranged:
            //      r[k+1] - r[k] - v[k] * dt - (1/3) * a[k] * dt^2 - (1/6) * a[k+1] * dt^2 = 0
            eq!(
                &[
                    (idx_r(k + 1, i), 1.0),
                    (idx_r(k, i), -1.0),
                    (idx_v(k, i), -dt),
                    (idx_a(k, i), -dt * dt / 3.0),
                    (idx_a(k + 1, i), -dt * dt / 6.0),
                ],
                0.0
            );
        }
    }

    // Velocity dynamics (Equation 66)
    for k in 0..N - 1 {
        for i in 0..3 {
            // Original:
            //      v[k+1] = v[k] + 1/2 * (a[k] + a[k+1]) * dt
            // Rearranged:
            //      v[k+1] - v[k] - (1/2) * a[k] * dt - (1/2) * a[k+1] * dt = 0
            eq!(
                &[
                    (idx_v(k + 1, i), 1.0),
                    (idx_v(k, i), -1.0),
                    (idx_a(k, i), -dt / 2.0),
                    (idx_a(k + 1, i), -dt / 2.0),
                ],
                0.0
            );
        }
    }

    // Acceleration dynamics (Equation 67)
    for k in 0..N - 1 {
        for i in 0..3 {
            // TODO!: Revisit this equation, my brain is fried
            // Original:
            //      a[k] = 1/mu[k] * (T[k] - 1/2 * p * S_D * C_D * s[k] * v[k]) + a_R[k] + g
            // Rearranged:
            //      1/mu[k] * T[k] (- 1/2 * 1/mu[k] * p * S_D * C_D * s[k] * v[k] + a_r[k]) - a[k] = -g
            eq!(
                &[
                    (idx_a(k, i), 1.0),
                    (idx_aR(k, i), -1.0),
                    (idx_T(k, i), -1.0 / mu(k)),
                    (idx_v(k, i), 0.5 * rho * S_D * C_D * s(k) / mu(k)),
                ],
                g_vec[i]
            );
        }
    }

    // ---------------------------------------------------
    // State constraints
    // ---------------------------------------------------

    // Mass (Equation 68):
    // Original:
    //       m[k] >= m_dry
    // Rearranged:
    //      -1 * m[k] <= -m_dry
    for k in 0..N {
        leq!(&[(idx_m(k), -1.0)], -m_dry);
    }

    // Glide-slope constraint (Equation 69):
    // Original:
    //      ||r[k]|| cos(gamma_gs) <= e_u^T * r[k]
    // Rearranged:
    //      ||r[k]|| <= (e_u^T * r[k]) / cos(gamma_gs)

    let sec_gamma_gs = 1.0 / f64::to_radians(gamma_gs).cos();

    for k in 0..N {
        // introdce auxiliary variable z[k]
        eq!(
            &[
                (idx_z(k), 1.0),
                (idx_r(k, 0), -e_hat_Tu[0] * sec_gamma_gs),
                (idx_r(k, 1), -e_hat_Tu[1] * sec_gamma_gs),
                (idx_r(k, 2), -e_hat_Tu[2] * sec_gamma_gs),
            ],
            0.0
        );

        soc4!(
            &[
                (idx_r(k, 0), -1.0),
                (idx_r(k, 1), -1.0),
                (idx_r(k, 2), -1.0),
            ],
            (idx_z(k), -1.0)
        );
    }

    // Thrust (Equation 70)
    // Original:
    //      ||T[k]|| <= Gamma[k]
    for k in 0..N {
        soc4!(
            &[
                (idx_T(k, 0), -1.0),
                (idx_T(k, 1), -1.0),
                (idx_T(k, 2), -1.0),
            ],
            (idx_Gamma(k), -1.0)
        );
    }

    // Max/Min thrust (Equation 71)
    // Original:
    //      T_min <= Gamma[k] <= T_max
    // Rearranged:
    //      -Gamma[k] <= -T_min
    //      Gamma[k] <= T_max
    for k in 0..N {
        leq!(&[(idx_Gamma(k), -1.0)], -T_min);
        leq!(&[(idx_Gamma(k), 1.0)], T_max);
    }

    // Tilt constraint (Equation 72):
    // Original:
    //      Gamma[k] * cos(theta_max) <= e^T T[k].
    // Rearranged:
    //      −e_hat_T * T[k] + Gamma[k] * cos(θmax​) <= 0
    for k in 0..N {
        let mut cols = vec![];
        for i in 0..3 {
            cols.push((idx_T(k, i), -e_hat_Tu[i])); // -e_hat_T * T[k]
        }
        cols.push((idx_Gamma(k), f64::to_radians(theta_max).cos())); // Gamma[k] * cos(theta_max)
        leq!(&cols, 0.0); // <= 0
    }

    // Rate of change of thrust (Equation 73):
    // Original:
    //      Tdot_min*dt <= Gamma[k+1] - Gamma[k] <= Tdot_max*dt
    // Rearranged:
    //      Gamma[k+1] - Gamma[k] <= Tdot_max*dt
    //      -Gamma[k+1] + Gamma[k] <= -Tdot_min*dt
    for k in 0..(N - 1) {
        leq!(
            &[(idx_Gamma(k + 1), 1.0), (idx_Gamma(k), -1.0),],
            Tdot_max * dt
        );
        leq!(
            &[(idx_Gamma(k + 1), -1.0), (idx_Gamma(k), 1.0),],
            -Tdot_min * dt
        );
    }

    // SC Modifications
    // Original:
    //      ||a_R[k]|| <= k_aR[k]
    // Rearranged:

    for k in 0..N {
        soc4!(
            &[
                (idx_aR(k, 0), -1.0),
                (idx_aR(k, 1), -1.0),
                (idx_aR(k, 2), -1.0),
            ],
            (idx_kappa_aR(k), -1.0)
        );
    }

    // ---------------------------------------------------
    // Convert to a CscMatrix and solve
    // ---------------------------------------------------

    // print out the number of rows and columns
    println!("row_count: {}", row_count);
    println!("n: {}", n);
    println!("len of Ai: {}", Ai.len());
    println!("len of Aj: {}", Aj.len());
    println!("len of Av: {}", Av.len());
    println!("len of b: {}", b.len());
    println!("len of cones: {}", cones.len());

    let A: CscMatrix = CscMatrix::new_from_triplets(row_count, n, Ai, Aj, Av);

    // Check the format of the matrix
    assert!(P.check_format().is_ok());
    assert!(A.check_format().is_ok());

    let settings = DefaultSettingsBuilder::default()
        .verbose(true)
        .build()
        .unwrap();

    let mut solver = DefaultSolver::new(&P, &q, &A, &b, &cones, settings);

    solver.solve();

    let trajectory: Vec<(f64, f64, f64)> = (0..N)
        .map(|k| {
            (
                solver.solution.x[idx_r(k, 1)],
                solver.solution.x[idx_r(k, 0)],
                solver.solution.x[idx_r(k, 2)],
            )
        })
        .collect();

    let thrust_vectors: Vec<(f64, f64, f64)> = (0..N)
        .map(|k| {
            (
                solver.solution.x[idx_T(k, 1)],
                solver.solution.x[idx_T(k, 0)],
                solver.solution.x[idx_T(k, 2)],
            )
        })
        .collect();

    let velocity: Vec<(f64, f64, f64)> = (0..N)
        .map(|k| {
            (
                solver.solution.x[idx_v(k, 1)],
                solver.solution.x[idx_v(k, 0)],
                solver.solution.x[idx_v(k, 2)],
            )
        })
        .collect();

    // Print the different positions [3]

    println!("Trajectory: {:?}", trajectory);
    println!("Thrust Vectors: {:?}", thrust_vectors);
    println!("Velocity: {:?}", velocity);
}
