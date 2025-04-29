//! Simple steepest–descent geometry optimiser built on top of the force-field
//! primitives available in this crate.

use crate::field::ForceField;

/// Parameters controlling the steepest-descent run.
#[derive(Debug, Clone, Copy)]
pub struct SDParams {
    /// Step size (Å) applied along the negative gradient direction.
    pub step_size: f64,
    /// Maximum number of iterations before giving up.
    pub max_iters: usize,
    /// Convergence threshold expressed as RMS gradient (kcal·mol⁻¹·Å⁻¹).
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

/// Finite-difference displacement (central, 2-point) used for numerical
/// gradient evaluation (Å).
const FD_EPS: f64 = 1.0e-4;

/// Run a basic steepest-descent optimisation.  This is a numerical algorithm
/// intended as a temporary scaffold until analytic derivatives are hooked in.
///
/// On success returns the final energy of the system (kcal/mol).
pub fn steepest_descent(ff: &mut ForceField, params: SDParams) -> rdkit_core::Result<f64> {
    // Initial energy.
    let mut energy = ff.total_energy()?;

    for _iter in 0..params.max_iters {
        // ∇E by finite differences.
        let grad = numerical_gradient(ff, FD_EPS)?;

        // Root-mean-square magnitude of the gradient.
        let mut sum_sq = 0.0;
        for g in &grad {
            sum_sq += g.0 * g.0 + g.1 * g.1 + g.2 * g.2;
        }
        let rms = (sum_sq / grad.len() as f64).sqrt();
        if rms < params.tol {
            return Ok(energy);
        }

        // Backup current coordinates to allow rollback if the energy goes up.
        let backup_coords: Vec<_> = ff
            .atoms_mut()
            .iter()
            .map(|a| a.coord)
            .collect();

        let mut step = params.step_size;
        let mut accepted = false;

        // Simple backtracking line-search.
        for _retries in 0..10 {
            // Apply trial step.
            for (i, (atom, g)) in ff.atoms_mut().iter_mut().zip(grad.iter()).enumerate() {
                atom.coord.0 = backup_coords[i].0 - step * g.0;
                atom.coord.1 = backup_coords[i].1 - step * g.1;
                atom.coord.2 = backup_coords[i].2 - step * g.2;
            }

            let new_energy = ff.total_energy()?;
            if new_energy < energy {
                energy = new_energy;
                accepted = true;
                break;
            }

            step *= 0.5; // reduce step size and retry
        }

        if !accepted {
            // Could not find downhill direction — assume convergence.
            return Ok(energy);
        }
    }

    Ok(energy)
}

// ---------------------------------------------------------------------------
// Gradient helper -----------------------------------------------------------

/// Central finite-difference gradient for all atoms.
fn numerical_gradient(ff: &mut ForceField, h: f64) -> rdkit_core::Result<Vec<geometry::Point3D>> {
    use geometry::Point3D;

    let mut grad = Vec::with_capacity(ff.atom_count());

    for idx in 0..ff.atom_count() {
        let gx = fd_component(ff, idx, Axis::X, h)?;
        let gy = fd_component(ff, idx, Axis::Y, h)?;
        let gz = fd_component(ff, idx, Axis::Z, h)?;
        grad.push(Point3D(gx, gy, gz));
    }

    Ok(grad)
}

/// Cartesian axis enumeration.
enum Axis {
    X,
    Y,
    Z,
}

/// Single coordinate finite-difference derivative.
fn fd_component(
    ff: &mut ForceField,
    atom_idx: usize,
    axis: Axis,
    h: f64,
) -> rdkit_core::Result<f64> {
    // Helper closure to grab mutable reference to the chosen coordinate.
    fn coord_mut<'a>(p: &'a mut geometry::Point3D, axis: &Axis) -> &'a mut f64 {
        match axis {
            Axis::X => &mut p.0,
            Axis::Y => &mut p.1,
            Axis::Z => &mut p.2,
        }
    }

    // Forward displacement.
    // Backup, forward displacement -------------------------------------------------
    let orig;
    {
        let coord = coord_mut(&mut ff.get_atom_mut(atom_idx).coord, &axis);
        orig = *coord;
        *coord = orig + h;
    }
    let e_plus = ff.total_energy()?;

    // Backward displacement --------------------------------------------------------
    {
        let coord = coord_mut(&mut ff.get_atom_mut(atom_idx).coord, &axis);
        *coord = orig - h;
    }
    let e_minus = ff.total_energy()?;

    // Restore ----------------------------------------------------------------------
    {
        let coord = coord_mut(&mut ff.get_atom_mut(atom_idx).coord, &axis);
        *coord = orig;
    }

    Ok((e_plus - e_minus) / (2.0 * h))
}

// ---------------------------------------------------------------------------
// Tests ---------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn water_relaxes() {
        // Build a distorted water molecule.
        let mut ff = ForceField::default();

        let o = ff.add_atom("O_3", geometry::Point3D(0.0, 0.0, 0.0));
        let h1 = ff.add_atom("H_", geometry::Point3D(1.5, 0.0, 0.0));
        let h2 = ff.add_atom("H_", geometry::Point3D(-0.3, 1.2, 0.0));

        ff.add_bond(o, h1, 1.0);
        ff.add_bond(o, h2, 1.0);
        ff.add_angle(h1, o, h2, 1.0, 1.0);

        let e_initial = ff.total_energy().unwrap();
        assert!(e_initial > 5.0);

        let e_final = steepest_descent(&mut ff, SDParams::default()).unwrap();

        assert!(e_final < e_initial);
        assert_relative_eq!(e_final, 0.0, epsilon = 2.0);
    }
}
