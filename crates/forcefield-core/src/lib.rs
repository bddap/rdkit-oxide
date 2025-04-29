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

pub mod field;
pub mod optim;


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

/// Derivative of the harmonic bond-stretch energy with respect to the bond
/// distance *r* (∂E/∂r).
///
/// Using the harmonic form:
///
/// ```text
/// E = ½ k_b (r - r₀)²
/// ```
///
/// the analytic derivative is
///
/// ```text
/// ∂E/∂r = k_b (r - r₀)
/// ```
///
/// Returned value is in **kcal mol⁻¹ Å⁻¹**.
pub fn bond_stretch_energy_derivative(
    label_i: &str,
    label_j: &str,
    bond_order: f64,
    distance: f64,
) -> Result<f64> {
    let r0 = bond_rest_length(label_i, label_j, bond_order)?;
    let k_b = bond_force_constant(label_i, label_j, r0)?;
    Ok(k_b * (distance - r0))
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

/// Derivative of the angle-bend energy with respect to the angle *θ*
/// (∂E/∂θ, radians⁻¹).
///
/// Given the energy expression implemented in [`angle_bend_energy`]:
///
/// ```text
/// E(θ) = k_a ( C₀ + C₁ cosθ + C₂ cos 2θ )
/// ```
///
/// its analytic derivative w.r.t. the angle is
///
/// ```text
/// ∂E/∂θ = k_a ( -C₁ sinθ - 2 C₂ sin 2θ )
/// ```
///
/// where `sin 2θ = 2 sinθ cosθ`.
pub fn angle_bend_energy_derivative(
    label_i: &str,
    label_j: &str,
    label_k: &str,
    theta: f64,
    bond_order_ij: f64,
    bond_order_jk: f64,
) -> Result<f64> {
    let theta0 = atom_params(label_j)?.theta0;

    // Force constant k_a (same helper as energy function)
    let k_a = angle_force_constant(label_i, label_j, label_k, theta0, bond_order_ij, bond_order_jk)?;

    // Coefficients C₀, C₁, C₂ (duplicate calculation kept in sync with energy version)
    let sin_theta0 = theta0.sin();
    let cos_theta0 = theta0.cos();
    let c2 = 1.0 / (4.0 * (sin_theta0 * sin_theta0).max(1e-8));
    let c1 = -4.0 * c2 * cos_theta0;
    // c0 not needed for derivative

    let cos_theta = clip_to_one(theta.cos());
    let sin_theta = clip_to_one(theta.sin());
    // sin 2θ = 2 sinθ cosθ
    let sin_2theta = 2.0 * sin_theta * cos_theta;

    Ok(k_a * (-c1 * sin_theta - 2.0 * c2 * sin_2theta))
}

// ---------------------------------------------------------------------------
// Public API – torsion / dihedral term
// ---------------------------------------------------------------------------

/// UFF torsional potential uses the OPLS/CHARMM style cosine series:
///
///    E(φ) = ½ * V * (1 + cos(nφ − φ₀))
///
/// For most elements `φ₀` is either 0° or 180°; the literature barrier
/// heights `V` for common combinations (sp³, sp², etc.) will be hooked in
/// later.  For now callers provide `v_barrier`, `periodicity` (*n*) and the
/// equilibrium phase `phi0` in *radians*.
pub fn torsion_energy(v_barrier: f64, periodicity: u32, phi: f64, phi0: f64) -> f64 {
    0.5 * v_barrier * (1.0 - ((periodicity as f64) * phi - phi0).cos())
}

/// Derivative of the torsion energy with respect to the dihedral angle *φ*
/// (∂E/∂φ) – useful for gradient computation.
#[allow(clippy::needless_pass_by_value)]
pub fn torsion_energy_derivative(v_barrier: f64, periodicity: u32, phi: f64, phi0: f64) -> f64 {
    // d/dφ [½ V (1 + cos(nφ − φ₀))] = -½ V n sin(nφ − φ₀)
    0.5 * v_barrier * (periodicity as f64) * ((periodicity as f64) * phi - phi0).sin()
}

// ---------------------------------------------------------------------------
// Public API – inversion (improper torsion / out-of-plane)
// ---------------------------------------------------------------------------

/// Simple harmonic inversion potential used for out-of-plane bending around
/// trigonal centres (improper torsions).
///
/// Formula (harmonic):
/// ```text
/// E = ½ k (χ − χ₀)²
/// ```
/// where `χ` is the out-of-plane angle (*radians*).  Typically `χ₀ = 0` for planar
/// atoms.
#[inline]
pub fn inversion_energy(k_chi: f64, chi: f64, chi0: f64) -> f64 {
    0.5 * k_chi * (chi - chi0).powi(2)
}

/// Derivative ∂E/∂χ for gradient calculations.
#[inline]
pub fn inversion_energy_derivative(k_chi: f64, chi: f64, chi0: f64) -> f64 {
    k_chi * (chi - chi0)
}

// ---------------------------------------------------------------------------
// Public API – van-der-Waals (Lennard-Jones 12-6) term
// ---------------------------------------------------------------------------

/// Compute Lennard-Jones 12-6 potential energy for a pair of atoms.
///
/// ```text
/// E(r) = 4 * epsilon * [ (sigma / r)^12 - (sigma / r)^6 ]
/// ```
///
/// where
/// * `epsilon` – well depth (kcal/mol)
/// * `sigma`   – distance at which the potential crosses zero (Å)
/// * `r`       – inter-atomic distance (Å)
#[inline]
pub fn lj_energy(epsilon: f64, sigma: f64, r: f64) -> f64 {
    let sr6 = (sigma / r).powi(6);
    4.0 * epsilon * (sr6 * sr6 - sr6)
}

/// Derivative ∂E/∂r for the LJ potential.
#[inline]
pub fn lj_energy_derivative(epsilon: f64, sigma: f64, r: f64) -> f64 {
    let sr6 = (sigma / r).powi(6);
    // d/dr E = 4 ε [ -12 σ¹² / r¹³ + 6 σ⁶ / r⁷ ]
    24.0 * epsilon / r * (-2.0 * sr6 * sr6 + sr6)
}

// ---------------------------------------------------------------------------
// Public API – Electrostatic Coulomb term
// ---------------------------------------------------------------------------

/// Vacuum electrostatic constant (kcal·Å / mol·e²).  In AMBER and UFF this is
/// usually quoted as 332.06371.
const K_ELEC: f64 = 332.06371;

/// Coulomb interaction energy between point charges `q_i`, `q_j` separated by
/// distance `r` (Å).  Returns energy in kcal/mol.
#[inline]
pub fn coulomb_energy(q_i: f64, q_j: f64, r: f64) -> f64 {
    K_ELEC * q_i * q_j / r
}

/// Derivative ∂E/∂r (kcal mol⁻¹ Å⁻¹).
#[inline]
pub fn coulomb_energy_derivative(q_i: f64, q_j: f64, r: f64) -> f64 {
    -K_ELEC * q_i * q_j / (r * r)
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

    #[test]
    fn bond_derivative_matches_fd() {
        // Compare analytic derivative to finite-difference for C–O single bond
        let label_i = "C_3";
        let label_j = "O_3";
        let order = 1.0;

        let r0 = bond_rest_length(label_i, label_j, order).unwrap();
        let r = r0 + 0.02; // small stretch

        // Analytic derivative
        let d_ana = bond_stretch_energy_derivative(label_i, label_j, order, r).unwrap();

        // Finite difference derivative (central) with small dr
        let h = 1.0e-5;
        let e_plus = bond_stretch_energy(label_i, label_j, order, r + h).unwrap();
        let e_minus = bond_stretch_energy(label_i, label_j, order, r - h).unwrap();
        let d_fd = (e_plus - e_minus) / (2.0 * h);

        approx::assert_relative_eq!(d_ana, d_fd, epsilon = 1e-6);
    }

    #[test]
    fn angle_derivative_matches_fd() {
        // Water H–O–H angle derivative test
        let label_i = "H_";
        let label_j = "O_3";
        let label_k = "H_";
        let order_ij = 1.0;
        let order_jk = 1.0;

        let theta0 = atom_params(label_j).unwrap().theta0;
        let theta = theta0 + 5_f64.to_radians();

        // Analytic derivative
        let d_ana = angle_bend_energy_derivative(
            label_i,
            label_j,
            label_k,
            theta,
            order_ij,
            order_jk,
        )
        .unwrap();

        // Finite-difference
        let h = 1e-5;
        let e_plus = angle_bend_energy(
            label_i,
            label_j,
            label_k,
            theta + h,
            order_ij,
            order_jk,
        )
        .unwrap();
        let e_minus = angle_bend_energy(
            label_i,
            label_j,
            label_k,
            theta - h,
            order_ij,
            order_jk,
        )
        .unwrap();

        let d_fd = (e_plus - e_minus) / (2.0 * h);

        approx::assert_relative_eq!(d_ana, d_fd, epsilon = 1e-4);
    }

    #[test]
    fn torsion_energy_periodicity() {
        use std::f64::consts::PI;

        let v = 3.0; // kcal/mol
        let n = 3u32;
        let phi0 = 0.0;

        // Energy minima every 120° (2π/3)
        for k in 0..n {
            let angle = (k as f64) * (2.0 * PI / n as f64);
            let e = torsion_energy(v, n, angle, phi0);
            assert_relative_eq!(e, 0.0, epsilon = 1e-12);
            // derivative ~ 0 as well
            let d = torsion_energy_derivative(v, n, angle, phi0);
            assert_relative_eq!(d, 0.0, epsilon = 1e-12);
        }

        // Maximum at halfway between minima (60°)
        let phi_max = PI / 3.0; // 60°
        let e_max = torsion_energy(v, n, phi_max, phi0);
        assert_relative_eq!(e_max, v, epsilon = 1e-12);
    }

    #[test]
    fn inversion_energy_planar() {
        // Planar chi should have zero energy when chi = chi0
        let k = 10.0;
        let chi0 = 0.0;
        let e = inversion_energy(k, chi0, chi0);
        assert_relative_eq!(e, 0.0, epsilon = 1e-12);

        // Distort to 0.2 rad (~11.5°)
        let chi = 0.2;
        let e_distort = inversion_energy(k, chi, chi0);
        assert!(e_distort > 0.0);

        let deriv = inversion_energy_derivative(k, chi, chi0);
        // Derivative should be positive for chi > chi0
        assert!(deriv > 0.0);
    }

    #[test]
    fn lennard_jones_energy_minimum() {
        let epsilon = 0.5; // arbitrary
        let sigma = 3.4;
        // Minimum at r = 2^(1/6) * sigma
        let r_min = sigma * 2_f64.powf(1.0 / 6.0);
        let e_min = lj_energy(epsilon, sigma, r_min);
        assert_relative_eq!(e_min, -epsilon, epsilon = 1e-12);

        // Derivative zero at minimum
        let d = lj_energy_derivative(epsilon, sigma, r_min);
        assert_relative_eq!(d, 0.0, epsilon = 1e-10);

        // Repulsive wall at short distance
        let e_rep = lj_energy(epsilon, sigma, 0.5 * sigma);
        assert!(e_rep > 0.0);
    }
}
