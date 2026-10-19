//! On-disk layout:
//!
//! ```text
//! 0   8   magic "BURROWPW"
//! 8   2   format version (u16 LE)
//! 10  4   Argon2 m_cost, KiB (u32 LE)
//! 14  4   Argon2 t_cost (u32 LE)
//! 18  4   Argon2 parallelism (u32 LE)
//! 22  16  Argon2 salt
//! 38  24  XChaCha20-Poly1305 nonce
//! 62  ..  ciphertext || 16-byte Poly1305 tag
//! ```
//!
//! The whole 62-byte header is authenticated as AEAD associated data.

use crate::error::{Result, VaultError};

pub const MAGIC: &[u8; 8] = b"BURROWPW";
pub const FORMAT_VERSION: u16 = 1;
pub const SALT_LEN: usize = 16;
pub const NONCE_LEN: usize = 24;
pub const TAG_LEN: usize = 16;
pub const HEADER_LEN: usize = 8 + 2 + 4 + 4 + 4 + SALT_LEN + NONCE_LEN;

// Upper bounds protect against a malicious file forcing huge allocations.
const MAX_M_COST_KIB: u32 = 1 << 20; // 1 GiB
const MAX_T_COST: u32 = 20;
const MAX_PARALLELISM: u32 = 16;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KdfParams {
    m_cost: u32,
    t_cost: u32,
    parallelism: u32,
}

impl KdfParams {
    /// Validated constructor. `m_cost` is in KiB.
    pub fn new(m_cost: u32, t_cost: u32, parallelism: u32) -> Result<Self> {
        if parallelism == 0 || parallelism > MAX_PARALLELISM {
            return Err(VaultError::InvalidKdfParams(format!(
                "parallelism must be 1..={MAX_PARALLELISM}"
            )));
        }
        if m_cost < 8 * parallelism || m_cost > MAX_M_COST_KIB {
            return Err(VaultError::InvalidKdfParams(format!(
                "memory cost must be {}..={MAX_M_COST_KIB} KiB",
                8 * parallelism
            )));
        }
        if t_cost == 0 || t_cost > MAX_T_COST {
            return Err(VaultError::InvalidKdfParams(format!(
                "time cost must be 1..={MAX_T_COST}"
            )));
        }
        Ok(Self { m_cost, t_cost, parallelism })
    }

    pub fn m_cost(&self) -> u32 { self.m_cost }
    pub fn t_cost(&self) -> u32 { self.t_cost }
    pub fn parallelism(&self) -> u32 { self.parallelism }
}

impl Default for KdfParams {
    /// 64 MiB, 3 passes, 1 lane.
    fn default() -> Self {
        Self { m_cost: 64 * 1024, t_cost: 3, parallelism: 1 }
    }
}

#[derive(Clone, Debug)]
pub struct Header {
    pub version: u16,
    pub kdf: KdfParams,
    pub salt: [u8; SALT_LEN],
    pub nonce: [u8; NONCE_LEN],
}

impl Header {
    pub fn to_bytes(&self) -> [u8; HEADER_LEN] {
        let mut out = [0u8; HEADER_LEN];
        out[0..8].copy_from_slice(MAGIC);
        out[8..10].copy_from_slice(&self.version.to_le_bytes());
        out[10..14].copy_from_slice(&self.kdf.m_cost.to_le_bytes());
        out[14..18].copy_from_slice(&self.kdf.t_cost.to_le_bytes());
        out[18..22].copy_from_slice(&self.kdf.parallelism.to_le_bytes());
        out[22..38].copy_from_slice(&self.salt);
        out[38..62].copy_from_slice(&self.nonce);
        out
    }

    /// Parses and validates a header. Returns the header and the remaining
    /// bytes (ciphertext || tag).
    pub fn parse(data: &[u8]) -> Result<(Header, &[u8])> {
        if data.len() < MAGIC.len() || &data[0..8] != MAGIC {
            return Err(VaultError::InvalidMagic);
        }
        if data.len() < HEADER_LEN + TAG_LEN {
            return Err(VaultError::Corrupted);
        }
        let version = u16::from_le_bytes(data[8..10].try_into().unwrap());
        if version != FORMAT_VERSION {
            // Only v1 exists. Future versions: add a migration step here.
            return Err(VaultError::UnsupportedVersion(version));
        }
        let u32_at = |i: usize| u32::from_le_bytes(data[i..i + 4].try_into().unwrap());
        let kdf = KdfParams::new(u32_at(10), u32_at(14), u32_at(18))?;
        let header = Header {
            version,
            kdf,
            salt: data[22..38].try_into().unwrap(),
            nonce: data[38..62].try_into().unwrap(),
        };
        Ok((header, &data[HEADER_LEN..]))
    }
}
