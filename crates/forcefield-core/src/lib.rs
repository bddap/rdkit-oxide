//! Core force-field utilities: bond-stretch and angle-bend energy terms
//!
//! The implementation follows the original RDKit Universal Force-Field (UFF)
//! equations found in `rdkit/Code/ForceField/UFF/*.cpp`.  Only the pieces
//! required for bond-stretch and angle-bend energy evaluation are included for
//! now.  Gradients and more exotic special-case terms (e.g. the ring
//! corrections present in the C++ code) are left for a later stage.

#![warn(clippy::all, rust_2018_idioms)]

use forcefield_uff as uff;
use rdkit_core::{invariant, RdError, Result};


// ---------------------------------------------------------------------------
// Constants identical to those in rdkit C++ `Params.h`
// ---------------------------------------------------------------------------

/// Scaling factor for Pauling bond-order correction (λ)
const LAMBDA: f64 = 0.1332;

/// Electrostatic constant prefactor G (≈ 332.06 kcal·Å / (mol·e²))
const G: f64 = 332.06;

/// Helper: clamp value into the [-1, 1] range to avoid numerical issues in
/// `acos`.
#[inline]
fn clip_to_one(x: f64) -> f64 {
    x.clamp(-1.0, 1.0)
}

// ---------------------------------------------------------------------------
// Atom parameter access helpers
// ---------------------------------------------------------------------------

/// Wrapper around `uff::get()` that turns a missing parameter into an
/// informative error.
fn atom_params(label: &str) -> Result<uff::AtomParams> {
    uff::get(label).ok_or(RdError::Unimplemented("missing UFF atom label"))
}

// ---------------------------------------------------------------------------
// Public API – bond stretch
// ---------------------------------------------------------------------------

/// Calculate the UFF rest length *r₀* for a bond between atoms *i* and *j*.
///
/// Formula (Eq. 1 in the original UFF paper):
/// r₀ = rᵢ + rⱼ + r_BO − r_EN
///
/// with
/// • r_BO = −λ (rᵢ + rⱼ) ln(b)
/// • r_EN = (rᵢ rⱼ (√ξᵢ − √ξⱼ)²) / (ξᵢ rᵢ + ξⱼ rⱼ)
///
/// where `b` is the bond order.
pub fn bond_rest_length(label_i: &str, label_j: &str, bond_order: f64) -> Result<f64> {
    invariant!(bond_order > 0.0, "bond order must be positive");

    let p_i = atom_params(label_i)?;
    let p_j = atom_params(label_j)?;

    let ri = p_i.r1;
    let rj = p_j.r1;

    // Pauling bond-order correction
    let r_bo = -LAMBDA * (ri + rj) * bond_order.ln();

    // O'Keefe & Breese electronegativity correction
    let xi_i = p_i.xi;
    let xi_j = p_j.xi;
    let diff = xi_i.sqrt() - xi_j.sqrt();
    let r_en = (ri * rj * diff * diff) / (xi_i * ri + xi_j * rj);

    Ok(ri + rj + r_bo - r_en)
}

/// Calculate the bond force constant *k_b* (in kcal / mol Å²).
///
/// k_b = 2 G Zᵢ Zⱼ / r₀³  (Eq. 2)
pub fn bond_force_constant(label_i: &str, label_j: &str, r0: f64) -> Result<f64> {
    let p_i = atom_params(label_i)?;
    let p_j = atom_params(label_j)?;

    Ok(2.0 * G * p_i.z1 * p_j.z1 / (r0 * r0 * r0))
}

/// Return the harmonic bond-stretch energy for a pair of atoms at distance `r`.
///
/// E_bond = ½ k_b (r − r₀)²
pub fn bond_stretch_energy(label_i: &str, label_j: &str, bond_order: f64, distance: f64) -> Result<f64> {
    let r0 = bond_rest_length(label_i, label_j, bond_order)?;
    let k_b = bond_force_constant(label_i, label_j, r0)?;
    Ok(0.5 * k_b * (distance - r0).powi(2))
}

// ---------------------------------------------------------------------------
// Public API – angle bend
// ---------------------------------------------------------------------------

