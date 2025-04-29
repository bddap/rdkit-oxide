//! Minimal molecular graph datastructures (Atom, Bond, Mol).

use std::collections::HashMap;

/// Chemical element symbol (atomic number encoded as u8 for space efficiency).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Element(pub u8);

impl Element {
    pub const H: Self = Element(1);
    pub const C: Self = Element(6);
    pub const N: Self = Element(7);
    pub const O: Self = Element(8);
    pub const F: Self = Element(9);
    pub const P: Self = Element(15);
    pub const S: Self = Element(16);
    #[allow(non_upper_case_globals)]
    pub const Cl: Self = Element(17);
}

/// Bond order enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BondOrder {
    Single,
    Double,
    Triple,
    Aromatic,
}

/// Atom representation with minimal properties.
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

/// Bond connecting two atoms (undirected).
#[derive(Debug, Clone)]
pub struct Bond {
    pub a: usize,
    pub b: usize,
    pub order: BondOrder,
}

/// Molecule as adjacency list.
#[derive(Debug, Clone)]
pub struct Mol {
    pub atoms: Vec<Atom>,
    pub bonds: Vec<Bond>,
}

impl Default for Mol {
    fn default() -> Self { Self::new() }
}

impl Mol {
    pub fn new() -> Self { Self { atoms: Vec::new(), bonds: Vec::new() } }

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
    fn build_water() {
        let mut m = Mol::new();
        let o = m.add_atom(Atom::new(Element::O));
        let h1 = m.add_atom(Atom::new(Element::H));
        let h2 = m.add_atom(Atom::new(Element::H));
        m.add_bond(o, h1, BondOrder::Single);
        m.add_bond(o, h2, BondOrder::Single);
        assert_eq!(m.atoms.len(), 3);
        assert_eq!(m.bonds.len(), 2);
    }
}
