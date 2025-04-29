//! Aggregate `ForceField` struct – stores particles and bonded/non-bonded
//! terms, computes total potential energy by delegating to individual term
//! functions implemented elsewhere in this crate.

use crate::*; // bring helper functions into scope
use geometry::Point3D;

/// Index into the atom list (convenience newtype).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AtomIdx(pub usize);

/// An atom in the force-field context.
#[derive(Debug, Clone)]
pub struct Atom {
    pub label: String,   // UFF label (e.g. "C_3")
    pub coord: Point3D,  // Cartesian Å

    pub charge: f64,     // partial charge (e)
}

// ---------------------------------------------------------------------------
// Unit tests ---------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn water_equilibrium_energy() {
        // Simple water molecule with experimental geometry
        let mut ff = ForceField::default();

        let o = ff.add_atom("O_3", Point3D(0.0, 0.0, 0.0));
        let h1 = ff.add_atom("H_", Point3D(0.9584, 0.0, 0.0));
        let h2 = ff.add_atom("H_", Point3D(-0.2390, 0.9270, 0.0));

        ff.add_bond(o, h1, 1.0);
        ff.add_bond(o, h2, 1.0);
        ff.add_angle(h1, o, h2, 1.0, 1.0);

        let e = ff.total_energy().unwrap();
        // Expect small absolute value (close to 0 at equilibrium)
        assert_relative_eq!(e, 0.0, epsilon = 2.0);
    }

    #[test]
    fn bond_gradients_match_fd() {
        // Simple H–C bond slightly stretched from equilibrium.
        let mut ff = ForceField::default();

        let c = ff.add_atom("C_3", Point3D(0.0, 0.0, 0.0));
        let h = ff.add_atom("H_", Point3D(1.2, 0.0, 0.0)); // ~0.1 Å stretch beyond ~1.09 Å

        ff.add_bond(c, h, 1.0);

        // Analytic bond-only gradients (method under test).
        let g_ana = ff.bond_gradients().unwrap();

        // Finite-difference full gradient (should include only bond term anyway).
        // Numerical finite-difference gradients for comparison (central, 2-point).
        fn numerical_grad(ff: &mut ForceField, h: f64) -> rdkit_core::Result<Vec<Point3D>> {
            use geometry::Point3D;

            enum Axis {
                X,
                Y,
                Z,
            }

            fn coord_mut<'a>(p: &'a mut Point3D, axis: &Axis) -> &'a mut f64 {
                match axis {
                    Axis::X => &mut p.0,
                    Axis::Y => &mut p.1,
                    Axis::Z => &mut p.2,
                }
            }

            fn fd_comp(
                ff: &mut ForceField,
                atom_idx: usize,
                axis: Axis,
                h: f64,
            ) -> rdkit_core::Result<f64> {
                let orig;
                {
                    let coord = coord_mut(&mut ff.get_atom_mut(atom_idx).coord, &axis);
                    orig = *coord;
                    *coord = orig + h;
                }
                let e_plus = ff.total_energy()?;

                {
                    let coord = coord_mut(&mut ff.get_atom_mut(atom_idx).coord, &axis);
                    *coord = orig - h;
                }
                let e_minus = ff.total_energy()?;

                {
                    let coord = coord_mut(&mut ff.get_atom_mut(atom_idx).coord, &axis);
                    *coord = orig;
                }

                Ok((e_plus - e_minus) / (2.0 * h))
            }

            let natoms = ff.atom_count();
            let mut grad = Vec::with_capacity(natoms);
            for idx in 0..natoms {
                let gx = fd_comp(ff, idx, Axis::X, h)?;
                let gy = fd_comp(ff, idx, Axis::Y, h)?;
                let gz = fd_comp(ff, idx, Axis::Z, h)?;
                grad.push(Point3D(gx, gy, gz));
            }

            Ok(grad)
        }

        let g_fd = numerical_grad(&mut ff, 1e-4).unwrap();

        // Compare components.
        for (ga, gf) in g_ana.iter().zip(g_fd.iter()) {
            assert_relative_eq!(ga.0, gf.0, epsilon = 1e-5);
            assert_relative_eq!(ga.1, gf.1, epsilon = 1e-5);
            assert_relative_eq!(ga.2, gf.2, epsilon = 1e-5);
        }
    }

    #[test]
    fn angle_gradients_match_fd() {
        // Water molecule – focus on angle contribution only.
        let mut ff = ForceField::default();

        let o = ff.add_atom("O_3", Point3D(0.0, 0.0, 0.0));
        let h1 = ff.add_atom("H_", Point3D(0.96, 0.0, 0.0));
        let h2 = ff.add_atom("H_", Point3D(-0.24, 0.93, 0.0));

        ff.add_angle(h1, o, h2, 1.0, 1.0);

        // Analytic gradients for angle term.
        let g_ana = ff.angle_gradients().unwrap();

        // Finite-difference gradients (full energy, but only angle term present).
        fn numerical_grad(ff: &mut ForceField, h: f64) -> rdkit_core::Result<Vec<Point3D>> {
            use geometry::Point3D;

            enum Axis {
                X,
                Y,
                Z,
            }

            fn coord_mut<'a>(p: &'a mut Point3D, axis: &Axis) -> &'a mut f64 {
                match axis {
                    Axis::X => &mut p.0,
                    Axis::Y => &mut p.1,
                    Axis::Z => &mut p.2,
                }
            }

            fn fd_comp(
                ff: &mut ForceField,
                atom_idx: usize,
                axis: Axis,
                h: f64,
            ) -> rdkit_core::Result<f64> {
                let orig;
                {
                    let coord = coord_mut(&mut ff.get_atom_mut(atom_idx).coord, &axis);
                    orig = *coord;
                    *coord = orig + h;
                }
                let e_plus = ff.total_energy()?;

                {
                    let coord = coord_mut(&mut ff.get_atom_mut(atom_idx).coord, &axis);
                    *coord = orig - h;
                }
                let e_minus = ff.total_energy()?;

                {
                    let coord = coord_mut(&mut ff.get_atom_mut(atom_idx).coord, &axis);
                    *coord = orig;
                }

                Ok((e_plus - e_minus) / (2.0 * h))
            }

            let natoms = ff.atom_count();
            let mut grad = Vec::with_capacity(natoms);
            for idx in 0..natoms {
                let gx = fd_comp(ff, idx, Axis::X, h)?;
                let gy = fd_comp(ff, idx, Axis::Y, h)?;
                let gz = fd_comp(ff, idx, Axis::Z, h)?;
                grad.push(Point3D(gx, gy, gz));
            }

            Ok(grad)
        }

        let g_fd = numerical_grad(&mut ff, 1e-4).unwrap();

        for (ga, gf) in g_ana.iter().zip(g_fd.iter()) {
            assert_relative_eq!(ga.0, gf.0, epsilon = 2e-4);
            assert_relative_eq!(ga.1, gf.1, epsilon = 2e-4);
            assert_relative_eq!(ga.2, gf.2, epsilon = 2e-4);
        }
    }

    #[test]
    fn torsion_gradients_match_fd() {
        // Simple four-atom system forming a non-degenerate dihedral (butane-like).
        let mut ff = ForceField::default();



        let a = ff.add_atom("C_3", Point3D(0.0, 0.0, 0.0));
        let b = ff.add_atom("C_3", Point3D(1.54, 0.0, 0.0)); // typical C–C bond
        let c = ff.add_atom("C_3", Point3D(2.54, 0.1, 0.0));
        let d = ff.add_atom("C_3", Point3D(3.54, 0.1, 0.1));

        // Add torsion term with arbitrary barrier height.
        let v = 3.0; // kcal/mol
        let n = 3; // threefold periodicity (sp3-sp3)
        let phi0 = 0.0; // equilibrium phase (radians)

        ff.add_torsion(a, b, c, d, v, n as u32, phi0);

        // Analytic gradients for torsion term.
        let g_ana = ff.torsion_gradients().unwrap();

        // Finite-difference gradients of total energy (only torsion present).
        fn numerical_grad(ff: &mut ForceField, h: f64) -> rdkit_core::Result<Vec<Point3D>> {
            enum Axis { X, Y, Z }

            fn coord_mut<'a>(p: &'a mut Point3D, axis: &Axis) -> &'a mut f64 {
                match axis {
                    Axis::X => &mut p.0,
                    Axis::Y => &mut p.1,
                    Axis::Z => &mut p.2,
                }
            }

            fn fd_comp(
                ff: &mut ForceField,
                atom_idx: usize,
                axis: Axis,
                h: f64,
            ) -> rdkit_core::Result<f64> {
                let orig;
                {
                    let coord = coord_mut(&mut ff.get_atom_mut(atom_idx).coord, &axis);
                    orig = *coord;
                    *coord = orig + h;
                }
                let e_plus = ff.total_energy()?;

                {
                    let coord = coord_mut(&mut ff.get_atom_mut(atom_idx).coord, &axis);
                    *coord = orig - h;
                }
                let e_minus = ff.total_energy()?;

                {
                    let coord = coord_mut(&mut ff.get_atom_mut(atom_idx).coord, &axis);
                    *coord = orig;
                }

                Ok((e_plus - e_minus) / (2.0 * h))
            }

            let natoms = ff.atom_count();
            let mut grad = Vec::with_capacity(natoms);
            for idx in 0..natoms {
                let gx = fd_comp(ff, idx, Axis::X, h)?;
                let gy = fd_comp(ff, idx, Axis::Y, h)?;
                let gz = fd_comp(ff, idx, Axis::Z, h)?;
                grad.push(Point3D(gx, gy, gz));
            }

            Ok(grad)
        }

        let g_fd = numerical_grad(&mut ff, 1e-4).unwrap();

        for (ga, gf) in g_ana.iter().zip(g_fd.iter()) {
            assert_relative_eq!(ga.0, gf.0, epsilon = 2e-4);
            assert_relative_eq!(ga.1, gf.1, epsilon = 2e-4);
            assert_relative_eq!(ga.2, gf.2, epsilon = 2e-4);
        }
    }

    #[test]
    fn lj_gradient_matches_fd() {
        // Two argon-like atoms interacting via Lennard-Jones only.
        let mut ff = ForceField::default();

        let a = ff.add_atom("Ar", Point3D(0.0, 0.0, 0.0));
        let b = ff.add_atom("Ar", Point3D(4.0, 0.0, 0.0));

        let epsilon = 0.238; // kcal/mol
        let sigma = 3.405;
        ff.add_lj_pair(a, b, epsilon, sigma);

        let g_ana = ff.lj_gradients();

        // Finite-difference helper --------------------------------------------------
        fn numerical_grad(ff: &mut ForceField, h: f64) -> rdkit_core::Result<Vec<Point3D>> {
            enum Axis { X, Y, Z }

            fn coord_mut<'a>(p: &'a mut Point3D, axis: &Axis) -> &'a mut f64 {
                match axis {
                    Axis::X => &mut p.0,
                    Axis::Y => &mut p.1,
                    Axis::Z => &mut p.2,
                }
            }

            fn fd_comp(ff: &mut ForceField, atom_idx: usize, axis: Axis, h: f64) -> rdkit_core::Result<f64> {
                let orig;
                {
                    let coord = coord_mut(&mut ff.get_atom_mut(atom_idx).coord, &axis);
                    orig = *coord;
                    *coord = orig + h;
                }
                let e_plus = ff.total_energy()?;
                {
                    let coord = coord_mut(&mut ff.get_atom_mut(atom_idx).coord, &axis);
                    *coord = orig - h;
                }
                let e_minus = ff.total_energy()?;
                {
                    let coord = coord_mut(&mut ff.get_atom_mut(atom_idx).coord, &axis);
                    *coord = orig;
                }
                Ok((e_plus - e_minus) / (2.0 * h))
            }

            let natoms = ff.atom_count();
            let mut grad = Vec::with_capacity(natoms);
            for idx in 0..natoms {
                let gx = fd_comp(ff, idx, Axis::X, h)?;
                let gy = fd_comp(ff, idx, Axis::Y, h)?;
                let gz = fd_comp(ff, idx, Axis::Z, h)?;
                grad.push(Point3D(gx, gy, gz));
            }
            Ok(grad)
        }

        let g_fd = numerical_grad(&mut ff, 1e-4).unwrap();

        for (ga, gf) in g_ana.iter().zip(g_fd.iter()) {
            assert_relative_eq!(ga.0, gf.0, epsilon = 1e-3);
            assert_relative_eq!(ga.1, gf.1, epsilon = 1e-3);
            assert_relative_eq!(ga.2, gf.2, epsilon = 1e-3);
        }
    }

    #[test]
    fn inversion_gradient_matches_fd() {
        // Simple improper torsion: central atom at origin, basal plane in xy, out-of-plane atom slightly off.
        let mut ff = ForceField::default();

        use geometry::Point3D;

        let j = ff.add_atom("C_3", Point3D(0.0, 0.0, 0.0));
        let k = ff.add_atom("C_3", Point3D(1.0, 0.0, 0.0));
        let l = ff.add_atom("C_3", Point3D(0.0, 1.0, 0.0));
        let i = ff.add_atom("C_3", Point3D(0.0, 0.0, 0.2)); // out-of-plane

        let k_chi = 5.0;
        let chi0 = 0.0;
        ff.add_inversion(i, j, k, l, k_chi, chi0);

        let g_ana = ff.inversion_gradients();

        // Finite difference
        fn numerical_grad(ff: &mut ForceField, h: f64) -> rdkit_core::Result<Vec<Point3D>> {
            enum Axis { X, Y, Z }
            fn coord_mut<'a>(p: &'a mut Point3D, axis: &Axis) -> &'a mut f64 {
                match axis {
                    Axis::X => &mut p.0,
                    Axis::Y => &mut p.1,
                    Axis::Z => &mut p.2,
                }
            }
            fn fd_comp(ff: &mut ForceField, atom_idx: usize, axis: Axis, h: f64) -> rdkit_core::Result<f64> {
                let orig;
                {
                    let coord = coord_mut(&mut ff.get_atom_mut(atom_idx).coord, &axis);
                    orig = *coord;
                    *coord = orig + h;
                }
                let e_plus = ff.total_energy()?;
                {
                    let coord = coord_mut(&mut ff.get_atom_mut(atom_idx).coord, &axis);
                    *coord = orig - h;
                }
                let e_minus = ff.total_energy()?;
                {
                    let coord = coord_mut(&mut ff.get_atom_mut(atom_idx).coord, &axis);
                    *coord = orig;
                }
                Ok((e_plus - e_minus) / (2.0 * h))
            }
            let natoms = ff.atom_count();
            let mut grad = Vec::with_capacity(natoms);
            for idx in 0..natoms {
                let gx = fd_comp(ff, idx, Axis::X, h)?;
                let gy = fd_comp(ff, idx, Axis::Y, h)?;
                let gz = fd_comp(ff, idx, Axis::Z, h)?;
                grad.push(Point3D(gx, gy, gz));
            }
            Ok(grad)
        }

        let g_fd = numerical_grad(&mut ff, 1e-4).unwrap();

        for (ga, gf) in g_ana.iter().zip(g_fd.iter()) {
            assert_relative_eq!(ga.0, gf.0, epsilon = 1e-3);
            assert_relative_eq!(ga.1, gf.1, epsilon = 1e-3);
            assert_relative_eq!(ga.2, gf.2, epsilon = 1e-3);
        }
    }

    #[test]
    fn coulomb_gradient_matches_fd() {
        // Two point charges +1e and -1e separated by 5 Å.
        let mut ff = ForceField::default();
        let a = ff.add_atom("X", Point3D(0.0, 0.0, 0.0));
        let b = ff.add_atom("Y", Point3D(5.0, 0.0, 0.0));

        ff.add_coulomb_pair(a, b, 1.0, -1.0);

        let g_ana = ff.coulomb_gradients();

        // Numerical finite difference.
        fn numerical_grad(ff: &mut ForceField, h: f64) -> rdkit_core::Result<Vec<Point3D>> {
            enum Axis { X, Y, Z }
            fn coord_mut<'a>(p: &'a mut Point3D, axis: &Axis) -> &'a mut f64 {
                match axis {
                    Axis::X => &mut p.0,
                    Axis::Y => &mut p.1,
                    Axis::Z => &mut p.2,
                }
            }
            fn fd(ff: &mut ForceField, idx: usize, axis: Axis, h: f64) -> rdkit_core::Result<f64> {
                let orig;
                {
                    let c = coord_mut(&mut ff.get_atom_mut(idx).coord, &axis);
                    orig = *c;
                    *c = orig + h;
                }
                let e_plus = ff.total_energy()?;
                {
                    let c = coord_mut(&mut ff.get_atom_mut(idx).coord, &axis);
                    *c = orig - h;
                }
                let e_minus = ff.total_energy()?;
                {
                    let c = coord_mut(&mut ff.get_atom_mut(idx).coord, &axis);
                    *c = orig;
                }
                Ok((e_plus - e_minus) / (2.0 * h))
            }
            let mut g = Vec::new();
            for idx in 0..ff.atom_count() {
                let gx = fd(ff, idx, Axis::X, h)?;
                let gy = fd(ff, idx, Axis::Y, h)?;
                let gz = fd(ff, idx, Axis::Z, h)?;
                g.push(Point3D(gx, gy, gz));
            }
            Ok(g)
        }

        let g_fd = numerical_grad(&mut ff, 1e-4).unwrap();

        for (ga, gf) in g_ana.iter().zip(g_fd.iter()) {
            assert_relative_eq!(ga.0, gf.0, epsilon = 1e-3);
            assert_relative_eq!(ga.1, gf.1, epsilon = 1e-3);
            assert_relative_eq!(ga.2, gf.2, epsilon = 1e-3);
        }
    }

    #[test]
    fn default_charge_assignment() {
        let mut ff = ForceField::default();
        let c = ff.add_atom("C_3", Point3D(0.0, 0.0, 0.0));
        let o = ff.add_atom("O_3", Point3D(1.2, 0.0, 0.0));

        ff.assign_default_charges();

        // Charges should be populated
        assert!((ff.atoms[c.0].charge + 0.027).abs() < 1e-3);
        assert!((ff.atoms[o.0].charge + 0.5).abs() < 1e-3);

        // Coulomb pair list should have one entry
        assert_eq!(ff.coulomb_pairs.len(), 1);
    }
}


