use crate::{
    prelude::*,
    rocket::{self, EngineControlState, RocketConfig},
};
use bevy_rapier3d::na::Vector3;
use gfold_rs::trajectories::{APDGProblemSolver, APDGSolutionTimeStep, Settings, SimulationParams};

/// Gravity magnitude used by the solver and by Rapier [m/s²].
const G: f32 = 9.807;

/// Outer-loop tracking gains.
/// a_cmd = a_ff + KP * (r* - r) + KD * (v* - v)
///
/// Keep these modest: lateral authority is only T_max·sin(θ_max)/m ≈ 4 m/s²,
/// so aggressive feedback saturates the thrust magnitude and the vertical
/// component (T·cos θ ≥ T_min·cos θ_max) overcomes gravity — the rocket climbs
/// away from the plan instead of converging onto it.
const KP: f32 = 0.4;
const KD: f32 = 1.2;

/// Replan when the tracking error exceeds these bounds. Divergence past them
/// is unrecoverable by feedback alone (lateral authority is actuator-limited),
/// so the plan is regenerated from the current state instead.
const REPLAN_POS_ERR: f32 = 20.0; // [m]
const REPLAN_VEL_ERR: f32 = 8.0; // [m/s]
const REPLAN_COOLDOWN: f32 = 3.0; // [s] plan age before a replan may trigger

/// Vertical band around the touchdown point at which engines are cut off [m].
const TOUCHDOWN_BAND: f32 = 0.3;

/// Height of the landing pad's top surface above the ground plane [m].
const PAD_TOP: f32 = 0.1;

/// Auto-abort margin above the plan's highest altitude [m]: if the rocket
/// climbs this far past the plan envelope the engines are cut.
const ABORT_MARGIN: f32 = 50.0;

/// Plan against a shrunk actuator envelope so the min-fuel optimum is *interior*
/// and the tracking feedback keeps authority in reserve.
const MARGIN_THETA_SCALE: f64 = 0.8; // plan tilt ≤ 12° of the 15° gimbal limit
const MARGIN_THRUST_SCALE: f64 = 0.90; // plan thrust ≤ 225 kN of the 250 kN engine

/// A generated landing trajectory and the sim time it was generated at.
#[derive(Resource)]
pub struct Trajectory {
    sol: gfold_rs::trajectories::APDGSolution,
    generated_at: f32,
    /// Highest altitude in the plan [m]; abort ceiling is this + ABORT_MARGIN.
    ceiling: f32,
}

/// Set once the rocket has reached the touchdown point or aborted.
#[derive(Resource, Default, Deref, DerefMut)]
pub struct Landed(pub bool);

/// Sim time of the touchdown; the app exits shortly after.
#[derive(Resource, Default)]
pub struct TouchdownAt(pub Option<f32>);

pub fn plugin(app: &mut App) {
    app.init_resource::<Landed>()
        .init_resource::<TouchdownAt>()
        .add_systems(
            Update,
            solve_landing_system.run_if(|traj: Option<Res<Trajectory>>| traj.is_none()),
        )
        .add_systems(
            Update,
            (
                replan_on_divergence.run_if(resource_exists::<Trajectory>),
                draw_trajectory.run_if(resource_exists::<Trajectory>),
                track_trajectory.run_if(resource_exists::<Trajectory>),
                exit_after_touchdown,
            )
                .chain()
                .after(solve_landing_system),
        );
}

/// End the simulation shortly after touchdown: with engines cut the unguided
/// vehicle would just tip over and drift off the pad.
fn exit_after_touchdown(
    touchdown_at: Res<TouchdownAt>,
    time: Res<Time>,
    mut app_exit: EventWriter<AppExit>,
) {
    const GRACE: f32 = 1.5;
    if let Some(t0) = touchdown_at.0 {
        if time.elapsed_secs() - t0 > GRACE {
            info!("Touchdown confirmed — ending simulation");
            app_exit.send(AppExit::Success);
        }
    }
}

/// Solve a landing trajectory from the rocket's current state.
///
/// The convex solve blocks the schedule (~1 s wall clock); the rocket is
/// unpowered while it runs.
fn solve_landing_system(
    mut commands: Commands,
    rocket_q: Query<(&Transform, &Velocity, &RocketConfig), With<rocket::RocketRoot>>,
    time: Res<Time>,
) {
    if let Some(traj) = solve_landing(&rocket_q, time.elapsed_secs()) {
        info!(
            "Trajectory generated: {} steps, dt = {:.3}s, final mass = {:.0}kg",
            traj.sol.num_steps(),
            traj.sol.dt(),
            traj.sol.steps().last().map(|s| s.m).unwrap_or(0.0),
        );
        commands.insert_resource(traj);
    }
}

