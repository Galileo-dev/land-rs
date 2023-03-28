use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

// pub fn diagnostics_system() {}

fn setup_diagnostics(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "FPS: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/Poppins-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "0".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/Poppins-Medium.ttf"),
                            font_size: 20.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..Default::default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(5.0),
                    left: Val::Px(10.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(FpsText);

    commands
        .spawn(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Frame Time: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/Poppins-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "0ms".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/Poppins-Medium.ttf"),
                            font_size: 20.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..Default::default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(20.0),
                    left: Val::Px(10.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(FrameTimeText);
}

fn update_fps(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}

fn update_frame_time(
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<FrameTimeText>>,
) {
    for mut text in &mut query {
        if let Some(frame_time) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME) {
            if let Some(value) = frame_time.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}ms");
            }
        }
    }
}

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct FrameTimeText;

// A unit struct to help identify the color-changing Text component
#[derive(Component)]
struct ColorText;

pub struct DiagnosticsPlugin;

impl Plugin for DiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(setup_diagnostics)
            .add_system(update_fps)
            .add_system(update_frame_time);
    }
}
