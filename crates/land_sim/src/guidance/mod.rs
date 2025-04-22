use crate::prelude::*;
use bevy_rapier3d::na::Vector3;
use gfold_rs::trajectories::{APDGProblemSolver, Settings};

#[derive(Resource)]
pub struct Trajectory {
    sol: gfold_rs::trajectories::APDGSolution,
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, generate_trajectory)
        .add_systems(Update, follow_trajectory)
        .add_systems(Update, draw_trajectory);
}

fn generate_trajectory(
    mut commands: Commands,
    rocket_q: Query<(&Transform, &Velocity, &ColliderMassProperties), With<rocket::RocketRoot>>,
) {
    let (transform, vel, mass_prop) = rocket_q.single();

    let r0 = Vector3::new(
        transform.translation.y as f64, // U
        transform.translation.x as f64, // E
        transform.translation.z as f64, // N
    );

    let v0 = Vector3::new(
        vel.linvel.x as f64,
        vel.linvel.y as f64,
        vel.linvel.z as f64,
    );

    // let mass = mass_props;
    let settings = Settings::builder().build();
    let (solution, _) = APDGProblemSolver::default()
        .solve(&settings)
        .expect("trajectory generation failed");
    commands.insert_resource(Trajectory { sol: solution });
}

fn draw_trajectory(traj: Res<Trajectory>, mut gizmos: Gizmos) {
    let steps = traj.sol.steps();
    for pair in steps.windows(2) {
        let a = &pair[0].r;
        let b = &pair[1].r;

        // Map (U, E, N) -> (Y = up)
        let v_a = Vec3::new(a.y as f32, a.x as f32, a.z as f32);
        let v_b = Vec3::new(b.y as f32, b.x as f32, b.z as f32);

        gizmos.line(v_a, v_b, Color::rgb(1.0, 0.2, 0.2));
    }
}

fn follow_trajectory(
    time: Res<Time>,
    traj: Res<Trajectory>,
    mut ev_ctrl: EventWriter<rocket::RocketControlInput>,
    rocket_q: Query<Entity, With<rocket::RocketRoot>>,
) {
    let Some(rocket_entity) = rocket_q.iter().next() else {
        return;
    };

    let t = time.elapsed_secs_f64();
    let dt = traj.sol.dt();
    let k = (t / dt).floor() as usize;
    let next_k = k.min(traj.sol.num_steps() - 1);
    let step = &traj.sol.steps()[next_k];

    // throttle
    ev_ctrl.send(rocket::RocketControlInput {
        entity: rocket_entity,
        input_type: rocket::RocketInputType::ThrustIncrease,
    });
    let throttle = (step.gamma / traj.sol.steps()[0].gamma * 100.).clamp(0., 100.);
    if throttle > 50. {
        ev_ctrl.send(rocket::RocketControlInput {
            entity: rocket_entity,
            input_type: rocket::RocketInputType::ThrustIncrease,
        });
    } else {
        ev_ctrl.send(rocket::RocketControlInput {
            entity: rocket_entity,
            input_type: rocket::RocketInputType::ThrustDecrease,
        });
    }

    // pitch / yaw
    let dir = step.t.normalize();
    let pitch = dir.z.atan2(dir.y); // crude mapping
    let yaw = dir.x.atan2(dir.y);

    if pitch > 0.01 {
        ev_ctrl.send(rocket::RocketControlInput {
            entity: rocket_entity,
            input_type: rocket::RocketInputType::PitchUp,
        });
    } else if pitch < -0.01 {
        ev_ctrl.send(rocket::RocketControlInput {
            entity: rocket_entity,
            input_type: rocket::RocketInputType::PitchDown,
        });
    }

    if yaw > 0.01 {
        ev_ctrl.send(rocket::RocketControlInput {
            entity: rocket_entity,
            input_type: rocket::RocketInputType::YawRight,
        });
    } else if yaw < -0.01 {
        ev_ctrl.send(rocket::RocketControlInput {
            entity: rocket_entity,
            input_type: rocket::RocketInputType::YawLeft,
        });
    }
}
