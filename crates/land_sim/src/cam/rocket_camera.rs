use crate::{prelude::*, rocket::Rocket};

use super::PanOrbitCamera;

#[derive(Component)]
pub struct RocketCamera;

pub fn follow_rocket_system(
    rocket_query: Query<&Transform, With<Rocket>>,
    mut camera_query: Query<&mut PanOrbitCamera, With<RocketCamera>>,
) {
    if let Ok(rocket_transform) = rocket_query.get_single() {
        if let Ok(mut cam_orbit) = camera_query.get_single_mut() {
            let target_focus = rocket_transform.translation;
            cam_orbit.focus = cam_orbit.focus.lerp(target_focus, 0.1);
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (follow_rocket_system, follow_rocket_system));
}