/// Bonded term definitions --------------------------------------------------

#[derive(Debug, Clone)]
pub struct Bond {
    a: AtomIdx,
    b: AtomIdx,
    order: f64,
}

#[derive(Debug, Clone)]
pub struct Angle {
    i: AtomIdx,
    j: AtomIdx, // central atom
    k: AtomIdx,
    order_ij: f64,
    order_jk: f64,
}

#[derive(Debug, Clone)]
pub struct Torsion {
    i: AtomIdx,
    j: AtomIdx,
    k: AtomIdx,
    l: AtomIdx,
    v: f64,
    periodicity: u32,
    phi0: f64,
}

#[derive(Debug, Clone)]
pub struct Inversion {
    i: AtomIdx,
    j: AtomIdx,
    k: AtomIdx,
    l: AtomIdx,
    k_chi: f64,
    chi0: f64,
}

#[derive(Debug, Clone)]
pub struct LjPair {
    a: AtomIdx,
    b: AtomIdx,
    epsilon: f64,
    sigma: f64,
}

#[derive(Debug, Clone)]
pub struct CoulombPair {
    a: AtomIdx,
    b: AtomIdx,
    q_a: f64,
    q_b: f64,
}

/// Main container.
#[derive(Default)]
pub struct ForceField {
    atoms: Vec<Atom>,
    bonds: Vec<Bond>,
    angles: Vec<Angle>,
    torsions: Vec<Torsion>,
    inversions: Vec<Inversion>,
    lj_pairs: Vec<LjPair>,

