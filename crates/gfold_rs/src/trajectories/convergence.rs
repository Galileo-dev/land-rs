//! Container for successive-convexification convergence logs.

/// Position/velocity convergence history (log10 of the per-iteration max differences).
#[derive(Debug, Clone)]
#[allow(non_snake_case)] // `aR` matches the notation used in the APDG formulation.
pub struct ConvergenceHistory {
    /// Position difference per iteration
    pub pos: Vec<f64>,
    /// Velocity difference per iteration
    pub vel: Vec<f64>,
    /// Thrust difference per iteration
    pub thrust: Vec<f64>,
    /// Acceleration relaxation term difference per iteration
    pub aR: Vec<f64>,
}

impl ConvergenceHistory {
    /// Get the length of the history.
    pub fn len(&self) -> usize {
        self.pos.len()
    }

    /// Get the number of iterations.
    pub fn iterations(&self) -> impl Iterator<Item = usize> + '_ {
        1..=self.len()
    }
}
