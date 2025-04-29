//! Sparse bit vector – stores indices of set bits in a `HashSet`.
//! This is only a *partial* implementation, aimed at supporting the
//! first batch of ported RDKit unit-tests (limit conditions and basic CRUD).

use std::collections::HashSet;
use std::fmt;
use std::ops::{BitAnd, BitOr, BitXor, Not};

/// A sparse bit-vector with a fixed length.
#[derive(Clone)]
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

    /// Return iterator of set-bit indices.
    pub fn on_bits(&self) -> impl Iterator<Item = u32> + '_ {
        self.bits.iter().copied()
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

impl<'a, 'b> BitOr<&'b SparseBitVect> for &'a SparseBitVect {
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

impl<'a, 'b> BitAnd<&'b SparseBitVect> for &'a SparseBitVect {
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

impl<'a, 'b> BitXor<&'b SparseBitVect> for &'a SparseBitVect {
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

impl<'a> Not for &'a SparseBitVect {
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
        assert_eq!(bv.set_bit(max), false);
        assert!(bv.get_bit(max));
        assert_eq!(bv.num_on_bits(), 1);
    }
}
