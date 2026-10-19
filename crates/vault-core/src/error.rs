use std::io;

use thiserror::Error;
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, VaultError>;

#[derive(Debug, Error)]
pub enum VaultError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("not a Burrow vault file (bad magic bytes)")]
    InvalidMagic,
    #[error("unsupported vault format version {0}")]
    UnsupportedVersion(u16),
    #[error("vault file is truncated or corrupted")]
    Corrupted,
    #[error("invalid key-derivation parameters: {0}")]
    InvalidKdfParams(String),
    #[error("key derivation failed")]
    Kdf,
    /// Wrong master password *or* tampered file: AEAD cannot tell them apart.
    #[error("decryption failed: wrong master password or corrupted file")]
    DecryptionFailed,
    #[error("encryption failed")]
    EncryptionFailed,
    #[error("the vault file changed on disk since it was opened")]
    ExternalModification,
    #[error("a file already exists at this location")]
    AlreadyExists,
    #[error("could not (de)serialize vault contents: {0}")]
    Serialization(String),
    #[error("entry {0} not found")]
    EntryNotFound(Uuid),
    #[error("invalid generator options: {0}")]
    InvalidGeneratorOptions(&'static str),
}