/// Solve from the rocket's current state; `None` on infeasibility or if the
/// rocket does not exist yet.
///
/// The plan is generated against a *shrunk* actuator envelope: the min-fuel
/// optimum sits exactly on the tilt/thrust bounds (bang-bang), and a plan on
/// the boundary saturates the tracking feedback — lateral authority is capped
/// by the tilt constraint, so any plant-model mismatch turns magnitude
/// saturation into a runaway climb. Planning with margin keeps tilt/thrust
/// reserve for the feedback loop. If the state is too far gone for the
/// margined envelope to be feasible, fall back to the nominal one.
fn solve_landing(
    rocket_q: &Query<(&Transform, &Velocity, &RocketConfig), With<rocket::RocketRoot>>,
    generated_at: f32,
) -> Option<Trajectory> {
    let (transform, vel, rocket_cfg) = rocket_q.get_single().ok()?;

    // Sim frame is x=E, y=U, z=N; the solver works in UEN.
    let r0 = Vector3::new(
        transform.translation.y as f64, // U
        transform.translation.x as f64, // E
        transform.translation.z as f64, // N
    );
    let v0 = Vector3::new(
        vel.linvel.y as f64, // U
        vel.linvel.x as f64, // E
        vel.linvel.z as f64, // N
    );

    // Touchdown when the engine bell rests on the pad: body centre at
    // touchdown offset above the pad top.
    let rf = Vector3::new(rocket_cfg.touchdown_offset() + PAD_TOP as f64, 0.0, 0.0);

    let nominal = SimulationParams {
        rf,
        ..rocket_cfg.0.to_sim_params(r0, v0)
    };

    let mut candidates: Vec<(&str, SimulationParams)> = Vec::with_capacity(2);
    if MARGIN_THETA_SCALE < 1.0 || MARGIN_THRUST_SCALE < 1.0 {
        candidates.push((
            "margined",
            SimulationParams {
                theta_max: rocket_cfg.0.theta_max * MARGIN_THETA_SCALE,
                t_max_vac: rocket_cfg.0.t_max_vac * MARGIN_THRUST_SCALE,
                ..nominal.clone()
            },
        ));
    }
    candidates.push(("nominal", nominal));

    for (label, params) in candidates {
        let settings = Settings::builder().simulation_settings(params).build();

        match APDGProblemSolver::default().solve(&settings) {
            Ok((solution, _)) => {
                let max_tilt = solution
                    .steps()
                    .iter()
                    .map(|s| {
                        (s.t.x / s.t.norm()) // e_hat_up = +U in UEN
                            .clamp(-1.0, 1.0)
                            .acos()
                            .to_degrees()
                    })
                    .fold(0.0_f64, f64::max);
                debug!(
                    "Solved {label} plan: max tilt {max_tilt:.1}deg, max thrust {:.0}N",
                    solution
                        .steps()
                        .iter()
                        .map(|s| s.gamma)
                        .fold(0.0_f64, f64::max),
                );

                let ceiling = solution
                    .steps()
                    .iter()
                    .map(|s| s.r.x as f32)
                    .fold(0.0_f32, f32::max);
                return Some(Trajectory {
                    sol: solution,
                    generated_at,
                    ceiling,
                });
            }
            Err(e) => {
                warn!("Solving {label} plan failed: {e}");
            }
        }
    }
    error!("Trajectory generation failed, retrying");
    None
}

/// Regenerate the plan from the current state when the rocket has drifted too
/// far from it for feedback to recover (the thrust envelope is anisotropic:
/// lateral authority is capped by the tilt constraint, so large errors cause
/// magnitude saturation and a runaway climb instead of convergence).
fn replan_on_divergence(
    mut commands: Commands,
    time: Res<Time>,
    traj: Res<Trajectory>,
    landed: Res<Landed>,
    rocket_q: Query<(&Transform, &Velocity, &RocketConfig), With<rocket::RocketRoot>>,
) {
    if **landed {
        return;
    }

    let t = time.elapsed_secs() - traj.generated_at;
    if t < REPLAN_COOLDOWN {
        return;
    }

    let Some(state_now) = plan_state_at(t, &traj) else {
        return; // plan exhausted; hover fallback handles it
    };

    let Ok((transform, vel, _)) = rocket_q.get_single() else {
        return;
    };

    let pos_err = (utils::uen_to_xyz(state_now.r) - transform.translation).length();
    let vel_err = (utils::uen_to_xyz(state_now.v) - vel.linvel).length();

    if pos_err > REPLAN_POS_ERR || vel_err > REPLAN_VEL_ERR {
        warn!(
            "Tracking diverged (pos err {pos_err:.1}m, vel err {vel_err:.1}m/s) — replanning"
        );
        if let Some(new_traj) = solve_landing(&rocket_q, time.elapsed_secs()) {
            info!(
                "Replanned: {} steps, dt = {:.3}s, ceiling = {:.0}m",
                new_traj.sol.num_steps(),
                new_traj.sol.dt(),
                new_traj.ceiling,
            );
            commands.insert_resource(new_traj);
        }
    }
}