    coulomb_pairs: Vec<CoulombPair>,
}

impl ForceField {
    /// Add atom, returning its index.
    pub fn add_atom(&mut self, label: impl Into<String>, coord: Point3D) -> AtomIdx {
        let idx = AtomIdx(self.atoms.len());
        self.atoms.push(Atom {
            label: label.into(),
            coord,
            charge: 0.0,
        });
        idx
    }

    pub fn add_bond(&mut self, a: AtomIdx, b: AtomIdx, order: f64) {
        self.bonds.push(Bond { a, b, order });
    }

    pub fn add_angle(&mut self, i: AtomIdx, j: AtomIdx, k: AtomIdx, order_ij: f64, order_jk: f64) {
        self.angles.push(Angle {
            i,
            j,
            k,
            order_ij,
            order_jk,
        });
    }

    #[allow(clippy::too_many_arguments)]
    pub fn add_torsion(
        &mut self,
        i: AtomIdx,
        j: AtomIdx,
        k: AtomIdx,
        l: AtomIdx,
        v: f64,
        periodicity: u32,
        phi0: f64,
    ) {
        self.torsions.push(Torsion {
            i,
            j,
            k,
            l,
            v,
            periodicity,
            phi0,
        });
    }

    pub fn add_inversion(
        &mut self,
        i: AtomIdx,
        j: AtomIdx,
        k: AtomIdx,
        l: AtomIdx,
        k_chi: f64,
        chi0: f64,
    ) {
        self.inversions.push(Inversion {
            i,
            j,
            k,
            l,
            k_chi,
            chi0,
        });
    }

