#![allow(clippy::needless_pass_by_value)]

pub mod cam;
pub mod error;
pub mod event_mapper;
pub mod gestures;
pub mod guidance;
pub mod metrics;
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
use std::time::Duration;
use bevy_rapier3d::plugin::TimestepMode;

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
        // Clock sync — see setup_physics: physics must advance with the same
        // virtual time the guidance plan is indexed by, without the default
        // 1/60 s per-tick cap that dilates time below 60 fps.
        .insert_resource(TimestepMode::Variable {
            max_dt: 1.0 / 30.0,
            time_scale: 1.0,
            substeps: 1,
        })
        .add_plugins((
            // Custom plugins
            crate::cam::plugin,
            crate::event_mapper::plugin,
            crate::gestures::plugin,
            crate::rocket::plugin,
            crate::utils::diagnostics::plugin,
            crate::guidance::plugin,
            crate::metrics::plugin,
        ))
        .add_systems(Startup, (setup_camera, setup_physics, temp_screenshots))
        .run();
}

use bevy::render::view::screenshot::{save_to_disk, Screenshot};

/// TEMPORARY: capture verification screenshots at two points of the descent.
fn temp_screenshots(mut commands: Commands, time: Res<Time>, mut taken: Local<u8>) {
    let t = time.elapsed_secs();
    if *taken == 0 && t > 6.0 {
        *taken = 1;
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk("/tmp/land_hot.png"));
    }
    if *taken == 1 && t > 20.0 {
        *taken = 2;
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk("/tmp/land_legs.png"));
    }
}

fn setup_camera(mut commands: Commands) {
    // Chase cam: focus is lerped onto the rocket every frame; the radius is
    // close enough to show the hull, legs and plume. Zoom out with the
    // pan-orbit controls for the overview.
    let focus = Vec3::new(350.0, 700.0, 0.0);
    let translation = focus + Vec3::new(90.0, 30.0, 110.0);
    let radius = translation.distance(focus);

    commands.spawn((
        Name::new("Camera"),
        Camera3d::default(),
        Projection::from(PerspectiveProjection {
            fov: 70.0_f32.to_radians(),
            ..default()
        }),
        Transform::from_translation(translation).looking_at(focus, Vec3::Y),
        PanOrbitCamera {
            focus,
            radius,
            upside_down: false,
        },
        RocketCamera,
    ));
}

fn setup_physics(
    mut commands: Commands,
    mut rapier_config: Query<&mut RapierConfiguration>,
    mut virtual_time: ResMut<Time<Virtual>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Match the gravity the guidance solver assumes (it uses the same 9.807).
    //
    // Clock sync: the guidance plan is indexed by Bevy's virtual time, which
    // advances by the real frame delta. The default physics step is capped at
    // 1/60 s per tick (`TimestepMode::Variable { max_dt: 1/60 }`), so below
    // 60 fps physics runs in slow motion and the plan-time vs plant-time
    // desync accumulates until tracking diverges. Raise the physics step cap
    // and clamp virtual time to the same bound so both clocks tick exactly
    // alike, whatever the frame rate (accuracy is fine: dt ≤ 33 ms for a
    // vehicle at ≤ 150 m/s with CCD enabled).
    let max_dt = 1.0 / 30.0;
    virtual_time.set_max_delta(Duration::from_secs_f32(max_dt));
    for mut config in &mut rapier_config {
        config.gravity = Vec3::new(0.0, -9.807, 0.0);
    }

    /* Create the ground. */
    let ground_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.25, 0.27, 0.24),
        ..default()
    });
    commands
        .spawn((
            Mesh3d(meshes.add(Plane3d {
                half_size: Vec2::new(1500.0, 1500.0),
                ..default()
            })),
            MeshMaterial3d(ground_material.clone()),
        ))
        .insert(Collider::cuboid(1000.0, 0.1, 1000.0))
        .insert(Transform::from_xyz(0.0, 0.0, 0.0));
    /* Create a landing pad. */
    commands
        .spawn((
            Mesh3d(meshes.add(Cylinder::new(20.0, 0.05))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.45, 0.12, 0.12),
                ..default()
            })),
        ))
        .insert(Collider::cylinder(0.05, 20.0))
        .insert(Transform::from_xyz(0.0, 0.05, 0.0));
}