fn draw_trajectory(traj: Res<Trajectory>, mut gizmos: Gizmos) {
    let steps = traj.sol.steps();
    for pair in steps.windows(2) {
        let a = utils::uen_to_xyz(pair[0].r);
        let b = utils::uen_to_xyz(pair[1].r);

        gizmos.line(a, b, Color::srgb(1.0, 0.2, 0.2));

        let thrust = utils::uen_to_xyz(pair[0].t);
        gizmos.line(a, a + thrust / 10.0, Color::srgb(0.2, 1.0, 0.2));
        gizmos.sphere(a, 0.5, Color::srgb(0.2, 0.2, 1.0));
    }
}

/// Trajectory-tracking controller.
///
/// Feedforward from the interpolated solution step plus PD feedback on the
/// position/velocity error, converted to a thrust vector and gimbal angles:
///
///   a_cmd = a* + KP (r* - r) + KD (v* - v)
///   T     = m (a_cmd - g) - D        (thrust vector, sim frame)
///
/// The gimbal steers the nozzle axis onto T; the magnitude is clamped to the
/// engine's thrust envelope.
fn track_trajectory(
    time: Res<Time>,
    traj: Res<Trajectory>,
    mut landed: ResMut<Landed>,
    mut touchdown_at: ResMut<TouchdownAt>,
    mut ev_tx: EventWriter<rocket::RocketControlInput>,
    mut last_log: Local<i32>,
    engine_q: Query<(Entity, &EngineControlState), With<EngineControlState>>,
    body_q: Query<
        (
            &GlobalTransform,
            &mut Velocity,
            &ReadMassProperties,
            &RocketConfig,
            Option<&rocket::DragForce>,
        ),
        With<rocket::RocketRoot>,
    >,
    mut gizmos: Gizmos,
) {
    let Ok((engine_ent, eng_state)) = engine_q.get_single() else {
        return;
    };
    let Ok((body_tf, body_vel, body_mass, rocket_cfg, drag)) = body_q.get_single() else {
        return;
    };

    if **landed {
        send_cut_off(&mut ev_tx, engine_ent);
        return;
    }

    // Trajectory-relative time with inter-step interpolation.
    let t = time.elapsed_secs() - traj.generated_at;
    let Some(state) = plan_state_at(t, &traj) else {
        // Plan exhausted without touchdown: hold minimum thrust near upright.
        send_hover(&mut ev_tx, engine_ent, rocket_cfg.0.t_min_vac as f32);
        return;
    };

    let r = body_tf.translation();
    let v = body_vel.linvel;
    let m = body_mass.mass as f32;

    let r_ref = utils::uen_to_xyz(state.r);
    let v_ref = utils::uen_to_xyz(state.v);
    let a_ff = utils::uen_to_xyz(state.a);

    let a_cmd = a_ff + KP * (r_ref - r) + KD * (v_ref - v);

    // Thrust vector: cancel gravity and measured aerodynamic drag.
    let drag_force = drag.map(|d| d.0).unwrap_or(Vec3::ZERO);
    let thrust_vec = m * (a_cmd - Vec3::new(0.0, -G, 0.0)) - drag_force;

    let t_min = rocket_cfg.0.t_min_vac as f32;
    let t_max = rocket_cfg.0.t_max_vac as f32;
    let thrust_mag = thrust_vec.length().clamp(t_min, t_max);

    gizmos.line(r, r_ref, Color::srgb(0.2, 0.2, 1.0));
    gizmos.line(r, r + thrust_vec / 10_000.0, Color::srgb(0.2, 1.0, 0.2));

    // 1 Hz telemetry: lets a headless run verify the descent.
    let whole_sec = t as i32;
    if whole_sec != *last_log {
        *last_log = whole_sec;
        let pos_err = r_ref - r;
        let vel_err = v_ref - v;
        info!(
            "t={:>4.1}s alt={:>6.1} ref={:>6.1} eP={:>5.2} eV={:>4.2} thrust={:>6.0}N",
            t,
            r.y,
            r_ref.y,
            pos_err.length(),
            vel_err.length(),
            thrust_mag,
        );
    }

    // Auto-abort: climbing above the plan envelope means tracking has failed
    // (magnitude saturation overcame gravity); cut the engines.
    if r.y > traj.ceiling + ABORT_MARGIN {
        error!(
            "ABORT: altitude {:.0}m exceeds plan ceiling {:.0}m + {ABORT_MARGIN}m — engines cut",
            r.y, traj.ceiling
        );
        *landed = Landed(true);
        send_cut_off(&mut ev_tx, engine_ent);
        return;
    }

    // Touchdown: near the landing band with the plan nearly spent.
    let plan_end = traj.sol.num_steps() as f32 * traj.sol.dt() as f32;
    let touchdown_y = rocket_cfg.touchdown_offset() as f32 + PAD_TOP + TOUCHDOWN_BAND;
    if r.y <= touchdown_y && t > 0.9 * plan_end {
        *landed = Landed(true);
        *touchdown_at = TouchdownAt(Some(time.elapsed_secs()));
        info!(
            "Touchdown at alt {:.2}m, |v| {:.2}m/s — engines cut off",
            r.y,
            v.length()
        );
        send_cut_off(&mut ev_tx, engine_ent);
        return;
    }

    // Gimbal: steer the nozzle axis onto the thrust direction.
    let body_rot = body_tf.compute_transform().rotation;
    let dir = thrust_vec.normalize_or_zero();
    let d_body = body_rot.inverse() * dir;

    // Nozzle local +Y maps to (-sin(yaw)cos(p), cos(yaw)cos(p), sin(p)) in the
    // body frame, so: pitch = asin(dz), yaw = atan2(-dx, dy).
    let pitch = d_body.z.clamp(-1.0, 1.0).asin();
    let yaw = (-d_body.x).atan2(d_body.y.max(1e-3));
    let max_angle = rocket_cfg.0.theta_max as f32;

    ev_tx.send(rocket::RocketControlInput {
        entity: engine_ent,
        input_type: rocket::RocketInputType::SetThrust(thrust_mag),
    });
    ev_tx.send(rocket::RocketControlInput {
        entity: engine_ent,
        input_type: rocket::RocketInputType::SetPitch((pitch / max_angle.to_radians()).clamp(-1.0, 1.0)),
    });
    ev_tx.send(rocket::RocketControlInput {
        entity: engine_ent,
        input_type: rocket::RocketInputType::SetYaw((yaw / max_angle.to_radians()).clamp(-1.0, 1.0)),
    });
}