    pub fn add_lj_pair(&mut self, a: AtomIdx, b: AtomIdx, epsilon: f64, sigma: f64) {
        self.lj_pairs.push(LjPair { a, b, epsilon, sigma });
    }

    pub fn add_coulomb_pair(&mut self, a: AtomIdx, b: AtomIdx, q_a: f64, q_b: f64) {
        self.coulomb_pairs.push(CoulombPair { a, b, q_a, q_b });
    }

    /// Assign default UFF partial charges (if available) and populate the
    /// Coulomb pair list for **all** atom pairs.  Existing `coulomb_pairs` are
    /// cleared beforehand.
    pub fn assign_default_charges(&mut self) {
        use forcefield_uff::charge as uff_charge;

        // Set charges on atoms.
        for atom in &mut self.atoms {
            if let Some(q) = uff_charge::get(&atom.label) {
                atom.charge = q;
            }
        }

        // Rebuild pair list.
        self.coulomb_pairs.clear();
        let n = self.atoms.len();
        for i in 0..n {
            let qi = self.atoms[i].charge;
            if qi.abs() < 1e-12 {
                continue;
            }
            for j in (i + 1)..n {
                let qj = self.atoms[j].charge;
                if qj.abs() < 1e-12 {
                    continue;
                }
                self.add_coulomb_pair(AtomIdx(i), AtomIdx(j), qi, qj);
            }
        }
    }

    // -------------------------------------------------------------------
    // Gradient aggregation ------------------------------------------------

    /// Return the current analytic Cartesian gradient for all energy terms
    /// that have dedicated derivative implementations.
    ///
    /// The present version aggregates the contributions from *bond-stretch*
    /// and *angle-bend* terms.  The other components (torsion, inversion,
    /// van-der-Waals, electrostatics, …) still fall back to numerical
    /// derivatives in the optimisation stage until their analytic forms are
    /// added.
    pub fn analytic_gradient(&self) -> rdkit_core::Result<Vec<geometry::Point3D>> {
        use geometry::Point3D;

        let mut grad = vec![Point3D(0.0, 0.0, 0.0); self.atom_count()];

        // Bond term ------------------------------------------------------
        let bond_grad = self.bond_gradients()?;
        for (total, g) in grad.iter_mut().zip(bond_grad.iter()) {
            total.0 += g.0;
            total.1 += g.1;
            total.2 += g.2;
        }

        // Angle term -----------------------------------------------------
        let angle_grad = self.angle_gradients()?;
        for (total, g) in grad.iter_mut().zip(angle_grad.iter()) {
            total.0 += g.0;
            total.1 += g.1;
            total.2 += g.2;
        }

        // Torsion term ---------------------------------------------------
        let torsion_grad = self.torsion_gradients()?;
        for (total, g) in grad.iter_mut().zip(torsion_grad.iter()) {
            total.0 += g.0;
            total.1 += g.1;
            total.2 += g.2;
        }

        // Lennard-Jones term --------------------------------------------
        let lj_grad = self.lj_gradients();
        for (total, g) in grad.iter_mut().zip(lj_grad.iter()) {
            total.0 += g.0;
            total.1 += g.1;
            total.2 += g.2;
        }

        // Inversion term -------------------------------------------------
        let inv_grad = self.inversion_gradients();
        for (total, g) in grad.iter_mut().zip(inv_grad.iter()) {
            total.0 += g.0;
            total.1 += g.1;
            total.2 += g.2;
        }

        // Coulomb term ----------------------------------------------------
        let q_grad = self.coulomb_gradients();
        for (total, g) in grad.iter_mut().zip(q_grad.iter()) {
            total.0 += g.0;
            total.1 += g.1;
            total.2 += g.2;
        }

        Ok(grad)
    }

