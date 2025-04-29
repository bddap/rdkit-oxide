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
    }
}
