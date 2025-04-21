use crate::prelude::*;

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
pub fn map_keyboard_input_to_control_system(
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
                    control.thrust = settings
                        .delta_angle
                        .mul_add(delta_thrust, control.thrust)
                        .min(max_thrust);
                }
                RocketInputType::ThrustDecrease => {
                    control.thrust = settings
                        .delta_angle
                        .mul_add(-delta_thrust, control.thrust)
                        .max(0.0);
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
    mut nozzles: Query<(
        &GlobalTransform,
        &EngineControlState,
        &EngineSettings,
        &mut ExternalForce,
    )>,
) {
    for (global_transform, control, settings, mut external_force) in nozzles.iter_mut() {
        let thrust = settings.max_thrust * control.thrust / settings.max_thrust;

        let world_transform = global_transform.compute_transform();
        let thrust_direction = world_transform.rotation * Vec3::Y;

        external_force.force = thrust_direction * thrust;
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
                map_keyboard_input_to_control_system,
                update_motor_system,
                apply_thrust_system,
                debug_thrust_system,
            )
                .chain(),
        );
    }
}
