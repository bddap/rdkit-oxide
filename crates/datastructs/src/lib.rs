//! Data structures (bit vectors, sparse vectors, etc.)

use bitvec::prelude::*;

pub mod sparse;


/// Count bits set in a u64 (popcount).
#[inline]
pub fn count_bits_u64(x: u64) -> u32 {
    x.count_ones()
}

/// Count bits set in a byte slice.
pub fn count_bits(buf: &[u8]) -> u32 {
    buf.iter().map(|b| b.count_ones()).sum()
}

/// Explicit (dense) bit vector.
#[derive(Debug)]
pub struct ExplicitBitVect {
    bits: BitVec<u8, Lsb0>,
}

impl Clone for ExplicitBitVect {
    fn clone(&self) -> Self {
        Self { bits: self.bits.clone() }
    }
}

impl PartialEq for ExplicitBitVect {
    fn eq(&self, other: &Self) -> bool {
        self.bits == other.bits
    }
}

impl Eq for ExplicitBitVect {}

impl ExplicitBitVect {
    /// Create a new bit vector of given length, all zeros.
    pub fn new(length: usize) -> Self {
        Self {
            bits: bitvec![u8, Lsb0; 0; length],
        }
    }

    pub fn len(&self) -> usize {
        self.bits.len()
    }

    pub fn set_bit(&mut self, idx: usize) {
        self.bits.set(idx, true);
    }

    pub fn get_bit(&self, idx: usize) -> bool {
        self.bits
            .get(idx)
            .map(|bit| *bit)
            .unwrap_or(false)
    }

    /// Clear (unset) a bit. Returns previous value.
    pub fn unset_bit(&mut self, idx: usize) -> bool {
        if idx >= self.len() {
            return false;
        }
        let prev = self.bits[idx];
        if prev {
            self.bits.set(idx, false);
        }
        prev
    }

    /// Total length (alias for `len`).
    pub fn num_bits(&self) -> usize {
        self.len()
    }

    /// Number of bits that are zero.
    pub fn num_off_bits(&self) -> u32 {
        (self.len() as u32) - self.num_on_bits()
    }

    /// Return indices of all bits that are set.
    pub fn get_on_bits(&self) -> Vec<usize> {
        self.bits
            .iter()
            .enumerate()
            .filter_map(|(idx, bit)| if *bit { Some(idx) } else { None })
            .collect()
    }

    pub fn num_on_bits(&self) -> u32 {
        self.bits.count_ones() as u32
    }

    /// Return new bit vector = self OR other.
    pub fn bit_or(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        let bits = self
            .bits
            .iter()
            .zip(other.bits.iter())
            .map(|(a, b)| *a | *b)
            .collect();
        Self { bits }
    }

    /// Return new bit vector = self AND other.
    pub fn bit_and(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        let bits = self
            .bits
            .iter()
            .zip(other.bits.iter())
            .map(|(a, b)| *a & *b)
            .collect();
        Self { bits }
    }

    /// Return new bit vector = self XOR other.
    pub fn bit_xor(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        let bits = self
            .bits
            .iter()
            .zip(other.bits.iter())
            .map(|(a, b)| *a ^ *b)
            .collect();
        Self { bits }
    }

    /// Fold the fingerprint by a power-of-two factor, combining bits with OR.
    pub fn fold(&self, factor: usize) -> Self {
        assert!(factor.is_power_of_two());
        assert_eq!(self.len() % factor, 0);
        let new_len = self.len() / factor;
        let mut folded = Self::new(new_len);
        for (idx, bit) in self.bits.iter().enumerate() {
            if *bit {
                let new_idx = idx % new_len;
                folded.set_bit(new_idx);
            }
        }
        folded
    }

    /// Bitwise AND with another vector, returning number of set bits in common.
    pub fn num_on_bits_in_common(&self, other: &Self) -> u32 {
        assert_eq!(self.len(), other.len());
        self.bits
            .iter()
            .zip(other.bits.iter())
            .map(|(a, b)| (*a & *b) as u32)
            .sum()
    }

    /// Tanimoto (a.k.a. Jaccard) similarity.
    pub fn tanimoto_similarity(&self, other: &Self) -> f64 {
        let c = self.num_on_bits_in_common(other) as f64;
        let a = self.num_on_bits() as f64;
        let b = other.num_on_bits() as f64;
        if a + b - c == 0.0 {
            0.0
        } else {
            c / (a + b - c)
        }
    }
}

// --- operator traits -------------------------------------------------------

use std::ops::{BitAnd, BitOr, BitXor, Not};

impl<'a, 'b> BitOr<&'b ExplicitBitVect> for &'a ExplicitBitVect {
    type Output = ExplicitBitVect;
    fn bitor(self, rhs: &'b ExplicitBitVect) -> Self::Output {
        self.bit_or(rhs)
    }
}

