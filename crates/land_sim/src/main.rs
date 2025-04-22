#![allow(clippy::needless_pass_by_value)]

pub mod cam;
pub mod error;
pub mod event_mapper;
pub mod gestures;
pub mod guidance;
pub mod prelude;
pub mod rocket;
pub mod utils;

use bevy::{
    log::LogPlugin,
    render::{
        settings::{RenderCreation, WgpuFeatures, WgpuSettings},
        RenderPlugin,
    },
    window::PresentMode,
};
use cam::{PanOrbitCamera, RocketCamera};
use prelude::*;

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
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .add_plugins((
            // Custom plugins
            crate::cam::plugin,
            crate::event_mapper::plugin,
            crate::gestures::plugin,
            crate::rocket::plugin,
            crate::utils::diagnostics::plugin,
            // crate::guidance::plugin,
        ))
        .add_systems(Startup, (setup_camera, setup_physics))
        .run();
}

fn setup_camera(mut commands: Commands) {
    let translation = Vec3::new(0.0, 0.0, 100.0);
    let radius = translation.length();

    commands.spawn((
        Name::new("Camera"),
        Camera3d::default(),
        Projection::from(PerspectiveProjection {
            fov: 70.0_f32.to_radians(),
            ..default()
        }),
        Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius,
            upside_down: false,
        },
        RocketCamera,
    ));
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(1000.0, 0.1, 1000.0))
        .insert(Transform::from_xyz(0.0, 0.0, 0.0));
    /* Create a landing pad. */
    commands
        .spawn(Collider::cylinder(0.05, 20.0))
        .insert(Transform::from_xyz(0.0, 0.05, 0.0));
}
