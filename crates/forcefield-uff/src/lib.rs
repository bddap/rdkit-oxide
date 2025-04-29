//! UFF force-field atomic parameters.
//!
//! Data is generated at build time from the original RDKit C++ table in
//! `rdkit/Code/ForceField/UFF/Params.cpp` and turned into a `phf::Map` for
//! constant-time lookup at runtime.  The generated code is placed in the
//! build directory and `include!`d below.

#![warn(clippy::all, rust_2018_idioms)]

use once_cell::sync::Lazy;

include!(concat!(env!("OUT_DIR"), "/params_generated.rs"));

/// Retrieve atomic parameters by UFF atom label (e.g. "C_3", "O_3_z").
pub fn get(label: &str) -> Option<AtomParams> {
    PARAMS.get(label).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hydrogen_params() {
        let h = get("H_").expect("H_ params");
        // sanity-check a couple of known constants from the C++ table.
        assert!((h.r1 - 0.354).abs() < 1e-6);
        assert!((h.theta0.to_degrees() - 180.0).abs() < 1e-6);
    }
}
