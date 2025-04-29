//! Data structures (bit vectors, sparse vectors, etc.)

use bitvec::prelude::*;


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
pub struct ExplicitBitVect {
    bits: BitVec<u8, Lsb0>,
}

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

    pub fn num_on_bits(&self) -> u32 {
        self.bits.count_ones() as u32
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
}
