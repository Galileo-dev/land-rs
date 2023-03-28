use bevy::prelude::*;
use bevy_rapier3d::{
    prelude::{ImpulseJoint, SphericalJoint},
    rapier::prelude::JointMotor,
};

// system to control the rocket with the keyboard
// shift : thrust +
// ctrl :  thrust -
// w : pitch +
// s : pitch -
// a : yaw +
// d : yaw -
// q : roll +
// e : roll -

// a mix of rcs and main thrusters will be used for pitch, yaw and roll
// the main thrusters will be used for thrust
// the nozzle can be rotated to change the direction of the thrust

#[derive(Component)]
pub struct RocketControl {
    thrust: f32,
    pitch: f32,
    yaw: f32,
    roll: f32,
}

impl Default for RocketControl {
    fn default() -> Self {
        Self {
            thrust: 0.0,
            pitch: 0.0,
            yaw: 0.0,
            roll: 0.0,
        }
    }
}

pub fn keyboard_control_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut RocketControl>,
) {
    for mut control in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::LShift) {
            control.thrust += 0.1;
        }
        if keyboard_input.pressed(KeyCode::LControl) {
            control.thrust -= 0.1;
        }
        if keyboard_input.pressed(KeyCode::W) {
            control.pitch += 0.1;
        }
        if keyboard_input.pressed(KeyCode::S) {
            control.pitch -= 0.1;
        }
        if keyboard_input.pressed(KeyCode::A) {
            control.yaw += 0.1;
        }
        if keyboard_input.pressed(KeyCode::D) {
            control.yaw -= 0.1;
        }
        if keyboard_input.pressed(KeyCode::Q) {
            control.roll += 0.1;
        }
        if keyboard_input.pressed(KeyCode::E) {
            control.roll -= 0.1;
        }
    }
}

// fn update_motor_system(mut joints: Query<(&mut _, &RocketControl)>) {
//     for (mut motor, control) in joints.iter_mut() {
//         // motor.target_velocity = control.thrust;
//         debug!("thrust: {}", control.thrust);
//     }
// }

// rocket control plugin
pub struct RocketControlPlugin;

impl Plugin for RocketControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(keyboard_control_system);
        // .add_system(update_motor_system);
    }
}
