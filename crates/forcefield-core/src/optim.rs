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

// ---------------------------------------------------------------------------
// Conjugate-gradient optimiser ----------------------------------------------

/// Parameters for the conjugate-gradient optimiser.  Same structure as
/// `SDParams`; kept separate for potential algorithm-specific knobs.
#[derive(Debug, Clone, Copy)]
pub struct CGParams {
    /// Initial step size used by the backtracking line-search (Å).
    pub step_size: f64,
    /// Maximum optimisation iterations.
    pub max_iters: usize,
    /// RMS gradient convergence threshold (kcal·mol⁻¹·Å⁻¹).
    pub tol: f64,
}

impl Default for CGParams {
    fn default() -> Self {
        Self {
            step_size: 0.05,
            max_iters: 600,
            tol: 1e-4,
        }
    }
}

/// Perform a non-linear conjugate-gradient geometry optimisation using the
/// Polak–Ribiere update formula.  Numerical gradients are employed for now.
pub fn conjugate_gradient(ff: &mut ForceField, params: CGParams) -> rdkit_core::Result<f64> {
    use geometry::Point3D;

    // Initial energy and gradient.
    let mut energy = ff.total_energy()?;
    let mut g_prev = ff
        .analytic_gradient()
        .unwrap_or_else(|_| numerical_gradient(ff, FD_EPS).unwrap());

    // Initial search direction is the negative gradient.
    let mut dir: Vec<Point3D> = g_prev.iter().map(|p| Point3D(-p.0, -p.1, -p.2)).collect();

    for _iter in 0..params.max_iters {
        // RMS gradient check.
        if rms_grad(&g_prev) < params.tol {
            return Ok(energy);
        }

        // Backup coordinates.
        let backup_coords: Vec<_> = ff.atoms_mut().iter().map(|a| a.coord).collect();

        // Line search along `dir` (backtracking).
        let mut step = params.step_size;
        let mut new_energy = energy;
        let mut accepted = false;
        for _ls in 0..10 {
            // trial move
            for (i, atom) in ff.atoms_mut().iter_mut().enumerate() {
                atom.coord.0 = backup_coords[i].0 + step * dir[i].0;
                atom.coord.1 = backup_coords[i].1 + step * dir[i].1;
                atom.coord.2 = backup_coords[i].2 + step * dir[i].2;
            }

            let e_trial = ff.total_energy()?;
            if e_trial < energy {
                new_energy = e_trial;
                accepted = true;
                break;
            }

            // reduce step
            step *= 0.5;
        }

        if !accepted {
            // Could not find acceptable step — fall back to steepest descent step.
            dir = g_prev.iter().map(|p| Point3D(-p.0, -p.1, -p.2)).collect();
            continue;
        }

        energy = new_energy;

        // Compute new gradient.
        let g_curr = ff
            .analytic_gradient()
            .unwrap_or_else(|_| numerical_gradient(ff, FD_EPS).unwrap());

        // Polak–Ribiere β = g_k · (g_k − g_{k-1}) / (g_{k-1} · g_{k-1})
        let gg = grad_dot(&g_prev, &g_prev);
        let beta = if gg.abs() < 1e-12 {
            0.0
        } else {
            let mut num = 0.0;
            for (gc, gp) in g_curr.iter().zip(g_prev.iter()) {
                num += gc.0 * (gc.0 - gp.0) + gc.1 * (gc.1 - gp.1) + gc.2 * (gc.2 - gp.2);
            }
            (num / gg).max(0.0) // restart if negative (Fletcher–Reeves safeguard)
        };

        // Update direction: d_{k+1} = -g_k + β d_k
        for (d, g) in dir.iter_mut().zip(g_curr.iter()) {
            d.0 = -g.0 + beta * d.0;
            d.1 = -g.1 + beta * d.1;
            d.2 = -g.2 + beta * d.2;
        }

        g_prev = g_curr;
    }

    Ok(energy)
}

