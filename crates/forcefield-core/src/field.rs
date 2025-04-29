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

/// Main container.
#[derive(Default)]
pub struct ForceField {
    atoms: Vec<Atom>,
    bonds: Vec<Bond>,
    angles: Vec<Angle>,
    torsions: Vec<Torsion>,
    inversions: Vec<Inversion>,
    lj_pairs: Vec<LjPair>,
}

impl ForceField {
    /// Add atom, returning its index.
    pub fn add_atom(&mut self, label: impl Into<String>, coord: Point3D) -> AtomIdx {
        let idx = AtomIdx(self.atoms.len());
        self.atoms.push(Atom {
            label: label.into(),
            coord,
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
            let dEdr = crate::bond_stretch_energy_derivative(
                &self.atoms[a_idx].label,
                &self.atoms[b_idx].label,
                bond.order,
                r,
            )?;

            // Unit vector along bond.
            let u = r_vec / r;

            // Contribution vector.
            let g = u * dEdr; // Vector3D

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

    /// Mutable access to a specific atom by index.  Panics if out of bounds –
    /// optimisation code should ensure valid indices.
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
