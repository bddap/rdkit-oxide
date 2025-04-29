//! Sparse bit vector – stores indices of set bits in a `HashSet`.
//! This is only a *partial* implementation, aimed at supporting the
//! first batch of ported RDKit unit-tests (limit conditions and basic CRUD).

use std::collections::HashSet;
use std::fmt;
use std::ops::{BitAnd, BitOr, BitXor, Not};

use base64::prelude::*;

/// A sparse bit-vector with a fixed length.
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct SparseBitVect {
    size: u32,
    bits: HashSet<u32>,
}

impl SparseBitVect {
    /// Create a new sparse vector with the given number of bits (all zero).
    pub fn new(size: u32) -> Self {
        Self {
            size,
            bits: HashSet::new(),
        }
    }

    /// Convenience: maximum `u32` value.
    fn is_max_size(&self) -> bool {
        self.size == u32::MAX
    }

    /// Helper for index validation.  RDKit’s C++ implementation treats
    /// `idx == size` as *allowed* only when `size == UINT_MAX`.
    fn check_index(&self, idx: u32) -> bool {
        idx < self.size || (idx == self.size && self.is_max_size())
    }

    /// Total number of addressable bits.
    pub fn num_bits(&self) -> u32 {
        self.size
    }

    /// Set a bit, returning the previous value.
    pub fn set_bit(&mut self, idx: u32) -> bool {
        assert!(self.check_index(idx), "bit index {} out of range", idx);
        let already = self.bits.contains(&idx);
        self.bits.insert(idx);
        already
    }

    /// Unset a bit, returning the previous value.
    pub fn unset_bit(&mut self, idx: u32) -> bool {
        assert!(self.check_index(idx), "bit index {} out of range", idx);
        self.bits.remove(&idx)
    }

    /// Get bit value.
    pub fn get_bit(&self, idx: u32) -> bool {
        assert!(self.check_index(idx), "bit index {} out of range", idx);
        self.bits.contains(&idx)
    }

    /// Number of set bits.
    pub fn num_on_bits(&self) -> u32 {
        self.bits.len() as u32
    }

    /// Number of zero bits.
    pub fn num_off_bits(&self) -> u32 {
        self.size.wrapping_add(self.is_max_size() as u32) - self.num_on_bits()
    }

    /// Number of on bits in common with another sparse vector.
    pub fn num_on_bits_in_common(&self, other: &Self) -> u32 {
        assert_eq!(self.size, other.size);
        self.bits.intersection(&other.bits).count() as u32
    }

    /// Tanimoto similarity.
    pub fn tanimoto_similarity(&self, other: &Self) -> f64 {
        let c = self.num_on_bits_in_common(other) as f64;
        let a = self.num_on_bits() as f64;
        let b = other.num_on_bits() as f64;
        if (a + b - c).abs() < f64::EPSILON {
            0.0
        } else {
            c / (a + b - c)
        }
    }

    /// Fold the vector by a power-of-two factor, combining bits with OR.
    pub fn fold(&self, factor: u32) -> Self {
        assert!(factor.is_power_of_two());
        assert_eq!(self.size % factor, 0);
        let new_size = self.size / factor;
        let mut folded = SparseBitVect::new(new_size);
        for &idx in &self.bits {
            let new_idx = idx % new_size;
            folded.bits.insert(new_idx);
        }
        folded
    }

    /// Return iterator of set-bit indices.
    pub fn on_bits(&self) -> impl Iterator<Item = u32> + '_ {
        self.bits.iter().copied()
    }

    /// Base64 serialization (bincode + base64).
    pub fn to_base64(&self) -> String {
        let bytes = bincode::serialize(self).expect("serialize SparseBitVect");
        base64::engine::general_purpose::STANDARD.encode(bytes)
    }

    pub fn from_base64(s: &str) -> Self {
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(s)
            .expect("decode base64 SparseBitVect");
        bincode::deserialize(&bytes).expect("deserialize SparseBitVect")
    }
}

// -------------------------------------------------------------------------
// Display / Debug helpers