    /// Analytic gradients for Lennard-Jones 12-6 non-bonded pairs.
    pub fn lj_gradients(&self) -> Vec<geometry::Point3D> {
        use geometry::{Point3D, Vector3D};

        let mut grad = vec![Point3D(0.0, 0.0, 0.0); self.atom_count()];

        for pair in &self.lj_pairs {
            let a_idx = pair.a.0;
            let b_idx = pair.b.0;

            let coord_a = self.atoms[a_idx].coord;
            let coord_b = self.atoms[b_idx].coord;

            let r_vec: Vector3D = coord_a - coord_b;
            let r = r_vec.norm();
            if r < 1e-12 {
                continue; // overlapping atoms, skip
            }

            let d_edr = crate::lj_energy_derivative(pair.epsilon, pair.sigma, r);

            let u = r_vec / r;
            let g = u * d_edr;

            grad[a_idx].0 += g.x();
            grad[a_idx].1 += g.y();
            grad[a_idx].2 += g.z();

            grad[b_idx].0 -= g.x();
            grad[b_idx].1 -= g.y();
            grad[b_idx].2 -= g.z();
        }

        grad
    }

    /// Analytic gradients for improper torsion / inversion terms.
    pub fn inversion_gradients(&self) -> Vec<geometry::Point3D> {
        use geometry::{Point3D, Vector3D};

        let mut grad = vec![Point3D(0.0, 0.0, 0.0); self.atom_count()];

        const EPS: f64 = 1e-8;

        for inv in &self.inversions {
            let i_idx = inv.i.0; // out-of-plane atom
            let j_idx = inv.j.0; // central atom
            let k_idx = inv.k.0;
            let l_idx = inv.l.0;

            let p_i = self.atoms[i_idx].coord;
            let p_j = self.atoms[j_idx].coord;
            let p_k = self.atoms[k_idx].coord;
            let p_l = self.atoms[l_idx].coord;

            // Unit vectors rJI, rJK, rJL (from central j)
            let mut r_ji: Vector3D = p_i - p_j;
            let mut r_jk: Vector3D = p_k - p_j;
            let mut r_jl: Vector3D = p_l - p_j;

            let d_ji = r_ji.norm();
            let d_jk = r_jk.norm();
            let d_jl = r_jl.norm();

            if d_ji < EPS || d_jk < EPS || d_jl < EPS {
                continue;
            }

            r_ji = r_ji / d_ji;
            r_jk = r_jk / d_jk;
            r_jl = r_jl / d_jl;

            // Normal vector to plane (j,i,k)
            let mut n = (-r_ji).cross(r_jk);
            let n_len = n.norm();
            if n_len < EPS {
                continue;
            }
            n = n / n_len;

            let cos_y = clip_to_one(n.dot(r_jl));
            let sin_y_sq = 1.0 - cos_y * cos_y;
            let sin_y = sin_y_sq.max(0.0).sqrt();

            // Angle between ji and jk
            let cos_theta = clip_to_one(r_ji.dot(r_jk));
            let sin_theta_sq = 1.0 - cos_theta * cos_theta;
            let sin_theta = sin_theta_sq.max(0.0).sqrt();

            // chi (out-of-plane) computed via asin(cos_y)
            let chi = cos_y.asin();

            // dE/dchi (harmonic)
            let d_e_dchi = inv.k_chi * (chi - inv.chi0);

            // Convert to dE/dW sign consistent with RDKit (W=chi)
            let d_e_dw = d_e_dchi;

            // Helper cross products
            let t1 = r_jl.cross(r_jk);
            let t2 = r_ji.cross(r_jl);
            let t3 = r_jk.cross(r_ji);

            let term1 = sin_y * sin_theta;
            if term1.abs() < EPS {
                continue;
            }
            let term2 = cos_y / (sin_y * sin_theta_sq.max(EPS));

            // tg1, tg3, tg4 as arrays
            let tg1 = Vector3D::new(
                (t1.x() / term1 - (r_ji.x() - r_jk.x() * cos_theta) * term2) / d_ji,
                (t1.y() / term1 - (r_ji.y() - r_jk.y() * cos_theta) * term2) / d_ji,
                (t1.z() / term1 - (r_ji.z() - r_jk.z() * cos_theta) * term2) / d_ji,
            );
            let tg3 = Vector3D::new(
                (t2.x() / term1 - (r_jk.x() - r_ji.x() * cos_theta) * term2) / d_jk,
                (t2.y() / term1 - (r_jk.y() - r_ji.y() * cos_theta) * term2) / d_jk,
                (t2.z() / term1 - (r_jk.z() - r_ji.z() * cos_theta) * term2) / d_jk,
            );
            let tg4 = Vector3D::new(
                (t3.x() / term1 - r_jl.x() * cos_y / sin_y) / d_jl,
                (t3.y() / term1 - r_jl.y() * cos_y / sin_y) / d_jl,
                (t3.z() / term1 - r_jl.z() * cos_y / sin_y) / d_jl,
            );

            // Accumulate
            grad[i_idx].0 += d_e_dw * tg1.x();
            grad[i_idx].1 += d_e_dw * tg1.y();
            grad[i_idx].2 += d_e_dw * tg1.z();

            grad[k_idx].0 += d_e_dw * tg3.x();
            grad[k_idx].1 += d_e_dw * tg3.y();
            grad[k_idx].2 += d_e_dw * tg3.z();

            grad[l_idx].0 += d_e_dw * tg4.x();
            grad[l_idx].1 += d_e_dw * tg4.y();
            grad[l_idx].2 += d_e_dw * tg4.z();

            // Central atom j gets negative sum
            grad[j_idx].0 -= d_e_dw * (tg1.x() + tg3.x() + tg4.x());
            grad[j_idx].1 -= d_e_dw * (tg1.y() + tg3.y() + tg4.y());
            grad[j_idx].2 -= d_e_dw * (tg1.z() + tg3.z() + tg4.z());
        }

        grad
    }

