use bevy::{
    log::LogPlugin,
    prelude::*,
    render::{
        settings::{WgpuFeatures, WgpuSettings},
        RenderPlugin,
    },
};
use bevy_rapier3d::prelude::*;

pub mod cam;
use cam::{PanOrbitCamera, PanOrbitCameraDefaults, PanOrbitCameraPlugin};

pub mod gestures;
use gestures::GesturePlugin;

pub mod event_mapper;
use event_mapper::EventMapperPlugin;
use std::convert::From;

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
                }),
        )
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(GesturePlugin)
        .add_plugin(PanOrbitCameraPlugin)
        .add_plugin(EventMapperPlugin)
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_physics)
        .run();
}

/// set up a simple 3D scene
fn setup_graphics(mut commands: Commands) {}

fn setup_camera(mut commands: Commands) {
    let translation = Vec3::new(-2.0, 2.5, 5.0);
    let radius = translation.length();

    let defaults = PanOrbitCameraDefaults {
        focus: Vec3::ZERO,
        radius,
        upside_down: false,
    };

    let component = PanOrbitCamera::from(&defaults);

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        component,
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

fn spawn_rocket(mut commands: Commands) {

    // a rocket is made up of multiple parts
    // 1. the rocket body. this has no use other than to be a parent to the other parts
    // 2. the rocket nose. this is the part that points forward
    // 3. the rocket engine exhaust. this is the part that points backwards and is the only part that moves

    // the rocket body
}
