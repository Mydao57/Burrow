use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::{
    aead::{Aead, KeyInit, Payload},
    Key, XChaCha20Poly1305, XNonce,
};
use rand::{rngs::OsRng, RngCore};
use zeroize::Zeroizing;

use crate::error::{Result, VaultError};
use crate::format::{KdfParams, NONCE_LEN, SALT_LEN};

pub type SecretKey = Zeroizing<[u8; 32]>;

pub fn random_salt() -> [u8; SALT_LEN] {
    let mut s = [0u8; SALT_LEN];
    OsRng.fill_bytes(&mut s);
    s
}

pub fn random_nonce() -> [u8; NONCE_LEN] {
    let mut n = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut n);
    n
}

pub fn derive_key(password: &str, salt: &[u8; SALT_LEN], kdf: &KdfParams) -> Result<SecretKey> {
    let params = Params::new(kdf.m_cost(), kdf.t_cost(), kdf.parallelism(), Some(32))
        .map_err(|e| VaultError::InvalidKdfParams(e.to_string()))?;
    let argon = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut key = Zeroizing::new([0u8; 32]);
    argon
        .hash_password_into(password.as_bytes(), salt, key.as_mut())
        .map_err(|_| VaultError::Kdf)?;
    Ok(key)
}

pub fn encrypt(key: &SecretKey, nonce: &[u8; NONCE_LEN], aad: &[u8], plaintext: &[u8]) -> Result<Vec<u8>> {
    let cipher = XChaCha20Poly1305::new(Key::from_slice(key.as_ref()));
    cipher
        .encrypt(XNonce::from_slice(nonce), Payload { msg: plaintext, aad })
        .map_err(|_| VaultError::EncryptionFailed)
}

pub fn decrypt(key: &SecretKey, nonce: &[u8; NONCE_LEN], aad: &[u8], ciphertext: &[u8]) -> Result<Zeroizing<Vec<u8>>> {
    let cipher = XChaCha20Poly1305::new(Key::from_slice(key.as_ref()));
    cipher
        .decrypt(XNonce::from_slice(nonce), Payload { msg: ciphertext, aad })
        .map(Zeroizing::new)
        .map_err(|_| VaultError::DecryptionFailed)
}
