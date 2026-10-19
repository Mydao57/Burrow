//! Burrow vault core: file format, cryptography and data model.
//!
//! Pure Rust, no UI or Tauri dependency. Crypto is delegated to the
//! `argon2` (Argon2id) and `chacha20poly1305` (XChaCha20-Poly1305) crates.

mod crypto;
mod error;
mod format;
mod generator;
mod model;
mod vault;

pub use error::{Result, VaultError};
pub use format::{KdfParams, FORMAT_VERSION, HEADER_LEN, MAGIC};
pub use generator::{generate_password, GeneratorOptions};
pub use model::Entry;
pub use uuid::Uuid;
pub use vault::Vault;