    /// Analytic gradients for Coulomb pairs.
    pub fn coulomb_gradients(&self) -> Vec<geometry::Point3D> {
        use geometry::{Point3D, Vector3D};

        let mut grad = vec![Point3D(0.0, 0.0, 0.0); self.atom_count()];
        for pair in &self.coulomb_pairs {
            let a_idx = pair.a.0;
            let b_idx = pair.b.0;
            let pa = self.atoms[a_idx].coord;
            let pb = self.atoms[b_idx].coord;
            let r_vec: Vector3D = pa - pb;
            let r = r_vec.norm();
            if r < 1e-12 {
                continue;
            }
            let d_edr = crate::coulomb_energy_derivative(pair.q_a, pair.q_b, r);
            let g_vec = r_vec / r * d_edr; // vector

            grad[a_idx].0 += g_vec.x();
            grad[a_idx].1 += g_vec.y();
            grad[a_idx].2 += g_vec.z();

            grad[b_idx].0 -= g_vec.x();
            grad[b_idx].1 -= g_vec.y();
            grad[b_idx].2 -= g_vec.z();
        }

        grad
    }

    // -------------------------------------------------------------------
    // Gradient helpers ---------------------------------------------------

    /// Compute analytic **per-atom** Cartesian gradients (∂E/∂x, ∂E/∂y, ∂E/∂z)
    /// arising **only** from the bond-stretch terms currently stored in the
    /// force-field.
    ///
    /// Returned vector has length equal to `self.atom_count()` and is ordered
    /// in the same sequence as atoms were inserted.  Each entry corresponds to
    /// the gradient on the matching atom (in kcal mol⁻¹ Å⁻¹).
    ///
    /// *Implementation notes*
    ///
    /// For a bond between atoms *a* and *b* separated by the vector
    /// **r** = **rₐ** − **r_b** with length *r = |**r**|*, the harmonic energy
    /// implemented in [`crate::bond_stretch_energy`] depends only on *r*.
    /// The chain-rule gives
    ///
    /// ```text
    /// ∂E/∂rₐ = (∂E/∂r) ∂r/∂rₐ
    ///        = (dEdr)  (r / |r|)
    ///
    /// ∂E/∂r_b = −(dEdr) (r / |r|)
    /// ```
    ///
    /// where `dEdr` is provided by [`crate::bond_stretch_energy_derivative`].
    pub fn bond_gradients(&self) -> rdkit_core::Result<Vec<geometry::Point3D>> {
        use geometry::{Point3D, Vector3D};

        let mut grad = vec![Point3D(0.0, 0.0, 0.0); self.atom_count()];

        for bond in &self.bonds {
            let a_idx = bond.a.0;
            let b_idx = bond.b.0;

            let coord_a = self.atoms[a_idx].coord;
            let coord_b = self.atoms[b_idx].coord;

            // Vector r = r_a - r_b
            let r_vec: Vector3D = coord_a - coord_b;
            let r = r_vec.norm();

            if r.abs() < f64::EPSILON {
                // Degenerate case – overlapping atoms; skip contribution to avoid NaNs.
                continue;
            }

            // ∂E/∂r (scalar)
            let d_edr = crate::bond_stretch_energy_derivative(
                &self.atoms[a_idx].label,
                &self.atoms[b_idx].label,
                bond.order,
                r,
            )?;

            // Unit vector along bond.
            let u = r_vec / r;

            // Contribution vector.
            let g = u * d_edr; // Vector3D

            // Accumulate (note the sign difference for the two atoms).
            grad[a_idx].0 += g.x();
            grad[a_idx].1 += g.y();
            grad[a_idx].2 += g.z();

            grad[b_idx].0 -= g.x();
            grad[b_idx].1 -= g.y();
            grad[b_idx].2 -= g.z();
        }

        Ok(grad)
    }

    /// Compute analytic per-atom gradients coming from **angle-bend** terms
    /// (kcal mol⁻¹ Å⁻¹).  See [`bond_gradients`] for the return format.
    pub fn angle_gradients(&self) -> rdkit_core::Result<Vec<geometry::Point3D>> {
        use geometry::{Point3D, Vector3D};

        let mut grad = vec![Point3D(0.0, 0.0, 0.0); self.atom_count()];

        for angle in &self.angles {
            let i_idx = angle.i.0;
            let j_idx = angle.j.0; // central
            let k_idx = angle.k.0;

            let r_i = self.atoms[i_idx].coord;
            let r_j = self.atoms[j_idx].coord;
            let r_k = self.atoms[k_idx].coord;

            // Vectors: r1 = r_i - r_j, r2 = r_k - r_j
            let r1: Vector3D = r_i - r_j;
            let r2: Vector3D = r_k - r_j;

            let r1_len = r1.norm();
            let r2_len = r2.norm();

            if r1_len < 1e-12 || r2_len < 1e-12 {
                // Degenerate; skip contribution
                continue;
            }

            // cos(theta) and theta
            let cos_theta = clip_to_one(r1.dot(r2) / (r1_len * r2_len));
            let theta = cos_theta.acos();
            let sin_theta = theta.sin().abs().max(1e-8); // avoid div by 0

            // dE/dθ (scalar)
            let d_ed_theta = crate::angle_bend_energy_derivative(
                &self.atoms[i_idx].label,
                &self.atoms[j_idx].label,
                &self.atoms[k_idx].label,
                theta,
                angle.order_ij,
                angle.order_jk,
            )?;

            // Precompute factors
            let inv_r1 = 1.0 / r1_len;
            let inv_r2 = 1.0 / r2_len;

            // Term A = (r2 / (|r1||r2|)) - cosθ * r1 / |r1|^2
            let term_a = (r2 * (inv_r1 * inv_r2)) - (r1 * (cos_theta * inv_r1 * inv_r1));
            // Term C analogously with r1/r2 swapped
            let term_c = (r1 * (inv_r1 * inv_r2)) - (r2 * (cos_theta * inv_r2 * inv_r2));

            // Scalar factor
            let coef = -d_ed_theta / sin_theta; // negative due to dcos/dx vs dθ/dcos

            let g_i = term_a * coef;
            let g_k = term_c * coef;
            let g_j = -(g_i + g_k); // momentum conservation

            // accumulate
            grad[i_idx].0 += g_i.x();
            grad[i_idx].1 += g_i.y();
            grad[i_idx].2 += g_i.z();

            grad[k_idx].0 += g_k.x();
            grad[k_idx].1 += g_k.y();
            grad[k_idx].2 += g_k.z();

            grad[j_idx].0 += g_j.x();
            grad[j_idx].1 += g_j.y();
            grad[j_idx].2 += g_j.z();
        }

        Ok(grad)
    }