impl<'a, 'b> BitAnd<&'b ExplicitBitVect> for &'a ExplicitBitVect {
    type Output = ExplicitBitVect;
    fn bitand(self, rhs: &'b ExplicitBitVect) -> Self::Output {
        self.bit_and(rhs)
    }
}

impl<'a, 'b> BitXor<&'b ExplicitBitVect> for &'a ExplicitBitVect {
    type Output = ExplicitBitVect;
    fn bitxor(self, rhs: &'b ExplicitBitVect) -> Self::Output {
        self.bit_xor(rhs)
    }
}

impl<'a> Not for &'a ExplicitBitVect {
    type Output = ExplicitBitVect;
    fn not(self) -> Self::Output {
        let bits = self.bits.iter().map(|b| !*b).collect();
        ExplicitBitVect { bits }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_counts() {
        assert_eq!(count_bits_u64(0b1011), 3);
        let buf = [0b1111_0000u8, 0b0001_0001u8];
        assert_eq!(count_bits(&buf), 6);
    }

    #[test]
    fn explicit_bitvect() {
        let mut bv = ExplicitBitVect::new(128);
        bv.set_bit(5);
        bv.set_bit(64);
        assert!(bv.get_bit(5));
        assert_eq!(bv.num_on_bits(), 2);
        assert!(!bv.get_bit(63));

        let mut bv2 = ExplicitBitVect::new(128);
        bv2.set_bit(5);
        bv2.set_bit(7);
        assert_eq!(bv.num_on_bits_in_common(&bv2), 1);
        let tanimoto = bv.tanimoto_similarity(&bv2);
        // c =1, a=2, b=2 => 1/(2+2-1) = 1/3
        assert!((tanimoto - 1.0 / 3.0).abs() < 1e-6);
    }

    #[test]
    fn bitwise_ops_and_fold() {
        let mut a = ExplicitBitVect::new(16);
        a.set_bit(1);
        a.set_bit(3);
        let mut b = ExplicitBitVect::new(16);
        b.set_bit(3);
        b.set_bit(4);

        let or = a.bit_or(&b);
        assert_eq!(or.num_on_bits(), 3);
        let and = a.bit_and(&b);
        assert_eq!(and.num_on_bits(), 1);
        let xor = a.bit_xor(&b);
        assert_eq!(xor.num_on_bits(), 2);

        let folded = or.fold(2);
        assert_eq!(folded.len(), 8);
        assert_eq!(folded.num_on_bits(), 3);
        assert!(folded.get_bit(1));
        assert!(folded.get_bit(3));
        assert!(folded.get_bit(4));
    }

    #[test]
    fn fold_by_power_of_two_multiple() {
        let mut v = ExplicitBitVect::new(64);
        // set bits spaced 16 apart so that after folding by 4 they collide.
        v.set_bit(1);
        v.set_bit(17);
        v.set_bit(33);
        v.set_bit(49);

        // Before folding we have 4 bits
        assert_eq!(v.num_on_bits(), 4);

        let folded = v.fold(4);
        assert_eq!(folded.len(), 16);
        // OR semantics -> all these map to index 1
        assert_eq!(folded.num_on_bits(), 1);
        assert!(folded.get_bit(1));
    }

    #[test]
    fn explicit_full_api_basics() {
        let mut bv = ExplicitBitVect::new(32);
        assert_eq!(bv.num_bits(), 32);
        assert_eq!(bv.num_on_bits(), 0);
        assert_eq!(bv.num_off_bits(), 32);

        // set/unset operations
        bv.set_bit(10);
        bv.set_bit(11);
        bv.set_bit(14);
        assert!(bv.get_bit(10));
        assert!(bv.get_bit(11));
        assert!(bv.get_bit(14));
        assert_eq!(bv.num_on_bits(), 3);

        // unset returns previous value
        assert!(bv.unset_bit(14));
        assert!(!bv.get_bit(14));
        assert_eq!(bv.num_on_bits(), 2);

        // bit vector equality
        let mut bv2 = bv.clone();
        assert_eq!(bv, bv2);
        bv2.set_bit(31);
        assert_ne!(bv, bv2);

        // get_on_bits list
        let on = bv.get_on_bits();
        assert_eq!(on, vec![10, 11]);

        // bitwise not
        let complement = !&bv;
        assert!(!complement.get_bit(10));
        assert!(complement.get_bit(14));
        assert_eq!(complement.num_on_bits(), 32 - 2);

        // Tanimoto similarity self
        assert!((bv.tanimoto_similarity(&bv2) - (2.0 / (3.0))).abs() < 1e-6);
    }
}
