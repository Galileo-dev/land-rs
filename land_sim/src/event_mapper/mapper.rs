use bevy::prelude::*;

use crate::cam::ResetCameraEvent;
use crate::gestures::{GestureEvent, GestureState};

fn double_click_mapper(
    mut double_click_events: EventReader<GestureEvent>,
    mut reset_camera_events: EventWriter<ResetCameraEvent>,
) {
    for ev in double_click_events.iter() {
        match ev.event {
            GestureState::DoubleClick => match ev.button {
                MouseButton::Middle => {
                    reset_camera_events.send(ResetCameraEvent);
                }
                MouseButton::Left => {}
                MouseButton::Right => {}
                MouseButton::Other(_) => {}
            },
        }
    }
}

pub struct EventMapperPlugin;

impl Plugin for EventMapperPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(double_click_mapper);
    }
}