// ---------------------------------------------------------------------------
// High-level convenience wrapper --------------------------------------------

/// Enumeration of the available optimisation algorithms with embedded
/// parameter structs (allowing callers to tweak only what they need).
#[non_exhaustive]
pub enum OptimMethod {
    SteepestDescent(Option<SDParams>),
    ConjugateGradient(Option<CGParams>),
}


impl Default for OptimMethod {
    fn default() -> Self {
        Self::ConjugateGradient(None)
    }
}

/// Optimise a molecular geometry stored in the given `ForceField`.  Returns
/// the final energy.
pub fn optimize_geometry(
    ff: &mut ForceField,
    method: impl Into<OptimMethod>,
) -> rdkit_core::Result<f64> {
    match method.into() {
        OptimMethod::SteepestDescent(p) => steepest_descent(ff, p.unwrap_or_default()),
        OptimMethod::ConjugateGradient(p) => conjugate_gradient(ff, p.unwrap_or_default()),
    }
}

/// Run a basic steepest-descent optimisation.  This is a numerical algorithm
/// intended as a temporary scaffold until analytic derivatives are hooked in.
///
/// On success returns the final energy of the system (kcal/mol).
pub fn steepest_descent(ff: &mut ForceField, params: SDParams) -> rdkit_core::Result<f64> {
    // Initial energy.
    let mut energy = ff.total_energy()?;

    for _iter in 0..params.max_iters {
        // Prefer analytic gradient when available.
        let grad = ff.analytic_gradient().unwrap_or_else(|_| numerical_gradient(ff, FD_EPS).unwrap());

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
// Utility helpers -----------------------------------------------------------

/// Root-mean-square of the gradient vector.
fn rms_grad(g: &[geometry::Point3D]) -> f64 {
    let mut sum_sq = 0.0;
    for p in g {
        sum_sq += p.0 * p.0 + p.1 * p.1 + p.2 * p.2;
    }
    (sum_sq / g.len() as f64).sqrt()
}

/// Dot product between two gradient vectors.
fn grad_dot(a: &[geometry::Point3D], b: &[geometry::Point3D]) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(u, v)| u.0 * v.0 + u.1 * v.1 + u.2 * v.2)
        .sum()
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

    #[test]
    fn water_relaxes_cg() {
        // More severely distorted water to highlight CG efficiency.
        let mut ff = ForceField::default();

        let o = ff.add_atom("O_3", geometry::Point3D(0.0, 0.0, 0.0));
        let h1 = ff.add_atom("H_", geometry::Point3D(2.0, 0.0, 0.0)); // big stretch
        let h2 = ff.add_atom("H_", geometry::Point3D(-0.5, 2.0, 0.0)); // big distortion

        ff.add_bond(o, h1, 1.0);
        ff.add_bond(o, h2, 1.0);
        ff.add_angle(h1, o, h2, 1.0, 1.0);

        let e_initial = ff.total_energy().unwrap();
        assert!(e_initial > 20.0);

        let e_final = conjugate_gradient(&mut ff, CGParams::default()).unwrap();

        // CG should converge to low energy (~0 within a few kcal/mol)
        assert!(e_final < 1.0);
        assert!(e_final < e_initial);
    }

    #[test]
    fn optimize_geometry_wrapper() {
        let mut ff = ForceField::default();
        let o = ff.add_atom("O_3", geometry::Point3D(0.0, 0.0, 0.0));
        let h1 = ff.add_atom("H_", geometry::Point3D(1.4, 0.0, 0.0));
        let h2 = ff.add_atom("H_", geometry::Point3D(-0.2, 1.1, 0.0));

        ff.add_bond(o, h1, 1.0);
        ff.add_bond(o, h2, 1.0);
        ff.add_angle(h1, o, h2, 1.0, 1.0);

        let e0 = ff.total_energy().unwrap();

        let e = optimize_geometry(&mut ff, OptimMethod::default()).unwrap();
        assert!(e < e0);
        assert!(e < 1.0);
    }
}
