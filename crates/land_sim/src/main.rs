use bevy::{
    log::LogPlugin,
    prelude::*,
    render::{
        settings::{RenderCreation, WgpuFeatures, WgpuSettings},
        RenderPlugin,
    },
    window::PresentMode,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

pub mod cam;
use cam::{PanOrbitCamera, PanOrbitCameraPlugin};

pub mod gestures;
use gestures::GesturePlugin;

pub mod event_mapper;
use event_mapper::EventMapperPlugin;

pub mod rocket;
use rocket::{spawn_rocket, ControlStateUIPlugin, RocketControlPlugin};

pub mod utils;
use utils::DiagnosticsPlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Menu,
    InGame,
}

fn main() {
    App::new()
        // setup renderer
        .add_plugins(
            DefaultPlugins
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        features: WgpuFeatures::POLYGON_MODE_LINE,
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    level: bevy::log::Level::DEBUG,
                    filter: "info,wgpu_core=error,wgpu_hal=error,land_sim=debug".to_string(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Land Sim".to_string(),
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins((
            WorldInspectorPlugin::new(),
            RapierDebugRenderPlugin::default(),
            GesturePlugin,
            PanOrbitCameraPlugin,
            EventMapperPlugin,
            DiagnosticsPlugin,
            ControlStateUIPlugin,
            RocketControlPlugin,
        ))
        .add_systems(Update, (setup_camera, setup_physics, spawn_rocket))
        .run();
}

fn setup_camera(mut commands: Commands) {
    let translation = Vec3::new(-2.0, 2.5, 5.0);
    let radius = translation.length();

    commands.spawn(PanOrbitCamera {
        focus: Vec3::ZERO,
        radius,
        upside_down: false,
    });
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(Transform::from_xyz(0.0, -2.0, 0.0));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(Transform::from_xyz(0.0, 4.0, 0.0));
}
