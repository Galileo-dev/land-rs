// ====================================================================
// Purpose of this file is to provide a plugin that will display the FPS
// and frame time in the top left corner of the screen. This is useful
// for debugging performance issues. This plugin is not intended to be
// used in production. It is only intended to be used in development.
// ===================================================================

use bevy::prelude::*;

use super::control::RocketControl;

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
struct ControlStateText;

// A unit struct to help identify the color-changing Text component
#[derive(Component)]
struct ColorText;

fn setup_control_state_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // commands
    //     .spawn(TextBundle {
    //         text: Text {
    //             sections: vec![
    //                 TextSection {
    //                     value: "Yaw: ".to_string(),
    //                     style: TextStyle {
    //                         font: asset_server.load("fonts/Poppins-Bold.ttf"),
    //                         font_size: 20.0,
    //                         color: Color::RED,
    //                     },
    //                 },
    //                 TextSection {
    //                     value: "0".to_string(),
    //                     style: TextStyle {
    //                         font: asset_server.load("fonts/Poppins-Medium.ttf"),
    //                         font_size: 20.0,
    //                         color: Color::WHITE,
    //                     },
    //                 },
    //                 TextSection {
    //                     value: "\nPitch: ".to_string(),
    //                     style: TextStyle {
    //                         font: asset_server.load("fonts/Poppins-Bold.ttf"),
    //                         font_size: 20.0,
    //                         color: Color::RED,
    //                     },
    //                 },
    //                 TextSection {
    //                     value: "0".to_string(),
    //                     style: TextStyle {
    //                         font: asset_server.load("fonts/Poppins-Medium.ttf"),
    //                         font_size: 20.0,
    //                         color: Color::WHITE,
    //                     },
    //                 },
    //                 TextSection {
    //                     value: "\nRoll: ".to_string(),
    //                     style: TextStyle {
    //                         font: asset_server.load("fonts/Poppins-Bold.ttf"),
    //                         font_size: 20.0,
    //                         color: Color::RED,
    //                     },
    //                 },
    //                 TextSection {
    //                     value: "0".to_string(),
    //                     style: TextStyle {
    //                         font: asset_server.load("fonts/Poppins-Medium.ttf"),
    //                         font_size: 20.0,
    //                         color: Color::WHITE,
    //                     },
    //                 },
    //                 TextSection {
    //                     value: "\nThrust: ".to_string(),
    //                     style: TextStyle {
    //                         font: asset_server.load("fonts/Poppins-Bold.ttf"),
    //                         font_size: 20.0,
    //                         color: Color::RED,
    //                     },
    //                 },
    //                 TextSection {
    //                     value: "0".to_string(),
    //                     style: TextStyle {
    //                         font: asset_server.load("fonts/Poppins-Medium.ttf"),
    //                         font_size: 20.0,
    //                         color: Color::WHITE,
    //                     },
    //                 },
    //             ],
    //             ..Default::default()
    //         },
    //         style: Style {
    //             position_type: PositionType::Absolute,
    //             position: UiRect {
    //                 top: Val::Px(10.0),
    //                 right: Val::Px(20.0),
    //                 ..Default::default()
    //             },
    //             ..Default::default()
    //         },
    //         ..Default::default()
    //     })
    //     .insert(ControlStateText);
}
// fn update_control_state_ui(
//     rocket_control: Query<&RocketControl>,
//     mut query: Query<&mut Text, With<ControlStateText>>,
// ) {
//     for mut text in &mut query {
//         if let Some(control) = rocket_control.iter().next() {
//             // Update the value of the yaw section
//             text.sections[1].value = format!("{:.2}", control.yaw);
//             // Update the value of the pitch section
//             text.sections[3].value = format!("{:.2}", control.pitch);
//             // Update the value of the roll section
//             text.sections[5].value = format!("{:.2}", control.roll);
//             // Update the value of the thrust section
//             text.sections[7].value = format!("{:.2}", control.thrust);
//         }
//     }
// }
pub struct ControlStateUIPlugin;

impl Plugin for ControlStateUIPlugin {
    fn build(&self, app: &mut App) {
        // app.add_startup_system(setup_control_state_ui)
        //     .add_system(update_control_state_ui);
    }
}