fn send_cut_off(ev_tx: &mut EventWriter<rocket::RocketControlInput>, entity: Entity) {
    ev_tx.send(rocket::RocketControlInput {
        entity,
        input_type: rocket::RocketInputType::SetThrust(0.0),
    });
}

/// Hold minimum thrust near vertically when the plan runs out mid-air.
fn send_hover(
    ev_tx: &mut EventWriter<rocket::RocketControlInput>,
    entity: Entity,
    min_thrust: f32,
) {
    ev_tx.send(rocket::RocketControlInput {
        entity,
        input_type: rocket::RocketInputType::SetThrust(min_thrust),
    });
    ev_tx.send(rocket::RocketControlInput {
        entity,
        input_type: rocket::RocketInputType::SetPitch(0.0),
    });
    ev_tx.send(rocket::RocketControlInput {
        entity,
        input_type: rocket::RocketInputType::SetYaw(0.0),
    });
}

/// Linearly interpolated state at time `t` into the plan, if in range.
fn plan_state_at(t: f32, traj: &Trajectory) -> Option<APDGSolutionTimeStep> {
    let sol = &traj.sol;
    let dt = sol.dt() as f32;
    if dt <= 0.0 || sol.num_steps() < 2 {
        return None;
    }

    let x = t / dt;
    let k = x.floor();
    if k < 0.0 || k >= (sol.num_steps() - 1) as f32 {
        return None;
    }
    let frac = x - k;
    let (s0, s1) = (&sol.steps()[k as usize], &sol.steps()[k as usize + 1]);
    let f = frac as f64;

    let lerp3 = |a: Vector3<f64>, b: Vector3<f64>| {
        Vector3::new(
            a.x + (b.x - a.x) * f,
            a.y + (b.y - a.y) * f,
            a.z + (b.z - a.z) * f,
        )
    };

    Some(APDGSolutionTimeStep {
        r: lerp3(s0.r, s1.r),
        v: lerp3(s0.v, s1.v),
        a: lerp3(s0.a, s1.a),
        m: s0.m + (s1.m - s0.m) * f,
        t: lerp3(s0.t, s1.t),
        gamma: s0.gamma + (s1.gamma - s0.gamma) * f,
        aR: lerp3(s0.aR, s1.aR),
    })
}
