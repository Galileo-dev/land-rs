use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::render::camera::Projection;

/// Tags an entity as capable of panning and orbiting.
#[derive(Component, Clone, Debug)]
pub struct PanOrbitCamera {
    /// The "focus point" to orbit around. It is automatically updated when panning the camera
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

#[derive(Resource)]
pub struct PanOrbitCameraDefaults {
    /// The "focus point" to orbit around. It is automatically updated when panning the camera
    pan_orbit: PanOrbitCamera,
    transform: Transform,
    projection: Projection,
}

impl Default for PanOrbitCameraDefaults {
    fn default() -> Self {
        Self {
            pan_orbit: PanOrbitCamera {
                focus: Vec3::ZERO,
                radius: 5.0,
                upside_down: false,
            },
            transform: Transform::default(),
            projection: Projection::default(),
        }
    }
}

#[derive(Event)]
pub struct ResetCameraEvent;

pub fn reset_camera(
    mut query: Query<(&mut PanOrbitCamera, &mut Transform, &mut Projection)>,
    mut default_storage: ResMut<PanOrbitCameraDefaults>,
    mut ev_reset: EventReader<ResetCameraEvent>,
) {
    for ev in ev_reset.read() {
        for (mut pan_orbit, mut transform, projection) in query.iter_mut() {
            *pan_orbit = default_storage.pan_orbit.clone();
            *transform = default_storage.transform.clone();
        }
    }
}

// first we need to store the defaults when the component is added
pub fn set_default(
    mut query: Query<(&mut PanOrbitCamera, &mut Transform, &Projection), Added<PanOrbitCamera>>,
    mut default_storage: ResMut<PanOrbitCameraDefaults>,
) {
    for (mut pan_orbit, mut transform, projection) in query.iter_mut() {
        *default_storage = PanOrbitCameraDefaults {
            pan_orbit: pan_orbit.clone(),
            transform: transform.clone(),
            projection: projection.clone(),
        }
    }
}

/// Pan the camera with middle mouse click, zoom with scroll wheel, orbit with right mouse click.
pub fn pan_orbit_camera(
    windows: Query<&mut Window>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut PanOrbitCamera, &mut Transform, &Projection)>,
) {
    // change input mapping for orbit and panning here
    let orbit_button = MouseButton::Left;
    let pan_button = MouseButton::Middle;

    let mut pan = Vec2::ZERO;
    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.0;
    let mut orbit_button_changed = false;

    if input_mouse.pressed(orbit_button) {
        for ev in ev_motion.read() {
            rotation_move += ev.delta;
        }
    } else if input_mouse.pressed(pan_button) {
        // Pan only if we're not rotating at the moment
        for ev in ev_motion.read() {
            pan += ev.delta;
        }
    }
    for ev in ev_scroll.read() {
        scroll += ev.y;
    }
    if input_mouse.just_released(orbit_button) || input_mouse.just_pressed(orbit_button) {
        orbit_button_changed = true;
    }

    for (mut pan_orbit, mut transform, projection) in query.iter_mut() {
        if orbit_button_changed {
            let up = transform.rotation * Vec3::Y;
            pan_orbit.upside_down = up.y <= 0.0;
        }

        let mut any = false;

        if rotation_move.length_squared() > 0.0 {
            any = true;
            let window = get_primary_window_size(&windows);
            let delta_x = {
                let delta = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
                if pan_orbit.upside_down {
                    -delta
                } else {
                    delta
                }
            };
            let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation; // rotate around global y axis
            transform.rotation = transform.rotation * pitch; // rotate around local x axis
        } else if pan.length_squared() > 0.0 {
            any = true;
            // make panning distance independent of resolution and FOV,
            let window = get_primary_window_size(&windows);
            if let Projection::Perspective(projection) = projection {
                pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window;
            }
            // translate by local axes
            let right = transform.rotation * Vec3::X * -pan.x;
            let up = transform.rotation * Vec3::Y * pan.y;
            // make panning proportional to distance away from focus point
            let translation = (right + up) * pan_orbit.radius;
            pan_orbit.focus += translation;
        } else if scroll.abs() > 0.0 {
            any = true;
            let delta = scroll * ((pan_orbit.radius * 0.8) / pan_orbit.radius);
            pan_orbit.radius -= delta;
            // don't allow zoom to reach zero or you get stuck
            pan_orbit.radius = pan_orbit.radius.max(0.01).max(10.0);
        }

        let rot_matrix = Mat3::from_quat(transform.rotation);
        let target_pos =
            pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));

        if any || transform.translation != target_pos {
            transform.translation = target_pos;
        }
    }

    // consume any remaining events, so they don't pile up if we don't need them
    // (and also to avoid Bevy warning us about not checking events every frame update)
    ev_motion.clear();
}

fn get_primary_window_size(windows: &Query<&mut Window>) -> Vec2 {
    let window = windows.single();
    let window = Vec2::new(window.width() as f32, window.height() as f32);
    window
}

pub fn plugin(app: &mut App) {
    app.add_event::<ResetCameraEvent>()
        .insert_resource(PanOrbitCameraDefaults::default())
        .add_systems(Update, (reset_camera, pan_orbit_camera, set_default));
}