impl fmt::Debug for SparseBitVect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SparseBitVect")
            .field("size", &self.size)
            .field("bits", &self.bits)
            .finish()
    }
}

impl PartialEq for SparseBitVect {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.bits == other.bits
    }
}

impl Eq for SparseBitVect {}

// -------------------------------------------------------------------------
// Bitwise operations.  These mirror the semantics of C++ `operator| & ^ ~`.

impl<'b> BitOr<&'b SparseBitVect> for &SparseBitVect {
    type Output = SparseBitVect;
    fn bitor(self, rhs: &'b SparseBitVect) -> Self::Output {
        assert_eq!(self.size, rhs.size);
        let mut bits = self.bits.clone();
        bits.extend(rhs.bits.iter().copied());
        SparseBitVect {
            size: self.size,
            bits,
        }
    }
}

impl<'b> BitAnd<&'b SparseBitVect> for &SparseBitVect {
    type Output = SparseBitVect;
    fn bitand(self, rhs: &'b SparseBitVect) -> Self::Output {
        assert_eq!(self.size, rhs.size);
        let bits = self
            .bits
            .intersection(&rhs.bits)
            .copied()
            .collect::<HashSet<u32>>();
        SparseBitVect {
            size: self.size,
            bits,
        }
    }
}

impl<'b> BitXor<&'b SparseBitVect> for &SparseBitVect {
    type Output = SparseBitVect;
    fn bitxor(self, rhs: &'b SparseBitVect) -> Self::Output {
        assert_eq!(self.size, rhs.size);
        let bits = self
            .bits
            .symmetric_difference(&rhs.bits)
            .copied()
            .collect::<HashSet<u32>>();
        SparseBitVect {
            size: self.size,
            bits,
        }
    }
}

impl Not for &SparseBitVect {
    type Output = SparseBitVect;
    fn not(self) -> Self::Output {
        let mut bits = HashSet::new();
        let inclusive_size = self.size as u64 + self.is_max_size() as u64; // inclusive behaviour
        for idx in 0..inclusive_size {
            let idx_u32 = idx as u32;
            if !self.bits.contains(&idx_u32) {
                bits.insert(idx_u32);
            }
        }
        SparseBitVect {
            size: self.size,
            bits,
        }
    }
}

// Indexing operator: sbv[idx]

use std::ops::Index;

const TRUE_VAL: bool = true;
const FALSE_VAL: bool = false;

impl Index<u32> for SparseBitVect {
    type Output = bool;
    fn index(&self, idx: u32) -> &Self::Output {
        if self.get_bit(idx) {
            &TRUE_VAL
        } else {
            &FALSE_VAL
        }
    }
}

impl Index<usize> for SparseBitVect {
    type Output = bool;
    fn index(&self, idx: usize) -> &Self::Output {
        self.index(idx as u32)
    }
}

// -------------------------------------------------------------------------
// Unit tests ---------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limits_sparse_vector() {
        let max = u32::MAX;
        let mut bv = SparseBitVect::new(max);
        assert_eq!(bv.num_bits(), max);
        // set last *inclusive* bit (allowed when size == UINT_MAX)
        assert!(!bv.set_bit(max));
        assert!(bv.get_bit(max));
        assert_eq!(bv.num_on_bits(), 1);
    }

    #[test]
    fn bit_common_and_tanimoto() {
        let mut a = SparseBitVect::new(64);
        a.set_bit(1);
        a.set_bit(3);
        let mut b = SparseBitVect::new(64);
        b.set_bit(3);
        b.set_bit(4);
        assert_eq!(a.num_on_bits_in_common(&b), 1);
        let tanimoto = a.tanimoto_similarity(&b);
        assert!((tanimoto - 1.0 / 3.0).abs() < 1e-6);
    }

    #[test]
    fn fold_vector() {
        let mut v = SparseBitVect::new(32);
        v.set_bit(1);
        v.set_bit(17);
        let folded = v.fold(2);
        assert_eq!(folded.num_bits(), 16);
        assert_eq!(folded.num_on_bits(), 1);
        assert!(folded.get_bit(1));
    }
}
