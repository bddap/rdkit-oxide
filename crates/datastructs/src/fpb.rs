//! FPB (Fingerprint Binary) reader.
//!
//! This is a *minimal* implementation of RDKit's FPB file format, just
//! sufficient for the first milestones of the ongoing Rust port.  It can:
//!
//! * Validate the magic header.
//! * Parse the `AREN` chunk to obtain the fingerprint arena.
//! * Lazily access individual fingerprints as `ExplicitBitVect` values.
//!
//! Many optional chunks present in full-featured FPB files (`POPC`, `HASH`,
//! `FPID`, …) are currently ignored.  Support can be added later if unit‐tests
//! require it.

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

use byteorder::{LittleEndian, ReadBytesExt};

use super::ExplicitBitVect;

const FPB_MAGIC: &[u8; 8] = b"FPB1\r\n\0\0";

/// Errors which can occur while reading an FPB file.
#[derive(thiserror::Error, Debug)]
pub enum FpbError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid FPB magic header")] // wrong header.
    InvalidMagic,
    #[error("Malformed FPB – missing AREN chunk")] // essential chunk missing.
    MissingArena,
    #[error("Fingerprint index {0} out of bounds (len = {1})")]
    IndexOutOfBounds(usize, usize),
}

/// Reader giving access to fingerprints stored in an FPB file.
///
/// For now the entire file is read into memory on construction.  If this turns
/// out to be a bottleneck we can add a mmap-based backend or true lazy I/O.
pub struct FpbReader {
    bytes: Vec<u8>,       // complete file content
    n_bits: usize,        // number of bits per fingerprint
    storage_size: usize,  // padded storage size per fingerprint (bytes)
    fp_offset: usize,     // offset of first fingerprint within `bytes`
    len: usize,           // number of fingerprints in arena
}

impl FpbReader {
    /// Load an FPB file from the given path.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, FpbError> {
        let mut file = File::open(path)?;
        let metadata = file.metadata()?;
        let mut bytes = Vec::with_capacity(metadata.len() as usize);
        file.read_to_end(&mut bytes)?;

        Self::from_bytes(bytes)
    }

    /// Parse an FPB file from its raw bytes.
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, FpbError> {
        if bytes.len() < FPB_MAGIC.len() || &bytes[..8] != FPB_MAGIC {
            return Err(FpbError::InvalidMagic);
        }

        let mut cursor = std::io::Cursor::new(&bytes);
        cursor.seek(SeekFrom::Start(8))?; // skip magic header.

        let mut fp_offset = None;
        let mut n_bits = 0usize;
        let mut storage_size = 0usize;
        let mut len = 0usize;

        loop {
            // If we reached the end of the file, break.
            if cursor.position() as usize >= bytes.len() {
                break;
            }

            let chunk_size = match cursor.read_u64::<LittleEndian>() {
                Ok(v) => v as usize,
                Err(e) => {
                    // We reached EOF unexpectedly.
                    if e.kind() == std::io::ErrorKind::UnexpectedEof {
                        break;
                    } else {
                        return Err(FpbError::Io(e));
                    }
                }
            };

            // Read 4-byte chunk name.
            let mut name_buf = [0u8; 4];
            cursor.read_exact(&mut name_buf)?;
            let name = name_buf;

            let data_start = cursor.position() as usize;

            match &name {
                b"AREN" => {
                    // Minimum header is 4+4+1 bytes.
                    if chunk_size < 9 {
                        return Err(FpbError::MissingArena);
                    }

                    // Parse arena details.
                    let num_bytes_per_fp = cursor.read_u32::<LittleEndian>()? as usize;
                    n_bits = num_bytes_per_fp * 8;

                    storage_size = cursor.read_u32::<LittleEndian>()? as usize;
                    let spacer = cursor.read_u8()? as usize;

                    // Skip spacer bytes.
                    cursor.seek(SeekFrom::Current(spacer as i64))?;

                    let fingerprints_offset = cursor.position() as usize;
                    fp_offset = Some(fingerprints_offset);

                    // Number of fingerprints.
                    len = (chunk_size - 9 - spacer) / storage_size;

                    // Jump to the end of this chunk.
                    cursor.seek(SeekFrom::Start((data_start + chunk_size) as u64))?;
                }
                b"FEND" => {
                    // End marker – we can stop parsing.
                    break;
                }
                _ => {
                    // Unrecognised chunk.  Skip.
                    cursor.seek(SeekFrom::Start((data_start + chunk_size) as u64))?;
                }
            }
        }

        let fp_offset = fp_offset.ok_or(FpbError::MissingArena)?;

        Ok(Self {
            bytes,
            n_bits,
            storage_size,
            fp_offset,
            len,
        })
    }

    /// Number of fingerprints stored in the file.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if there are no fingerprints.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Number of bits per fingerprint.
    pub fn num_bits(&self) -> usize {
        self.n_bits
    }

    /// Get fingerprint at the given index.
    pub fn fingerprint(&self, idx: usize) -> Result<ExplicitBitVect, FpbError> {
        if idx >= self.len {
            return Err(FpbError::IndexOutOfBounds(idx, self.len));
        }

        let start = self.fp_offset + idx * self.storage_size;
        let end = start + (self.n_bits / 8);

        let bytes = &self.bytes[start..end];

        Ok(Self::bytes_to_bitvect(bytes, self.n_bits))
    }

    fn bytes_to_bitvect(bytes: &[u8], n_bits: usize) -> ExplicitBitVect {
        let mut bv = ExplicitBitVect::new(n_bits);

        for (byte_idx, byte) in bytes.iter().enumerate() {
            let base_bit = byte_idx * 8;
            for bit_pos in 0..8 {
                if byte & (1u8 << bit_pos) != 0 {
                    bv.set_bit(base_bit + bit_pos);
                }
            }
        }

        bv
    }
}

