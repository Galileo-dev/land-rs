
mod gesture_system;
use std::time::Duration;

use gesture_system::{double_click_system, GestureEvent, GestureResource, GestureState};

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
        app.insert_resource(GestureResource {
            last_click_time: Duration::from_secs(0),
            cooldown: Timer::from_seconds(0.0, TimerMode::Once),
        })
        .add_event::<GestureEvent>()
        .add_systems(Update, double_click_system);
}
