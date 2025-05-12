use nalgebra::Vector3;

use crate::prelude::*;

pub(super) mod diagnostics;
pub(crate) mod prelude {
    pub(crate) use super::uen_to_xyz;
}

/// Conversion function for UEN to XYZ coordinates
pub fn uen_to_xyz(uen: Vector3<f64>) -> Vec3 {
    Vec3::new(uen.y as f32, uen.x as f32, uen.z as f32)
}