    /// Compute analytic Cartesian gradients originating from **torsion/dihedral**
    /// terms currently present in the force‐field.  The implementation is a
    /// direct Rust port of the algorithm used in RDKit’s `UFF::TorsionAngle`
    /// contribution (see `ForceField/UFF/TorsionAngle.cpp`).  Although the
    /// algebra is somewhat involved, the core idea is straightforward:
    ///
    /// 1. Let `φ` be the dihedral angle between the two planes defined by the
    ///    atom triplets *(i, j, k)* and *(j, k, l)*.
    /// 2. The energy depends only on `φ`, i.e. `E = f(φ)`.
    /// 3. By the chain-rule the Cartesian gradient on an atom *p* is
    ///    `∂E/∂p = (dE/dφ) · (∂φ/∂p)`.
    ///
    /// The derivative `dE/dφ` is trivial – we have an analytic expression in
    /// [`crate::torsion_energy_derivative`].  The heavy lifting is therefore
    /// buried in the geometric term `∂φ/∂p`.  To avoid re-deriving the entire
    /// expression we translate RDKit’s well-tested helper routine verbatim,
    /// keeping the same nomenclature (`r`, `t`, `dCos_dT`, …).
    pub fn torsion_gradients(&self) -> rdkit_core::Result<Vec<geometry::Point3D>> {
        use geometry::Vector3D;

        let mut grad = vec![geometry::Point3D(0.0, 0.0, 0.0); self.atom_count()];

        // Threshold to guard against division by zero.
        const EPS: f64 = 1e-8;

        for tor in &self.torsions {
            let (i_idx, j_idx, k_idx, l_idx) = (tor.i.0, tor.j.0, tor.k.0, tor.l.0);

            let p1 = self.atoms[i_idx].coord;
            let p2 = self.atoms[j_idx].coord;
            let p3 = self.atoms[k_idx].coord;
            let p4 = self.atoms[l_idx].coord;

            // --- Construct intermediate vectors identical to RDKit helper ---
            let r0: Vector3D = p1 - p2; // r[0] = p1 - p2
            let r1: Vector3D = p3 - p2; // r[1] = p3 - p2
            let r2: Vector3D = p2 - p3; // r[2] = -r1
            let r3: Vector3D = p4 - p3; // r[3] = p4 - p3

            // t0 = r0 × r1 ; t1 = r2 × r3
            let mut t0 = r0.cross(r1);
            let mut t1 = r2.cross(r3);

            // Normalise t0 and t1, retaining original norms in d0/d1.
            let d0 = t0.norm();
            let d1 = t1.norm();

            if d0 < EPS || d1 < EPS {
                // Atoms are nearly colinear – skip this torsion contribution to
                // avoid numerical blow-ups.  Energy and gradient are negligible
                // around the singularity anyway.
                continue;
            }

            t0 = t0 / d0;
            t1 = t1 / d1;

            // Cosine and sine of the dihedral.
            let cos_phi = clip_to_one(t0.dot(t1));
            let sin_phi_sq = 1.0 - cos_phi * cos_phi;
            let sin_phi = sin_phi_sq.max(0.0).sqrt();

            // Actual dihedral angle (signed) is only needed for dE/dφ.
            let phi = dihedral_angle(p1, p2, p3, p4);

            // dE/dφ from the torsion potential.
            let de_dphi = crate::torsion_energy_derivative(tor.v, tor.periodicity, phi, tor.phi0);

            // sinTerm = dE_dphi / sin_phi  (with fallback when sin_phi ≈ 0)
            let sin_term = if sin_phi.abs() < EPS {
                de_dphi / cos_phi
            } else {
                de_dphi / sin_phi
            };

            // Pre-compute helper derivatives dCos/dT (see original C++ code).
            //   dCos/dT0 = (t1 - cosφ · t0) / |t0_orig|
            //   dCos/dT1 = (t0 - cosφ · t1) / |t1_orig|
            let dcos_dt0 = (t1 - t0 * cos_phi) / d0;
            let dcos_dt1 = (t0 - t1 * cos_phi) / d1;

            // Unpack components for convenience.
            let (d0x, d0y, d0z) = (dcos_dt0.x(), dcos_dt0.y(), dcos_dt0.z());
            let (d1x, d1y, d1z) = (dcos_dt1.x(), dcos_dt1.y(), dcos_dt1.z());

            // r vectors components
            let (r1x, r1y, r1z) = (r1.x(), r1.y(), r1.z());
            let (r0x, r0y, r0z) = (r0.x(), r0.y(), r0.z());
            let (r2x, r2y, r2z) = (r2.x(), r2.y(), r2.z());
            let (r3x, r3y, r3z) = (r3.x(), r3.y(), r3.z());

            // Gradient for atom i (p1) --------------------------------------------------
            grad[i_idx].0 += sin_term * (d0z * r1y - d0y * r1z);
            grad[i_idx].1 += sin_term * (d0x * r1z - d0z * r1x);
            grad[i_idx].2 += sin_term * (d0y * r1x - d0x * r1y);

            // Atom j (p2) --------------------------------------------------------------
            grad[j_idx].0 += sin_term * (d0y * (r1z - r0z)
                + d0z * (r0y - r1y)
                + d1y * (-r3z)
                + d1z * (r3y));

            grad[j_idx].1 += sin_term * (d0x * (r0z - r1z)
                + d0z * (r1x - r0x)
                + d1x * (r3z)
                + d1z * (-r3x));

            grad[j_idx].2 += sin_term * (d0x * (r1y - r0y)
                + d0y * (r0x - r1x)
                + d1x * (-r3y)
                + d1y * (r3x));

            // Atom k (p3) --------------------------------------------------------------
            grad[k_idx].0 += sin_term * (d0y * (r0z)
                + d0z * (-r0y)
                + d1y * (r3z - r2z)
                + d1z * (r2y - r3y));

            grad[k_idx].1 += sin_term * (d0x * (-r0z)
                + d0z * (r0x)
                + d1x * (r2z - r3z)
                + d1z * (r3x - r2x));

            grad[k_idx].2 += sin_term * (d0x * (r0y)
                + d0y * (-r0x)
                + d1x * (r3y - r2y)
                + d1y * (r2x - r3x));

            // Atom l (p4) --------------------------------------------------------------
            grad[l_idx].0 += sin_term * (d1y * r2z - d1z * r2y);
            grad[l_idx].1 += sin_term * (d1z * r2x - d1x * r2z);
            grad[l_idx].2 += sin_term * (d1x * r2y - d1y * r2x);
        }

        Ok(grad)
    }

