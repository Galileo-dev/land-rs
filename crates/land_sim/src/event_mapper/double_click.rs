use crate::prelude::*;

use crate::cam::ResetCameraEvent;
use crate::gestures::{GestureEvent, GestureState};

pub(super) fn mapper(
    mut double_click_events: EventReader<GestureEvent>,
    mut reset_camera_events: EventWriter<ResetCameraEvent>,
) {
    for ev in double_click_events.read() {
        match ev.event {
            GestureState::DoubleClick => match ev.button {
                MouseButton::Middle => {
                    debug!("Reset camera event sent");
                    reset_camera_events.send(ResetCameraEvent);
                }
                MouseButton::Left => {}
                MouseButton::Right => {}
                MouseButton::Back => {}
                MouseButton::Forward => {}
                MouseButton::Other(_) => {}
            },
        }
    }
}
