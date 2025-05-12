use crate::prelude::*;

use super::object::{RocketBody, RocketEngine};

#[derive(Event)]
pub struct RocketControlInput {
    pub entity: Entity,
    pub input_type: RocketInputType,
}

#[derive(Debug, Clone, Copy)]
pub enum RocketInputType {
    // Incremental controls
    ThrustIncrease,
    ThrustDecrease,
    PitchUp,
    PitchDown,
    YawLeft,
    YawRight,
    RollLeft,
    RollRight,

    /// Absolute thrust in Newtons
    SetThrust(f32),

    /// Pitch ratio [-1, 1]
    SetPitch(f32),

    /// Yaw ratio [-1, 1]
    SetYaw(f32),

    /// Roll ratio [-1, 1]
    SetRoll(f32),
}
#[derive(Component, Default, PartialEq, Debug, Reflect, Clone)]
pub struct EngineSettings {
    pub degrees_of_freedom: Real,
    pub max_thrust: Real,
    pub motor_max_force: Real,
    pub motor_stiffness: Real,
    pub motor_damping: Real,
    pub delta_angle: Real,
    pub delta_thrust: Real,
}

#[derive(Component, Default, PartialEq, Debug, Reflect, Clone)]
pub struct EngineControlState {
    pub thrust: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}

#[derive(Component, Default, PartialEq, Debug, Reflect, Clone)]
#[require(EngineSettings, EngineControlState)]
pub struct EngineControl;

pub fn keyboard_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, With<EngineControlState>>,
    mut control_events: EventWriter<RocketControlInput>,
) {
    for entity in query.iter() {
        if keyboard.pressed(KeyCode::ShiftLeft) {
            control_events.send(RocketControlInput {
                entity,
                input_type: RocketInputType::ThrustIncrease,
            });
        }
        if keyboard.pressed(KeyCode::ControlLeft) {
            control_events.send(RocketControlInput {
                entity,
                input_type: RocketInputType::ThrustDecrease,
            });
        }
        if keyboard.pressed(KeyCode::KeyW) {
            control_events.send(RocketControlInput {
                entity,
                input_type: RocketInputType::PitchUp,
            });
        }
        if keyboard.pressed(KeyCode::KeyS) {
            control_events.send(RocketControlInput {
                entity,
                input_type: RocketInputType::PitchDown,
            });
        }
        if keyboard.pressed(KeyCode::KeyA) {
            control_events.send(RocketControlInput {
                entity,
                input_type: RocketInputType::YawLeft,
            });
        }
        if keyboard.pressed(KeyCode::KeyD) {
            control_events.send(RocketControlInput {
                entity,
                input_type: RocketInputType::YawRight,
            });
        }
        if keyboard.pressed(KeyCode::KeyQ) {
            control_events.send(RocketControlInput {
                entity,
                input_type: RocketInputType::RollLeft,
            });
        }
        if keyboard.pressed(KeyCode::KeyE) {
            control_events.send(RocketControlInput {
                entity,
                input_type: RocketInputType::RollRight,
            });
        }
    }
}

/// Process the keyboard input and update the rocket's control settings.
pub fn map_input_to_control_system(
    mut events: EventReader<RocketControlInput>,
    mut query: Query<(&mut EngineControlState, &EngineSettings)>,
) {
    for event in events.read() {
        if let Ok((mut control, settings)) = query.get_mut(event.entity) {
            let EngineSettings {
                delta_angle,
                delta_thrust,
                max_thrust,
                degrees_of_freedom,
                ..
            } = *settings;

            let max_angle = degrees_of_freedom;

            match event.input_type {
                RocketInputType::ThrustIncrease => {
                    control.thrust = (control.thrust + delta_thrust).min(max_thrust);
                }
                RocketInputType::ThrustDecrease => {
                    control.thrust = (control.thrust - delta_thrust).max(0.0);
                }
                RocketInputType::PitchUp => {
                    control.pitch = (control.pitch + delta_angle).min(max_angle);
                }
                RocketInputType::PitchDown => {
                    control.pitch = (control.pitch - delta_angle).max(-max_angle);
                }
                RocketInputType::YawLeft => {
                    control.yaw = (control.yaw + delta_angle).min(max_angle);
                }
                RocketInputType::YawRight => {
                    control.yaw = (control.yaw - delta_angle).max(-max_angle);
                }
                RocketInputType::RollLeft => {
                    control.roll = (control.roll + delta_angle).min(max_angle);
                }
                RocketInputType::RollRight => {
                    control.roll = (control.roll - delta_angle).max(-max_angle);
                }
                RocketInputType::SetThrust(t) => control.thrust = t.clamp(0.0, max_thrust),
                RocketInputType::SetPitch(r) => control.pitch = r.clamp(-1.0, 1.0) * max_angle,
                RocketInputType::SetYaw(r) => control.yaw = r.clamp(-1.0, 1.0) * max_angle,
                RocketInputType::SetRoll(r) => control.roll = r.clamp(-1.0, 1.0) * max_angle,
            }
        }
    }
}

fn update_motor_system(
    mut query: Query<(&EngineControlState, &EngineSettings, &mut ImpulseJoint)>,
) {
    for (control, settings, mut joint) in query.iter_mut() {
        let max_angle = settings.degrees_of_freedom.to_radians();
        let pitch = (control.pitch * max_angle).clamp(-max_angle, max_angle);
        let yaw = (control.yaw * max_angle).clamp(-max_angle, max_angle);

        joint.data.as_mut().set_motor_position(
            JointAxis::AngX,
            pitch,
            settings.motor_stiffness,
            settings.motor_damping,
        );
        joint.data.as_mut().set_motor_position(
            JointAxis::AngZ,
            yaw,
            settings.motor_stiffness,
            settings.motor_damping,
        );
    }
}

pub fn apply_thrust_system(
    nozzles: Query<
        (
            &GlobalTransform,
            &EngineControlState,
            &EngineSettings,
            &ImpulseJoint, // to reach the parent body
        ),
        With<RocketEngine>,
    >,
    mut bodies: Query<(&GlobalTransform, &mut ExternalForce), With<RocketBody>>,
) {
    for (nozzle_tf, control, settings, joint) in nozzles.iter() {
        let thrust = control.thrust.clamp(0.0, settings.max_thrust);

        let dir = nozzle_tf.compute_transform().rotation * Vec3::Y;
        let force = dir * thrust;

        if let Ok((body_tf, mut body_force)) = bodies.get_mut(joint.parent) {
            let r = nozzle_tf.translation() - body_tf.translation();
            let torque = r.cross(force);

            body_force.force = force;
            body_force.torque = torque;
        }
    }
}

pub fn debug_thrust_system(
    mut query: Query<(&GlobalTransform, &ExternalForce)>,
    mut gizmos: Gizmos,
) {
    let scale = 10.0;

    for (global_transform, external_force) in query.iter_mut() {
        let world_transform = global_transform.compute_transform();

        let thrust = external_force.force;

        gizmos.line(
            world_transform.translation,
            world_transform.translation + thrust / scale,
            bevy::color::palettes::css::GREEN,
        );
    }
}

// rocket control plugin
pub struct RocketControlPlugin;

impl Plugin for RocketControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RocketControlInput>().add_systems(
            Update,
            (
                keyboard_input_system,
                map_input_to_control_system,
                update_motor_system,
                apply_thrust_system,
                debug_thrust_system,
            )
                .chain(),
        );
    }
}
