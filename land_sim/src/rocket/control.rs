use bevy::{
    prelude::*, reflect::erased_serde::__private::serde::__private::de, transform::commands,
};
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::rapier::prelude::ImpulseJointSet;
use bevy_rapier3d::{
    prelude::{ImpulseJoint, RapierConfiguration, RapierImpulseJointHandle, SphericalJoint},
    rapier::prelude::JointMotor,
};

use crate::rocket::object::Rocket;

use super::object::RocketBundle;
use std::collections::HashSet;

// system to control the rocket with the keyboard
// shift : thrust +
// ctrl :  thrust -
// w : pitch +
// s : pitch -
// a : yaw +
// d : yaw -
// q : roll +
// e : roll -

//===================== Temporary =====================
/// todo(): replace these with values with a setting object

static MAX_THRUST: f32 = 100.0;
static MIN_THRUST: f32 = 0.0;

static MAX_PITCH: f32 = 1.0;
static MIN_PITCH: f32 = -1.0;

static MAX_YAW: f32 = 1.0;
static MIN_YAW: f32 = -1.0;

static MAX_ROLL: f32 = 1.0;
static MIN_ROLL: f32 = -1.0;

//===================== Temporary =====================

// a mix of rcs and main thrusters will be used for pitch, yaw and roll
// the main thrusters will be used for thrust
// the nozzle can be rotated to change the direction of the thrust

#[derive(Component)]
pub struct RocketControl {
    pub thrust: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
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

const EPSILON: f32 = 0.0001;

#[derive(Resource)]
pub struct KeyboardState {
    keys_pressed: HashSet<KeyCode>,
}

impl Default for KeyboardState {
    fn default() -> Self {
        Self {
            keys_pressed: HashSet::new(),
        }
    }
}

pub fn keyboard_control_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut RocketControl>,
    mut keyboard_state: ResMut<KeyboardState>,
) {
    // Update the set of currently pressed keys
    keyboard_state.keys_pressed.clear();
    for key_code in keyboard_input.get_pressed() {
        keyboard_state.keys_pressed.insert(*key_code);
    }

    // Update the control values based on the pressed keys
    for mut control in query.iter_mut() {
        if keyboard_state.keys_pressed.contains(&KeyCode::LShift) && control.thrust < MAX_THRUST {
            control.thrust += 0.1;
        }
        if keyboard_state.keys_pressed.contains(&KeyCode::LControl)
            && control.thrust > MIN_THRUST + EPSILON
        {
            control.thrust -= 0.1;
            debug!("thrust: {}", control.thrust);
        }

        if keyboard_state.keys_pressed.contains(&KeyCode::W)
            && control.pitch < MAX_PITCH
            && control.pitch > MIN_PITCH
        {
            control.pitch += 0.1;
        }
        if keyboard_state.keys_pressed.contains(&KeyCode::S)
            && control.pitch < MAX_PITCH
            && control.pitch > MIN_PITCH
        {
            control.pitch -= 0.1;
        }
        if keyboard_state.keys_pressed.contains(&KeyCode::A)
            && control.yaw < MAX_YAW
            && control.yaw > MIN_YAW
        {
            control.yaw += 0.1;
        }
        if keyboard_state.keys_pressed.contains(&KeyCode::D)
            && control.yaw < MAX_YAW
            && control.yaw > MIN_YAW
        {
            control.yaw -= 0.1;
        }
        if keyboard_state.keys_pressed.contains(&KeyCode::Q)
            && control.roll < MAX_ROLL
            && control.roll > MIN_ROLL
        {
            control.roll += 0.1;
        }
        if keyboard_state.keys_pressed.contains(&KeyCode::E)
            && control.roll < MAX_ROLL
            && control.roll > MIN_ROLL
        {
            control.roll -= 0.1;
        }
    }
}
pub fn update_control_system(
    mut query: Query<&mut RocketControl>,
    mut joints: Query<&mut RapierImpulseJointHandle>,
) {
    // gradually reduce the control values back to zero except for thrust
    for mut control in query.iter_mut() {
        control.pitch *= 0.9;
        control.yaw *= 0.9;
        control.roll *= 0.9;
    }

    // update the joints
    // for mut joint in joints.iter_mut() {
    //     let impulse_joint = joint.0;
    //     impulse_joint.
    //     // let mut motor = joint.motor_mut();
    //     // motor.set_desired_velocity(10.0);
    // }
}

fn update_motor(mut query: Query<&mut RapierImpulseJointHandle>) {
    for mut joint in query.iter_mut() {
        let impulse_joint = joint.0;
    }
}

// fn update_motor_system(mut rockets: Query<&mut SphericalJoint>) {
//     for mut rocket in rockets.iter_mut() {
//         let mut motor = rocket
//         motor.set_desired_velocity(10.0);
//     }
// }

// rocket control plugin
pub struct RocketControlPlugin;

impl Plugin for RocketControlPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(KeyboardState::default())
            .add_system(keyboard_control_system)
            .add_system(update_control_system);
        // .add_system(update_motor_system);
    }
}
