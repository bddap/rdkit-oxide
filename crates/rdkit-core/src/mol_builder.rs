//! Mutable builder for [`Mol`](crate::mol::Mol).

use crate::mol::{Atom, Bond, BondOrder, Element, Mol};

/// Index wrapper returned on `add_atom` so the caller can reuse it for bonds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AtomIdx(pub usize);

/// Builder collects edits then produces an immutable `Mol` via `finish()`.
#[derive(Default)]
pub struct MolBuilder {
    atoms: Vec<Atom>,
    bonds: Vec<Bond>,
}

impl MolBuilder {
    pub fn new() -> Self { Self::default() }

    /// Add atom, returning its index for subsequent calls.
    pub fn add_atom(&mut self, element: Element) -> AtomIdx {
        let idx = self.atoms.len();
        self.atoms.push(Atom::new(element));
        AtomIdx(idx)
    }

    pub fn add_bond(&mut self, a: AtomIdx, b: AtomIdx, order: BondOrder) -> &mut Self {
        self.bonds.push(Bond { a: a.0, b: b.0, order });
        self
    }

    pub fn set_formal_charge(&mut self, idx: AtomIdx, charge: i8) -> &mut Self {
        self.atoms[idx.0].formal_charge = charge;
        self
    }

    /// Finalise edits; sanitisation stub for now.
    pub fn finish(self) -> crate::Result<Mol> {
        // TODO: ring perception, valence checks, etc.
        Ok(Mol { atoms: self.atoms, bonds: self.bonds })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_water() {
        let mut b = MolBuilder::new();
        let o = b.add_atom(Element::O);
        let h1 = b.add_atom(Element::H);
        let h2 = b.add_atom(Element::H);
        b.add_bond(o, h1, BondOrder::Single)
            .add_bond(o, h2, BondOrder::Single);
        let m = b.finish().unwrap();
        assert_eq!(m.atoms.len(), 3);
    }
}
