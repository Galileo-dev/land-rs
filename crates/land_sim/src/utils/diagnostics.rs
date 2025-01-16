use crate::prelude::*;
use bevy::diagnostic::{
    EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin,
};
use bevy_inspector_egui::quick::{FilterQueryInspectorPlugin, WorldInspectorPlugin};
use iyes_perf_ui::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        // Needed for FPS
        FrameTimeDiagnosticsPlugin,
        EntityCountDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
        // iyes_perf_ui
        PerfUiPlugin,
        // bevy_inspector_egui
        WorldInspectorPlugin::new(),
        FilterQueryInspectorPlugin::<With<rocket::RocketControl>>::default(),
    ))
    .add_systems(Startup, setup);
}

fn setup(mut commands: Commands) {
    commands.spawn(PerfUiAllEntries::default());
}
