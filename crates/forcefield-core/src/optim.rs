//! Very simple steepest-descent minimiser for the `ForceField`.

use crate::field::ForceField;

#[derive(Debug, Clone, Copy)]
pub struct SDParams {
    pub step_size: f64,
    pub max_iters: usize,
    pub tol: f64,
}

impl Default for SDParams {
    fn default() -> Self {
        Self {
            step_size: 0.01,
            max_iters: 500,
            tol: 1e-4,
        }
    }
}

/// Perform steepest-descent directly on Cartesian coordinates.
pub fn steepest_descent(ff: &mut ForceField, _params: SDParams) -> Result<f64, rdkit_core::RdError> {
    // Placeholder: no gradients yet – just return current energy.
    ff.total_energy()
}
