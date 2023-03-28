use bevy::prelude::*;
use std::time::Duration;

#[derive(Resource)]
pub struct GestureResource {
    pub last_click_time: Duration,
    pub cooldown: Timer,
}

pub enum GestureState {
    DoubleClick,
}

pub struct GestureEvent {
    pub event: GestureState,
    pub button: MouseButton,
}

pub fn double_click_system(
    mut ev_gesture: EventWriter<GestureEvent>,
    time: Res<Time>,
    mut state: ResMut<GestureResource>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    if state.cooldown.tick(time.delta()).just_finished() {
        debug!("Cooldown over");
    }

    if !state.cooldown.finished() {
        return;
    }

    if (mouse_button_input.any_just_pressed([
        MouseButton::Left,
        MouseButton::Right,
        MouseButton::Middle,
    ])) {
        let now = time.elapsed();
        let elapsed = now - state.last_click_time;
        if elapsed < Duration::from_millis(500) {
            debug!("Double click!");

            let button_pressed = match mouse_button_input.get_pressed().next() {
                Some(MouseButton::Left) => MouseButton::Left,
                Some(MouseButton::Right) => MouseButton::Right,
                Some(MouseButton::Middle) => MouseButton::Middle,
                _ => MouseButton::Left,
            };

            ev_gesture.send(GestureEvent {
                event: GestureState::DoubleClick,
                button: button_pressed,
            });
            state.cooldown.set_duration(Duration::from_millis(500));
            state.cooldown.reset();
        }
        state.last_click_time = now;
    }
}

pub struct GesturePlugin;

impl Plugin for GesturePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GestureResource {
            last_click_time: Duration::from_secs(0),
            cooldown: Timer::from_seconds(0.0, TimerMode::Once),
        })
        .add_event::<GestureEvent>()
        .add_system(double_click_system);
    }
}
