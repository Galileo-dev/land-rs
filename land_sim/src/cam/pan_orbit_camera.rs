use bevy::prelude::*;

pub struct DoubleClickEvent(pub MouseButton);

pub struct DoubleClickPlugin {
    button: MouseButton,
}

impl DoubleClickPlugin {
    pub fn new(button: MouseButton) -> Self {
        DoubleClickPlugin { button }
    }
}

impl Plugin for DoubleClickPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<DoubleClickEvent>()
            .add_system(double_click_system.system());
    }
}

fn double_click_system(
    mouse_button_input: Res<Input<MouseButton>>,
    mut events: ResMut<Events<DoubleClickEvent>>,
    button: Res<DoubleClickButton>,
) {
    let now = Instant::now();
    let double_click_duration = Duration::from_millis(250);

    if mouse_button_input.just_pressed(button.0) {
        match button.last_click_time {
            Some(last_click_time) => {
                if now - last_click_time < double_click_duration {
                    events.send(DoubleClickEvent(button.0));
                }
            }
            None => {}
        }
        button.last_click_time = Some(now);
    } else if mouse_button_input.just_released(button.0) {
        button.last_click_time = None;
    }
}

struct DoubleClickButton {
    last_click_time: Option<Instant>,
    button: MouseButton,
}

impl DoubleClickButton {
    pub fn new(button: MouseButton) -> Self {
        DoubleClickButton {
            last_click_time: None,
            button,
        }
    }
}

impl FromWorld for DoubleClickButton {
    fn from_world(world: &mut World) -> Self {
        let button = world
            .get_resource::<DoubleClickPlugin>()
            .map(|plugin| plugin.button)
            .unwrap_or(MouseButton::Left);

        DoubleClickButton::new(button)
    }
}
