use bevy::{
    log::LogPlugin,
    prelude::*,
    render::{
        settings::{WgpuFeatures, WgpuSettings},
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
                    wgpu_settings: WgpuSettings {
                        features: WgpuFeatures::POLYGON_MODE_LINE,
                        ..default()
                    },
                })
                .set(LogPlugin {
                    level: bevy::log::Level::DEBUG,
                    filter: "info,wgpu_core=error,wgpu_hal=error,land_sim=debug".to_string(),
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Land Sim".to_string(),
                        present_mode: PresentMode::AutoVsync,
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(GesturePlugin)
        .add_plugin(PanOrbitCameraPlugin)
        .add_plugin(EventMapperPlugin)
        .add_plugin(DiagnosticsPlugin)
        .add_plugin(ControlStateUIPlugin)
        .add_plugin(RocketControlPlugin)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_physics)
        .add_startup_system(spawn_rocket)
        .run();
}

fn setup_camera(mut commands: Commands) {
    let translation = Vec3::new(-2.0, 2.5, 5.0);
    let radius = translation.length();

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius,
            upside_down: false,
        },
    ));
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));
}
