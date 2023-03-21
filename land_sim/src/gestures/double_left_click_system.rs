use bevy::prelude::*;
use std::time::{Duration, Instant};

pub struct DoubleLeftClickEvent;

struct DoubleLeftClickState {
    last_click_time: Option<Instant>,
}

impl Plugin for DoubleLeftClickState {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<DoubleLeftClickEvent>()
            .add_system(double_left_click_system.system());
    }
}

fn double_left_click_system(
    mut state: ResMut<DoubleLeftClickState>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut events: ResMut<Events<DoubleLeftClickEvent>>,
) {
    let now = Instant::now();
    let double_click_duration = Duration::from_millis(250);

    if mouse_button_input.just_pressed(MouseButton::Left) {
        match state.last_click_time {
            Some(last_click_time) => {
                if now - last_click_time < double_click_duration {
                    events.send(DoubleLeftClickEvent);
                }
            }
            None => {}
        }
        state.last_click_time = Some(now);
    } else if mouse_button_input.just_released(MouseButton::Left) {
        state.last_click_time = None;
    }
}
