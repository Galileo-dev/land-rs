pub mod error;
pub mod cam;
pub mod gestures;
pub mod event_mapper;
pub mod rocket;
pub mod utils;
pub mod prelude;

use bevy::{log::LogPlugin, render::{settings::{RenderCreation, WgpuFeatures, WgpuSettings}, RenderPlugin}, window::PresentMode};
use cam::PanOrbitCamera;
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
            crate::utils::diagnostics::plugin,
            crate::rocket::plugin,
        ))
        .add_systems(Startup, (setup_camera, setup_physics ))
        .run();
}

fn setup_camera(mut commands: Commands) {
    let translation = Vec3::new(-2.0, 2.5, 5.0);
    let radius = translation.length();

    commands.spawn((
        Name::new("Camera"),
        Camera3d::default(),
        Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
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
        .insert(Transform::from_xyz(0.0, -2.0, 0.0));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(Transform::from_xyz(0.0, 4.0, 0.0));
}
