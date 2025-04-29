//! Minimal molecule graph primitives in an idiomatic‐Rust style.

use std::collections::HashMap;

/// Chemical element.  Tagged with the IUPAC atomic number via `repr(u8)` so
/// that `as u8` yields `Z` cheaply.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Element {
    H = 1,
    C = 6,
    N = 7,
    O = 8,
    F = 9,
    P = 15,
    S = 16,
    Cl = 17,
    Br = 35,
    I = 53,
    // Extend as needed…
}

impl Element {
    /// Return the atomic number (`Z`).
    pub fn atomic_number(self) -> u8 { self as u8 }
}

/// Covalent bond order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BondOrder { Single, Double, Triple, Aromatic }

/// Single atom record – intentionally sparse for now.
#[derive(Debug, Clone)]
pub struct Atom {
    pub element: Element,
    pub formal_charge: i8,
    pub explicit_h_count: u8,

    #[allow(dead_code)]
    props: HashMap<String, String>,
}

impl Atom {
    pub fn new(element: Element) -> Self {
        Self { element, formal_charge: 0, explicit_h_count: 0, props: HashMap::new() }
    }
}

/// Undirected bond edge.
#[derive(Debug, Clone)]
pub struct Bond { pub a: usize, pub b: usize, pub order: BondOrder }

/// Molecule as adjacency lists.
#[derive(Debug, Clone, Default)]
pub struct Mol { pub atoms: Vec<Atom>, pub bonds: Vec<Bond> }

impl Mol {
    pub fn add_atom(&mut self, atom: Atom) -> usize {
        let idx = self.atoms.len();
        self.atoms.push(atom);
        idx
    }

    pub fn add_bond(&mut self, a: usize, b: usize, order: BondOrder) {
        self.bonds.push(Bond { a, b, order });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn element_atomic_number() {
        assert_eq!(Element::O.atomic_number(), 8);
        assert_eq!(Element::Cl.atomic_number(), 17);
    }

    #[test]
    fn build_water() {
        let mut m = Mol::default();
        let o = m.add_atom(Atom::new(Element::O));
        let h1 = m.add_atom(Atom::new(Element::H));
        let h2 = m.add_atom(Atom::new(Element::H));
        m.add_bond(o, h1, BondOrder::Single);
        m.add_bond(o, h2, BondOrder::Single);

        assert_eq!(m.atoms.len(), 3);
        assert_eq!(m.bonds.len(), 2);
    }
}