    // -------------------------------------------------------------------
    // Internal helper accessors (used by optimisation routines)

    /// Mutable slice of atoms – optimisation code may tweak coordinates.
    pub(crate) fn atoms_mut(&mut self) -> &mut [Atom] {
        &mut self.atoms
    }

    /// Number of atoms currently stored.
    pub(crate) fn atom_count(&self) -> usize {
        self.atoms.len()
    }

    /// Mutable access to a specific atom by index.  Used primarily by unit
    /// tests and optimisation helpers.
    #[cfg(test)]
    pub(crate) fn get_atom_mut(&mut self, idx: usize) -> &mut Atom {
        &mut self.atoms[idx]
    }

    /// Compute total potential energy (kcal/mol) as sum of all terms.
    pub fn total_energy(&self) -> Result<f64, rdkit_core::RdError> {
        let mut e = 0.0;

        // Bonds
        for bond in &self.bonds {
            let a = &self.atoms[bond.a.0];
            let b = &self.atoms[bond.b.0];
            let r = a.coord.distance(b.coord);
            e += bond_stretch_energy(&a.label, &b.label, bond.order, r)?;
        }

        // Angles
        for angle in &self.angles {
            let i = &self.atoms[angle.i.0];
            let j = &self.atoms[angle.j.0];
            let k = &self.atoms[angle.k.0];

            let vij = i.coord - j.coord;
            let vkj = k.coord - j.coord;
            let cos_theta = dot_v(vij, vkj) / (norm_v(vij) * norm_v(vkj));
            let theta = clip_to_one(cos_theta).acos();

            e += angle_bend_energy(
                &i.label,
                &j.label,
                &k.label,
                theta,
                angle.order_ij,
                angle.order_jk,
            )?;
        }

        // Torsions
        for tor in &self.torsions {
            let a = &self.atoms[tor.i.0];
            let b = &self.atoms[tor.j.0];
            let c = &self.atoms[tor.k.0];
            let d = &self.atoms[tor.l.0];

            let phi = dihedral_angle(a.coord, b.coord, c.coord, d.coord);
            e += torsion_energy(tor.v, tor.periodicity, phi, tor.phi0);
        }

        // Inversions
        for inv in &self.inversions {
            let i = &self.atoms[inv.i.0];
            let j = &self.atoms[inv.j.0];
            let k = &self.atoms[inv.k.0];
            let l = &self.atoms[inv.l.0];

            let chi = out_of_plane_angle(i.coord, j.coord, k.coord, l.coord);
            e += inversion_energy(inv.k_chi, chi, inv.chi0);
        }

        // Lennard-Jones
        for pair in &self.lj_pairs {
            let a = &self.atoms[pair.a.0];
            let b = &self.atoms[pair.b.0];
            let r = a.coord.distance(b.coord);
            e += lj_energy(pair.epsilon, pair.sigma, r);
        }

        // Coulomb
        for pair in &self.coulomb_pairs {
            let a = &self.atoms[pair.a.0];
            let b = &self.atoms[pair.b.0];
            let r = a.coord.distance(b.coord);
            e += crate::coulomb_energy(pair.q_a, pair.q_b, r);
        }

        Ok(e)
    }
}

// ---------------------------------------------------------------------------
// Helper geometry functions --------------------------------------------------

use geometry::{dot_v, norm_v};

/// Compute dihedral angle (radians) using standard textbook formula.
fn dihedral_angle(p1: Point3D, p2: Point3D, p3: Point3D, p4: Point3D) -> f64 {
    use geometry::Vector3D;
    let b1 = Vector3D::new(p2.0 - p1.0, p2.1 - p1.1, p2.2 - p1.2);
    let b2 = Vector3D::new(p3.0 - p2.0, p3.1 - p2.1, p3.2 - p2.2);
    let b3 = Vector3D::new(p4.0 - p3.0, p4.1 - p3.1, p4.2 - p3.2);

    let n1 = b1.cross(b2).normalized().unwrap();
    let n2 = b2.cross(b3).normalized().unwrap();

    let m1 = n1.cross(b2.normalized().unwrap());

    let x = n1.dot(n2);
    let y = m1.dot(n2);
    y.atan2(x)
}

/// Out-of-plane angle χ (radians) between atoms i (out-of-plane), j (central),
/// k, l (basal plane).
fn out_of_plane_angle(i: Point3D, j: Point3D, k: Point3D, l: Point3D) -> f64 {
    use geometry::Vector3D;
    let vji = Vector3D::new(i.0 - j.0, i.1 - j.1, i.2 - j.2);
    let vjk = Vector3D::new(k.0 - j.0, k.1 - j.1, k.2 - j.2);
    let vjl = Vector3D::new(l.0 - j.0, l.1 - j.1, l.2 - j.2);

    let n = vjk.cross(vjl);
    let numerator = vji.dot(n);
    let denom = n.norm() * vji.norm();
    (numerator / denom).asin()
}
