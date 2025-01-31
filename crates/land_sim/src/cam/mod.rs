use crate::prelude::*;

mod pan_orbit_camera;
pub use pan_orbit_camera::{PanOrbitCamera, PanOrbitCameraDefaults, ResetCameraEvent};
mod rocket_camera;
pub use rocket_camera::RocketCamera;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((pan_orbit_camera::plugin, rocket_camera::plugin));
}
