//! UFF partial charge lookup helper.
//!
//! UFF uses **Gasteiger–Marsili** charge equilibration as its default, but the
//! original RDKit implementation also provides a small table with *default*
//! point charges (in elementary‐charge units) for common atom‐type labels.
//! These are primarily used in the UFF test‐suite and for molecules lacking a
//! full charge calculation.
//!
//! This module implements the same lookup in Rust.  Unknown labels return
//! `None`, signalling to the caller that some other charge assignment scheme
//! is required.

use phf::phf_map;

static CHARGES: phf::Map<&'static str, f64> = phf_map! {
    // Values taken from RDKit `UFF/Params.cpp` (d_q in `AtomicParams`).
    "H_"  => 0.210,   // hydrogen
    "C_3" => -0.027,  // sp3 carbon
    "C_2" =>  0.000,  // sp2 carbon (aromatic, olefinic)
    "N_3" => -0.171,
    "N_R" => -0.171,
    "O_3" => -0.500,
    "O_R" => -0.500,
    "F_"  => -0.100,
    "Cl"  => -0.100,
    "Br"  => -0.100,
    "I_"  => -0.100,
    "P_3" =>  0.000,
    "S_3" => -0.100,
    "S_6" =>  0.000,
    // ... extend as needed
};

/// Return the default UFF partial charge for the given atom label (if
/// available).  Units: **e** (elementary charge).
pub fn get(label: &str) -> Option<f64> {
    CHARGES.get(label).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_labels() {
        assert_eq!(get("H_"), Some(0.210));
        assert_eq!(get("O_3"), Some(-0.500));
    }

    #[test]
    fn unknown_label() {
        assert_eq!(get("Xe"), None);
    }
}
