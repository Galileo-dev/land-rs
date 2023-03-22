//! Showcases wireframe rendering.

use bevy::app::PluginGroupBuilder;
use bevy::log::LogPlugin;
use bevy::{
    pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    prelude::*,
    render::{camera::Camera, render_resource::WgpuFeatures, settings::WgpuSettings, RenderPlugin},
};

mod cam;
use cam::{pan_orbit_camera, PanOrbitCamera};

mod gestures;
// use gestures::double_click_system;

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
        // setup wireframe rendering
        .add_plugin(WireframePlugin)
        // setup orbital camera
        .add_system(pan_orbit_camera)
        //setup gesture detection
        // .add_system(double_click_system)
        // setup scene
        .add_startup_system(setup)
        // loggin
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut wireframe_config: ResMut<WireframeConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    debug!("Setting up scene...");

    // To draw the wireframe on all entities, set this to 'true'
    wireframe_config.global = true;
    // plane
    // this is going to be a cube that is as flat as a pancake.
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Box::new(10.0, 0.1, 10.0).into()),
        material: materials.add(Color::rgba(0.0, 0.0, 0.0, 0.0).into()),
        ..default()
    });
    // cube
    commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgba(0.0, 0.0, 0.0, 0.0).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    },));
    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    spawn_camera(commands);
}

/// Spawn a camera like this
fn spawn_camera(mut commands: Commands) {
    let translation = Vec3::new(-2.0, 2.5, 5.0);
    let radius = translation.length();

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        PanOrbitCamera {
            radius,
            ..Default::default()
        },
    ));
}

fn spawn_rocket(mut commands: Commands) {

    // a rocket is made up of multiple parts
    // 1. the rocket body. this has no use other than to be a parent to the other parts
    // 2. the rocket nose. this is the part that points forward
    // 3. the rocket engine exhaust. this is the part that points backwards and is the only part that moves

    // the rocket body
}
