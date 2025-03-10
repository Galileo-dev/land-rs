use bevy::color::palettes::css::GREEN;

use crate::prelude::*;

use super::object::{Rocket, RocketEngine};

#[derive(Event)]
pub struct RocketControlInput {
    pub entity: Entity,
    pub input_type: RocketInputType,
}

#[derive(Debug, Clone, Copy)]
pub enum RocketInputType {
    ThrustIncrease,
    ThrustDecrease,
    PitchUp,
    PitchDown,
    YawLeft,
    YawRight,
    RollLeft,
    RollRight,
}

#[derive(Resource)]
pub struct RocketControlSettings {
    pub max_thrust: f32,
    pub min_thrust: f32,
    pub max_angle: f32,
    pub control_sensitivity: f32,
    pub dampening: f32,
}

impl Default for RocketControlSettings {
    fn default() -> Self {
        Self {
            max_thrust: 100.0,
            min_thrust: 0.0,
            max_angle: 1.0,
            control_sensitivity: 0.1,
            dampening: 0.90,
        }
    }
}

#[derive(Component, Default, PartialEq, Debug, Reflect, Clone)]
pub struct RocketControl {
    pub thrust: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}

pub fn keyboard_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, With<RocketControl>>,
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

pub fn dampen_controls_system(
    mut rockets: Query<&mut RocketControl>,
    settings: Res<RocketControlSettings>,
) {
    for mut control in &mut rockets {
        control.pitch *= settings.dampening;
        control.yaw *= settings.dampening;
        control.roll *= settings.dampening;
    }
}

pub fn rocket_control_system(
    mut events: EventReader<RocketControlInput>,
    mut rockets: Query<&mut RocketControl>,
    settings: Res<RocketControlSettings>,
) {
    for event in events.read() {
        if let Ok(mut control) = rockets.get_mut(event.entity) {
            match event.input_type {
                RocketInputType::ThrustIncrease => {
                    control.thrust = settings
                        .control_sensitivity
                        .mul_add(10.0, control.thrust)
                        .min(settings.max_thrust);
                }
                RocketInputType::ThrustDecrease => {
                    control.thrust = settings
                        .control_sensitivity
                        .mul_add(-10.0, control.thrust)
                        .max(settings.min_thrust);
                }
                RocketInputType::PitchUp => {
                    control.pitch =
                        (control.pitch + settings.control_sensitivity).min(settings.max_angle);
                }
                RocketInputType::PitchDown => {
                    control.pitch =
                        (control.pitch - settings.control_sensitivity).max(-settings.max_angle);
                }
                RocketInputType::YawLeft => {
                    control.yaw =
                        (control.yaw + settings.control_sensitivity).min(settings.max_angle);
                }
                RocketInputType::YawRight => {
                    control.yaw =
                        (control.yaw - settings.control_sensitivity).max(-settings.max_angle);
                }
                RocketInputType::RollLeft => {
                    control.roll =
                        (control.roll + settings.control_sensitivity).min(settings.max_angle);
                }
                RocketInputType::RollRight => {
                    control.roll =
                        (control.roll - settings.control_sensitivity).max(-settings.max_angle);
                }
            }
        }
    }
}

fn update_motor_system(
    rockets: Query<&RocketControl>,
    mut nozzles: Query<(Entity, &GlobalTransform, &RocketEngine, &mut ImpulseJoint)>,
) {
    for (nozzle_entity, global_transform, engine, mut joint) in nozzles.iter_mut() {
        let parent = joint.parent;

        if let Ok(control) = rockets.get(parent) {
            // Clamp the control values to prevent extreme angles
            let max_angle = engine.degrees_of_freedom.to_radians();
            let pitch = (control.pitch * max_angle).clamp(-max_angle, max_angle);
            let yaw = (control.yaw * max_angle).clamp(-max_angle, max_angle);

            joint.data.as_mut().set_motor_position(
                JointAxis::AngX,
                pitch,
                engine.motor_stiffness,
                engine.motor_damping,
            );
            joint.data.as_mut().set_motor_position(
                JointAxis::AngZ,
                yaw,
                engine.motor_stiffness,
                engine.motor_damping,
            );
        }
    }
}

pub fn apply_thrust_system(
    mut nozzles: Query<(
        Entity,
        &GlobalTransform,
        &RocketEngine,
        &mut ExternalForce,
        &ImpulseJoint,
    )>,
    rockets: Query<&RocketControl>,
    mut gizmos: Gizmos,
    settings: Res<RocketControlSettings>,
) {
    for (nozzle_entity, global_transform, engine, mut external_force, joint) in nozzles.iter_mut() {
        let parent = joint.parent;

        if let Ok(control) = rockets.get(parent) {
            let thrust = engine.max_thrust * control.thrust / settings.max_thrust;

            let world_transform = global_transform.compute_transform();
            let thrust_direction = world_transform.rotation * Vec3::Y;

            external_force.force = thrust_direction * thrust;

            gizmos.line(
                world_transform.translation,
                world_transform.translation + thrust_direction * thrust / 10.0,
                GREEN,
            );
        }
    }
}

// rocket control plugin
pub struct RocketControlPlugin;

impl Plugin for RocketControlPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RocketControlSettings::default())
            .add_event::<RocketControlInput>()
            .add_systems(
                Update,
                (
                    keyboard_input_system,
                    rocket_control_system,
                    dampen_controls_system,
                    update_motor_system,
                    apply_thrust_system,
                )
                    .chain(),
            );
    }
}