/// Internal helper: integer power known at compile time (i32 up to small N)
#[inline]
fn int_pow(mut base: f64, mut exp: usize) -> f64 {
    let mut res = 1.0;
    while exp > 0 {
        if exp & 1 == 1 {
            res *= base;
        }
        base *= base;
        exp >>= 1;
    }
    res
}

/// Compute the angle force constant *k_a* (kcal / mol)
///
/// Implementation is a direct translation of `calcAngleForceConstant()` from
/// RDKit C++.
pub fn angle_force_constant(
    label_i: &str,
    label_j: &str,
    label_k: &str,
    theta0: f64,
    bond_order_ij: f64,
    bond_order_jk: f64,
) -> Result<f64> {
    let pi = atom_params(label_i)?;
    let pk = atom_params(label_k)?;

    // Pre-compute rest bond lengths
    let r_ij = bond_rest_length(label_i, label_j, bond_order_ij)?;
    let r_jk = bond_rest_length(label_j, label_k, bond_order_jk)?;

    // r_ik by cosine law at equilibrium
    let cos_theta0 = theta0.cos();
    let r_ik = (r_ij * r_ij + r_jk * r_jk - 2.0 * r_ij * r_jk * cos_theta0).sqrt();

    let beta = 2.0 * G / (r_ij * r_jk);
    let pre = beta * pi.z1 * pk.z1 / int_pow(r_ik, 5);

    let r_term = r_ij * r_jk;
    let inner = 3.0 * r_term * (1.0 - cos_theta0 * cos_theta0) - r_ik * r_ik * cos_theta0;

    Ok(pre * r_term * inner)
}

/// Calculate the angle-bend energy.
///
/// For now we only implement the *standard* UFF functional form (order = 0):
///
/// E = k_a ( C₀ + C₁ cosθ + C₂ cos2θ )
pub fn angle_bend_energy(
    label_i: &str,
    label_j: &str,
    label_k: &str,
    theta: f64,
    bond_order_ij: f64,
    bond_order_jk: f64,
) -> Result<f64> {
    let theta0 = atom_params(label_j)?.theta0; // preferred angle stored on central atom

    // Force constant
    let k_a = angle_force_constant(label_i, label_j, label_k, theta0, bond_order_ij, bond_order_jk)?;

    // Pre-compute coefficients (same as C++ implementation)
    let sin_theta0 = theta0.sin();
    let cos_theta0 = theta0.cos();
    let c2 = 1.0 / (4.0 * (sin_theta0 * sin_theta0).max(1e-8));
    let c1 = -4.0 * c2 * cos_theta0;
    let c0 = c2 * (2.0 * cos_theta0 * cos_theta0 + 1.0);

    let cos_theta = clip_to_one(theta.cos());
    let sin_theta_sq = 1.0 - cos_theta * cos_theta;
    let cos_2theta = cos_theta * cos_theta - sin_theta_sq; // cos(2θ) identity

    let angle_term = c0 + c1 * cos_theta + c2 * cos_2theta;

    Ok(k_a * angle_term)
}

// ---------------------------------------------------------------------------
// Unit tests – basic sanity checks versus reference C++ values
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn bond_rest_len_and_energy_hc() {
        // Test H–C (single) bond: H_ / C_3, order 1.0
        let r0 = bond_rest_length("H_", "C_3", 1.0).unwrap();
        assert_relative_eq!(r0, 1.09, epsilon = 0.05); // ~1.09 Å according to paper

        // Stretch by 0.1 Å
        let e = bond_stretch_energy("H_", "C_3", 1.0, r0 + 0.1).unwrap();
        // Expect small but positive energy (~ few kcal/mol)
        assert!(e > 0.0);
    }

    #[test]
    fn angle_bend_energy_water() {
        // Water H-O-H angle (~104.5°). Use H_ / O_3 labels.
        let theta = 104.51_f64.to_radians();
        let e = angle_bend_energy("H_", "O_3", "H_", theta, 1.0, 1.0).unwrap();
        // At equilibrium the energy should be ~0
        assert_relative_eq!(e, 0.0, epsilon = 1e-6);

        // Distort to 120°, energy must increase
        let e_distort = angle_bend_energy("H_", "O_3", "H_", 120.0_f64.to_radians(), 1.0, 1.0).unwrap();
        assert!(e_distort > 0.1);
    }
}
