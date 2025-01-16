// ====================================================================
// Purpose of this file is to provide a plugin that will display the FPS
// and frame time in the top left corner of the screen. This is useful
// for debugging performance issues. This plugin is not intended to be
// used in production. It is only intended to be used in development.
// ===================================================================
use bevy::prelude::*;
use iyes_perf_ui::prelude::*;

fn setup_diagnostics(mut commands: Commands) {
    commands.spawn((
        PerfUiRoot {
            display_labels: false,
            layout_horizontal: true,
            ..default()
        },
        PerfUiEntryFPSWorst::default(),
        PerfUiEntryFPS::default(),
    ));
}

pub struct DiagnosticsPlugin;

impl Plugin for DiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_diagnostics);
    }
}
