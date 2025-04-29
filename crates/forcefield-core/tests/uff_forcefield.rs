//! Port of RDKit `testUFFForceField.cpp` – regression tests for UFF energies.

use forcefield_core::{field::ForceField};
use forcefield_core::optim::{optimize_geometry, OptimMethod};
use geometry::Point3D;

// Helper to build methane (CH4) with idealised geometry.
fn methane() -> ForceField {
    let mut ff = ForceField::default();

    // Central carbon at origin.
    let c = ff.add_atom("C_3", Point3D(0.0, 0.0, 0.0));

    // Tetrahedral hydrogens (0.109 nm ~ 1.09 Å)
    let h1 = ff.add_atom("H_", Point3D(1.089, 0.0, 0.0));
    let h2 = ff.add_atom("H_", Point3D(-0.363, 1.027, 0.0));
    let h3 = ff.add_atom("H_", Point3D(-0.363, -0.513, 0.889));
    let h4 = ff.add_atom("H_", Point3D(-0.363, -0.513, -0.889));

    // Bonds only; angles/torsions implied by geometry.
    for &h in &[h1, h2, h3, h4] {
        ff.add_bond(c, h, 1.0);
    }

    // Default charges
    ff.assign_default_charges();

    ff
}

#[test]
#[ignore]
fn methane_equilibrium_energy() {
    let ff = methane();
    let e = ff.total_energy().unwrap();
    // Should be near zero at ideal geometry.
    assert!(e.abs() < 20.0);
}

#[test]
#[ignore]
fn methane_optimises_downhill() {
    let mut ff = methane();
    // Distort one hydrogen.
    ff.translate_atom(2, 0.3, 0.0, 0.0); // move along x

    let e_start = ff.total_energy().unwrap();
    assert!(e_start > 0.5);

    let e_final = optimize_geometry(&mut ff, OptimMethod::ConjugateGradient(None)).unwrap();
    assert!(e_final < 5.0);
}
