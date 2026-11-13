//! End-to-end landing test: runs the full APDG / successive-convexification
//! stack and checks the result is physically valid and that the solution really
//! satisfies the Hermite-Simpson discretisation used in the problems.

use gfold_rs::trajectories::{APDGProblemSolver, AlgorithmParams, Settings, SimulationParams};

/// Touchdown altitude [m] (the up axis is +x for this model).
const RF: [f64; 3] = [30.0, 0.0, 0.0];

const BOUNDARY_TOL: f64 = 1e-3;
const THRUST_TOL: f64 = 1e-3 * 250_000.0;
/// The dynamics enter the successive problem as hard equalities, so a
/// well-scaled solve satisfies the Hermite-Simpson identity to solver accuracy.
const DYNAMICS_TOL: f64 = 1e-4;

#[test]
fn lands_with_feasible_thrust_and_consistent_dynamics() {
    let sim = SimulationParams::builder().rf(RF.into()).build();
    let settings = Settings::builder()
        .simulation_settings(sim.clone())
        .build();

    let mut solver = APDGProblemSolver::default();
    let (solution, _history) = solver.solve(&settings).expect("solver failed");

    let n = settings.solver_settings().N;
    let dt = solution.dt();
    let steps = solution.steps();
    assert_eq!(steps.len(), n, "unexpected number of time steps");
    assert!(dt > 0.0, "dt must be positive, got {dt}");

    // --- Boundary conditions ---------------------------------------------
    assert_vec(&steps[0].r, &sim.r0, BOUNDARY_TOL, "r[0] == r0");
    assert_vec(&steps[0].v, &sim.v0, BOUNDARY_TOL, "v[0] == v0");
    assert_vec(&steps[n - 1].r, &sim.rf, BOUNDARY_TOL, "r[last] == rf");
    assert_vec(&steps[n - 1].v, &sim.vf, BOUNDARY_TOL, "v[last] == vf");

    // --- Mass and thrust feasibility -------------------------------------
    for (k, step) in steps.iter().enumerate() {
        assert!(
            step.m >= sim.m_dry - BOUNDARY_TOL,
            "step {k}: mass {} below dry mass {}",
            step.m,
            sim.m_dry
        );
        assert!(
            step.gamma >= sim.t_min_vac - THRUST_TOL,
            "step {k}: thrust magnitude {} below t_min_vac {}",
            step.gamma,
            sim.t_min_vac
        );
        assert!(
            step.gamma <= sim.t_max_vac + THRUST_TOL,
            "step {k}: thrust magnitude {} above t_max_vac {}",
            step.gamma,
            sim.t_max_vac
        );
        assert!(
            step.t.norm() <= step.gamma + THRUST_TOL,
            "step {k}: ||t|| {} exceeds gamma {}",
            step.t.norm(),
            step.gamma
        );
    }
    assert!(
        steps[n - 1].m > sim.m_dry,
        "final mass {} must exceed dry mass {}",
        steps[n - 1].m,
        sim.m_dry
    );

    // --- The plan must be physically flyable ------------------------------
    // The aR relaxation slack exists to restore feasibility during SC; at
    // convergence it must be ~0, otherwise the plan's accelerations cannot be
    // produced by the reported thrust, drag and gravity (an unflyable plan).
    for (k, step) in steps.iter().enumerate() {
        assert!(
            step.aR.norm() < 0.1,
            "step {k}: relaxation slack ||aR|| = {} — dynamics carried by aR",
            step.aR.norm()
        );
        let drag = -0.5 * sim.rho * sim.s_d * sim.c_d * step.v.norm() * step.v;
        let expected = (step.t + drag) / step.m + sim.g_vec;
        let err = (step.a - expected).norm();
        assert!(
            err < 0.1,
            "step {k}: a {} inconsistent with (t + drag)/m + g {} (err {err})",
            step.a,
            expected
        );
    }

    // --- Hermite-Simpson position consistency ----------------------------
    // r[k+1] = r[k] + dt/2 (v[k] + v[k+1]) + dt^2/12 (a[k] - a[k+1])
    for k in 0..n - 1 {
        let (s, s_next) = (&steps[k], &steps[k + 1]);
        let expected = s.r + (s.v + s_next.v) * (dt / 2.0) + (s.a - s_next.a) * (dt * dt / 12.0);
        let err = (s_next.r - expected).norm();
        let rel = err / expected.norm().max(1.0);
        assert!(
            rel <= DYNAMICS_TOL,
            "step {k}: Hermite-Simpson residual {err:.3e} ({rel:.3e} relative) exceeds {DYNAMICS_TOL:.1e}\n  r[k+1] = {:?}\n  expected = {:?}",
            s_next.r,
            expected
        );
    }
}

fn assert_vec(got: &nalgebra::Vector3<f64>, want: &nalgebra::Vector3<f64>, tol: f64, what: &str) {
    let err = (got - want).norm();
    assert!(
        err <= tol,
        "{what}: got {got:?}, want {want:?} (error {err:.3e})"
    );
}
