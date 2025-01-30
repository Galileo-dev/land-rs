use crate::rocket;

use super::object::Rocket;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
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

#[derive(Component, Reflect)]
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
    keyboard_input: Res<ButtonInput<KeyCode>>,
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
        if keyboard_state.keys_pressed.contains(&KeyCode::ShiftLeft) && control.thrust < MAX_THRUST
        {
            control.thrust += 0.1;
        }
        if keyboard_state.keys_pressed.contains(&KeyCode::ControlLeft)
            && control.thrust > MIN_THRUST + EPSILON
        {
            control.thrust -= 0.1;
            debug!("thrust: {}", control.thrust);
        }

        if keyboard_state.keys_pressed.contains(&KeyCode::KeyW)
            && control.pitch < MAX_PITCH
            && control.pitch > MIN_PITCH
        {
            control.pitch += 0.1;
        }
        if keyboard_state.keys_pressed.contains(&KeyCode::KeyS)
            && control.pitch < MAX_PITCH
            && control.pitch > MIN_PITCH
        {
            control.pitch -= 0.1;
        }
        if keyboard_state.keys_pressed.contains(&KeyCode::KeyA)
            && control.yaw < MAX_YAW
            && control.yaw > MIN_YAW
        {
            control.yaw += 0.1;
        }
        if keyboard_state.keys_pressed.contains(&KeyCode::KeyD)
            && control.yaw < MAX_YAW
            && control.yaw > MIN_YAW
        {
            control.yaw -= 0.1;
        }
        if keyboard_state.keys_pressed.contains(&KeyCode::KeyQ)
            && control.roll < MAX_ROLL
            && control.roll > MIN_ROLL
        {
            control.roll += 0.1;
        }
        if keyboard_state.keys_pressed.contains(&KeyCode::KeyE)
            && control.roll < MAX_ROLL
            && control.roll > MIN_ROLL
        {
            control.roll -= 0.1;
        }
    }
}
pub fn update_control_system(mut rocket_control: Query<&mut RocketControl>) {
    // gradually reduce the control values back to zero except for thrust
    for mut control in rocket_control.iter_mut() {
        control.pitch *= 0.9;
        control.yaw *= 0.9;
        control.roll *= 0.9;
    }
}

fn update_motor(
    rocket_control: Query<&RocketControl>,
    mut nozzle_gimbal: Query<(&mut Transform, &mut ImpulseJoint), With<Rocket>>,
) {
    for (i, (nozzle_transform, mut impulse_joint)) in nozzle_gimbal.iter_mut().enumerate(){
        println!("gimbling nozzle: {i}");
        impulse_joint.data.as_mut().set_motor_position(JointAxis::AngY, rocket_control., 1.0e4, 1.0e3);
    }
}

// rocket control plugin
pub struct RocketControlPlugin;

impl Plugin for RocketControlPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(KeyboardState::default())
            .add_systems(Update, (keyboard_control_system, update_control_system));
    }
}
