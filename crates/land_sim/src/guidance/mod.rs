use crate::{
    prelude::*,
    rocket::{self, EngineControlState, EngineSettings, RocketConfig},
};
use bevy_rapier3d::na::Vector3;
use gfold_rs::trajectories::{APDGProblemSolver, APDGSolutionTimeStep, Settings, SimulationParams};

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
    rocket_q: Query<(&Transform, &Velocity, &RocketConfig), With<rocket::RocketRoot>>,
) {
    let Ok((transform, vel, rocket_cfg_component)) = rocket_q.get_single() else {
        warn!("generate_trajectory(): rocket not yet spawned or missing RocketConfig – will retry next frame");
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

    let sim_params = rocket_cfg_component.0.to_sim_params(r0, v0);

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

        let v_a = utils::uen_to_xyz(*a);
        let v_b = utils::uen_to_xyz(*b);

        gizmos.line(v_a, v_b, Color::srgb(1.0, 0.2, 0.2));

        let thrust: Vec3 = utils::uen_to_xyz(pair[0].t);
        let thrust_start = v_a;
        let thrust_end = v_a + (thrust / 10.0);
        gizmos.line(thrust_start, thrust_end, Color::srgb(0.2, 1.0, 0.2));
        gizmos.sphere(v_a, 0.5, Color::srgb(0.2, 0.2, 1.0));
    }
}

fn follow_trajectory(
    time: Res<Time>,
    traj: Res<Trajectory>,
    mut ev_tx: EventWriter<rocket::RocketControlInput>,
    engine_q: Query<(Entity, &EngineSettings, &EngineControlState), With<EngineControlState>>,
    body_q: Query<(&GlobalTransform, &Velocity), With<rocket::RocketRoot>>,
    mut gizmos: Gizmos,
) {
    let Ok((engine_ent, eng_set, eng_state)) = engine_q.get_single() else {
        return;
    };
    let Ok((body_tf, body_vel)) = body_q.get_single() else {
        return;
    };

    // Get the step
    let elapsed_time = time.elapsed_secs();
    let step = match step_at_time(elapsed_time, &traj.sol) {
        Some(step) => step,
        None => return,
    };

    let desired_pos = utils::uen_to_xyz(step.r);

    gizmos.line(
        body_tf.translation(),
        desired_pos,
        Color::srgb(0.2, 0.2, 1.0),
    );

    // Send commands
    ev_tx.send(rocket::RocketControlInput {
        entity: engine_ent,
        input_type: rocket::RocketInputType::SetThrust(0.0),
    });
    ev_tx.send(rocket::RocketControlInput {
        entity: engine_ent,
        input_type: rocket::RocketInputType::SetPitch(0.0),
    });
    ev_tx.send(rocket::RocketControlInput {
        entity: engine_ent,
        input_type: rocket::RocketInputType::SetYaw(0.0),
    });
}

fn step_at_time(
    elapsed_time: f32,
    sol: &gfold_rs::trajectories::APDGSolution,
) -> Option<&APDGSolutionTimeStep> {
    let dt = sol.dt() as f32;
    if dt <= 0.0 {
        return None;
    }

    let step_idx = ((elapsed_time / dt) as usize).min(sol.num_steps() - 1);
    Some(&sol.steps()[step_idx])
}
