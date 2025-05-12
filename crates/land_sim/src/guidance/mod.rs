use crate::{
    prelude::*,
    rocket::{EngineControlState, EngineSettings},
};
use bevy_rapier3d::na::Vector3;
use gfold_rs::trajectories::{APDGProblemSolver, Settings, SimulationParams};

#[derive(Resource)]
pub struct Trajectory {
    sol: gfold_rs::trajectories::APDGSolution,
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            generate_trajectory.run_if(|traj: Option<Res<Trajectory>>| traj.is_none()),
            follow_trajectory
                .run_if(resource_exists::<Trajectory>)
                .after(generate_trajectory),
            draw_trajectory.run_if(resource_exists::<Trajectory>),
        ),
    );
}

fn generate_trajectory(
    mut commands: Commands,
    rocket_q: Query<
        (
            &Transform,
            &Velocity,
            &ColliderMassProperties,
            &AdditionalMassProperties,
        ),
        With<rocket::RocketRoot>,
    >,
) {
    let Ok((transform, vel, mass_prop, additional_mass_prop)) = rocket_q.get_single() else {
        warn!("generate_trajectory(): rocket not yet spawned – will retry next frame");
        return;
    };

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

    let m_dry = match mass_prop {
        ColliderMassProperties::Mass(m) => *m as f64,
        _ => {
            error!("Failed to get dry mass from ColliderMassProperties");
            10_000.0
        }
    };
    let m_fuel = match additional_mass_prop {
        AdditionalMassProperties::Mass(m) => *m as f64,
        _ => {
            error!("Failed to get fuel mass from AdditionalMassProperties");
            5_000.0
        }
    };
    let m_0 = m_dry + m_fuel;

    let sim_params = SimulationParams::builder()
        .r0(r0)
        .v0(v0)
        .m_0(m_0)
        .m_dry(m_dry)
        .build();

    let settings = Settings::builder().simulation_settings(sim_params).build();

    let (solution, _) = APDGProblemSolver::default()
        .solve(&settings)
        .expect("trajectory generation failed");

    info!("Trajectory generated: {solution:?}");

    commands.insert_resource(Trajectory { sol: solution });
}

fn draw_trajectory(traj: Res<Trajectory>, mut gizmos: Gizmos) {
    let steps = traj.sol.steps();
    for pair in steps.windows(2) {
        let a = &pair[0].r;
        let b = &pair[1].r;

        // Map (U, E, N) -> (Y = up)
        let v_a = utils::uen_to_xyz(*a);
        let v_b = utils::uen_to_xyz(*b);

        gizmos.line(v_a, v_b, Color::rgb(1.0, 0.2, 0.2));

        // Add the thrust vector
        let thrust: Vec3 = utils::uen_to_xyz(pair[0].t);
        let thrust_start = v_a;
        let thrust_end = v_a + (thrust / 10.0);
        gizmos.line(thrust_start, thrust_end, Color::rgb(0.2, 1.0, 0.2));
        gizmos.sphere(v_a, 0.5, Color::rgb(0.2, 0.2, 1.0));
    }
}

fn follow_trajectory(
    time: Res<Time>,
    traj: Res<Trajectory>,
    mut ev_tx: EventWriter<rocket::RocketControlInput>,
    engine_q: Query<(Entity, &EngineSettings), With<EngineControlState>>,
    body_q: Query<&GlobalTransform, With<rocket::RocketRoot>>,
    mut gizmos: Gizmos,
) {
    let Ok((engine_ent, eng_set)) = engine_q.get_single() else {
        return;
    };
    let Ok(body_tf) = body_q.get_single() else {
        return;
    };

    let dt = traj.sol.dt();
    if dt <= 0.0 {
        return;
    }

    let elapsed_time = time.elapsed_secs_f64();
    let step_idx = ((elapsed_time / dt) as usize).min(traj.sol.num_steps() - 1);
    let step = &traj.sol.steps()[step_idx];

    let desired_dir_global = utils::uen_to_xyz(step.t).normalize();

    // Visualise desired direction
    let rocket_pos = body_tf.translation();
    gizmos.line(
        rocket_pos,
        rocket_pos + desired_dir_global * 5.0,
        Color::rgb(0.0, 1.0, 0.0),
    );
    let rocket_up = body_tf.up();

    // Rocket up direction
    gizmos.line(
        rocket_pos,
        rocket_pos + rocket_up * 5.0,
        Color::rgb(1.0, 0.0, 0.0),
    );

    let rocket_up_global = body_tf.up().as_vec3();

    let rotation = Quat::from_rotation_arc(rocket_up_global, desired_dir_global);

    // (No Roll, pitch around X, yaw around Z)
    let (_, desired_pitch, desired_yaw) = rotation.to_euler(EulerRot::YXZ);

    // Draw the yaw and pitch angles
    gizmos.line(
        rocket_pos,
        rocket_pos + Vec3::X * desired_pitch * 5.0,
        Color::rgb(0.0, 0.0, 1.0),
    );

    gizmos.line(
        rocket_pos,
        rocket_pos + Vec3::Z * desired_yaw * 5.0,
        Color::rgb(1.0, 1.0, 0.0),
    );

    let desired_pitch_ratio = desired_pitch / eng_set.degrees_of_freedom;
    let desired_yaw_ratio = desired_yaw / eng_set.degrees_of_freedom;

    ev_tx.send(rocket::RocketControlInput {
        entity: engine_ent,
        input_type: rocket::RocketInputType::SetThrust(step.t.magnitude() as f32),
    });

    ev_tx.send(rocket::RocketControlInput {
        entity: engine_ent,
        input_type: rocket::RocketInputType::SetPitch(-desired_pitch_ratio),
    });

    ev_tx.send(rocket::RocketControlInput {
        entity: engine_ent,
        input_type: rocket::RocketInputType::SetYaw(-desired_yaw_ratio),
    });
}
