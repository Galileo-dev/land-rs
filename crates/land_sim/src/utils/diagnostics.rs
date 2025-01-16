// ====================================================================
// Purpose of this file is to provide a plugin that will display the FPS
// and frame time in the top left corner of the screen. This is useful
// for debugging performance issues. This plugin is not intended to be
// used in production. It is only intended to be used in development.
// ===================================================================
use bevy::{diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin}, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use iyes_perf_ui::prelude::*;

pub struct DiagnosticsPlugin;

impl Plugin for DiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            // Needed for FPS
            FrameTimeDiagnosticsPlugin,
            EntityCountDiagnosticsPlugin,
            SystemInformationDiagnosticsPlugin,

            // iyes_perf_ui
            PerfUiPlugin,

            // bevy_inspector_egui
            WorldInspectorPlugin::new(),
        ))
        .add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(PerfUiAllEntries::default());
}
