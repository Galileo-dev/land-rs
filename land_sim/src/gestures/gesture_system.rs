use bevy::prelude::*;

pub struct GestureEvent;

struct GestureState {
    gestures: Vec<GestureEvent>,
}

impl Plugin for GestureState {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<DoubleLeftClickEvent>()
            .add_system(double_left_click_system.system());
    }
}

fn gesture_system(
    mut state: ResMut<GestureState>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut events: ResMut<Events<GestureEvent>>,
) {
    debug!("gesture_system");
}
