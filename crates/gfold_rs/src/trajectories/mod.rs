//! Native Rust implementation of the APDG solver.

#[allow(clippy::module_inception)]
mod apdg;
pub use apdg::*;
use nalgebra::Vector3;
use uom::si::time::second;

/// A thrust vector at a given time.
pub struct ThrustVector {
    /// The T- time when the thrust vector should be applied.
    pub time: second,
    /// The thrust vector to be applied.
    pub thrust: Vector3<f64>,
}

/// A trajectory is just a sequence of thrust vectors.
pub struct Trajectory(pub Vec<ThrustVector>);
