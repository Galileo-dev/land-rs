//! Container for successive-convexification convergence logs.

use std::{io::Write, path::Path};

#[derive(Debug, Clone)]
pub struct ConvergenceHistory {
    pub pos: Vec<f64>,
    pub vel: Vec<f64>,
    pub thrust: Vec<f64>,
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