// -------------------------------------------------------------------------
// Unit tests – construct a minimal FPB file in-memory and verify parsing.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_minimal_fpb() {
        // Two 64-bit fingerprints: 0b1011… and 0b0101… (arbitrary)
        let fp1 = 0b1011_0001u64.to_le_bytes(); // 8 bytes
        let fp2 = 0b0101_1110u64.to_le_bytes();

        let num_bytes_per_fp = 8u32; // 64 bits
        let storage_size = 8u32; // no padding
        let spacer_size = 0u8;

        // Build AREN chunk.
        let mut aren_chunk = Vec::new();
        aren_chunk.extend_from_slice(&num_bytes_per_fp.to_le_bytes());
        aren_chunk.extend_from_slice(&storage_size.to_le_bytes());
        aren_chunk.push(spacer_size);
        // no spacer bytes.
        aren_chunk.extend_from_slice(&fp1);
        aren_chunk.extend_from_slice(&fp2);

        let chunk_size = aren_chunk.len() as u64;

        let mut bytes = Vec::new();
        bytes.extend_from_slice(FPB_MAGIC);

        // AREN header: size + name
        bytes.extend_from_slice(&chunk_size.to_le_bytes());
        bytes.extend_from_slice(b"AREN");
        bytes.extend_from_slice(&aren_chunk);

        // FEND (size 0)
        bytes.extend_from_slice(&0u64.to_le_bytes());
        bytes.extend_from_slice(b"FEND");

        // Parse.
        let reader = FpbReader::from_bytes(bytes).expect("parse fpb");
        assert_eq!(reader.len(), 2);
        assert_eq!(reader.num_bits(), 64);

        let bv1 = reader.fingerprint(0).unwrap();
        assert!(bv1.get_bit(0)); // least-significant bit set.
        assert!(bv1.get_bit(4)); // 0b1011_0001 -> bits 0,4,5,7 etc? Wait compute.

        // Spots: let's just verify num bits.
        assert_eq!(bv1.num_on_bits(), fp1.iter().map(|b| b.count_ones()).sum());

        let bv2 = reader.fingerprint(1).unwrap();
        assert_eq!(bv2.num_on_bits(), fp2.iter().map(|b| b.count_ones()).sum());

        // Out of bounds
        assert!(reader.fingerprint(2).is_err());
    }
}
