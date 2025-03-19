/// The guidance module is responsible for the actual control of the rocket during descent.
/// It uses the trajectory module to determine a trajectory to follow and then uses PID controllers to control the rocket.
use std::sync::mpsc;

use nalgebra::Vector2;
use thiserror::Error;

/// Error codes returnable from guidence.
#[derive(Error, Debug)]
pub enum GuidanceError {
    /// Error controling the rocket
    #[error("Error controling the rocket.")]
    ControlError,
}

/// Raw command sent from the guidance module to be executed by the rocket.
pub enum GuidanceCommand {
    /// Rocket nozzle angle
    NozzleAngle(Vector2<f64>),
    /// Rocket throttle 0 to 1
    Throttle(f64),
}

/// Manages the guidance of the rocket during descent.
trait Guidance {
    ///Return a channel to receive commands from.
    fn run(&self) -> Result<mpsc::Receiver<GuidanceCommand>, GuidanceError>;
}
