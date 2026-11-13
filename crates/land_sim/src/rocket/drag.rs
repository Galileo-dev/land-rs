use crate::prelude::*;
use crate::rocket::control::{EngineControlState, EngineSettings};
use bevy_rapier3d::prelude::ExternalForce;

const RHO: f32 = 1.0;

/// Current aerodynamic drag force on the rocket body [N], world frame.
#[derive(Component, Default, Clone, Copy, Debug)]
pub struct DragForce(pub Vec3);

/// Measure drag from the body velocity. Only writes [`DragForce`]; the actual
/// force applied to Rapier is assembled by [`assemble_forces_system`].
pub fn aerodynamic_drag_system(
    mut commands: Commands,
    mut query: Query<
        (Entity, &Velocity, &rocket::RocketConfig, Option<&mut DragForce>),
        With<rocket::RocketBody>,
    >,
) {
    for (entity, velocity, config, drag_force) in &mut query {
        let linvel = velocity.linvel;
        let speed = linvel.length();

        let drag_vector = if speed > 0.0 {
            let s_d = config.0.s_d as f32;
            let c_d = config.0.c_d as f32;

            let drag_magnitude = 0.5 * RHO * c_d * s_d * speed * speed;
            -linvel / speed * drag_magnitude
        } else {
            Vec3::ZERO
        };

        match drag_force {
            Some(mut drag_force_comp) => {
                drag_force_comp.0 = drag_vector;
            }
            None => {
                commands.entity(entity).insert(DragForce(drag_vector));
            }
        }
    }
}

/// Assemble the net external force on the body: measured drag plus engine
/// thrust.
///
/// Thrust is applied at the body's centre of mass: 250 kN-class forces through
/// the ~1 kg engine body destabilise the gimbal joint, and thrust-vector
/// control is modelled kinematically by the nozzle gimbal — the nozzle's world
/// +Y axis *is* the thrust direction, the body just receives the force.
///
/// Bevy's `ExternalForce` holds its value between frames, so this *assigns*
/// the fully assembled force instead of accumulating.
pub fn assemble_forces_system(
    mut body_q: Query<(&mut ExternalForce, Option<&DragForce>), With<rocket::RocketBody>>,
    engine_q: Query<
        (&GlobalTransform, &EngineControlState, &EngineSettings),
        With<rocket::RocketEngine>,
    >,
) {
    for (mut external_force, drag_force) in &mut body_q {
        let drag = drag_force.map_or(Vec3::ZERO, |drag| drag.0);

        let mut thrust = Vec3::ZERO;
        for (nozzle_tf, control, settings) in &engine_q {
            let thrust_mag = control.thrust.clamp(0.0, settings.max_thrust);
            thrust += nozzle_tf.compute_transform().rotation * Vec3::Y * thrust_mag;
        }

        external_force.force = drag + thrust;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::MinimalPlugins;
    use gfold_rs::rocket_config::RocketConfig as GfoldRocketConfig;

    /// Body with a RocketConfig + ExternalForce, engine with thrust.
    fn test_app() -> (App, Entity, Entity) {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        let gfold_config = GfoldRocketConfig {
            m_dry: 10_000.0,
            m_fuel: 5_000.0,
            t_min_vac: 100_000.0,
            t_max_vac: 250_000.0,
            tdot_min: -100_000.0,
            tdot_max: 100_000.0,
            i_sp: 300.0,
            a_nozzle: 0.5,
            theta_max: 15.0,
            s_d: 10.0,
            c_d: 1.0,
        };

        let body = app
            .world_mut()
            .spawn((
                rocket::RocketBody,
                Velocity::linear(Vec3::new(0.0, -100.0, 0.0)),
                rocket::RocketConfig(gfold_config, 30.6),
                ExternalForce::default(),
            ))
            .id();

        let engine = app
            .world_mut()
            .spawn((
                rocket::RocketEngine,
                Transform::default(),
                GlobalTransform::default(),
                rocket::EngineControlState {
                    thrust: 250_000.0,
                    ..default()
                },
                rocket::EngineSettings {
                    degrees_of_freedom: 15.0,
                    max_thrust: 250_000.0,
                    ..default()
                },
                ExternalForce::default(),
            ))
            .id();

        app.add_systems(
            Update,
            (aerodynamic_drag_system, assemble_forces_system).chain(),
        );

        (app, body, engine)
    }

    #[test]
    fn drag_and_thrust_assemble_without_clobbering() {
        let (mut app, body, _engine) = test_app();

        // Two frames: drag must not accumulate into the body's ExternalForce,
        // and thrust must be assembled onto the body alongside the drag.
        app.update();
        app.update();

        let body_force = app.world().get::<ExternalForce>(body).unwrap().force;
        let drag = app
            .world()
            .get::<DragForce>(body)
            .expect("DragForce must be inserted")
            .0;

        // 0.5 * 1.0 * 1.0 * 10.0 * 100^2 = 50 kN, opposing the velocity.
        assert!(drag.y > 0.0, "drag must oppose downward motion, got {drag}");
        assert!((drag.length() - 50_000.0).abs() < 1.0, "drag magnitude {drag}");
        assert!(
            (body_force - drag - Vec3::new(0.0, 250_000.0, 0.0)).length() < 1.0,
            "body force {body_force} should be drag {drag} + 250 kN thrust"
        );
    }
}
