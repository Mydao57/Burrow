use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};
use uuid::Uuid;
use zeroize::Zeroizing;

use crate::crypto::{self, SecretKey};
use crate::error::{Result, VaultError};
use crate::format::{Header, KdfParams, FORMAT_VERSION, SALT_LEN};
use crate::model::{Entry, VaultData};

/// Content hash of the encrypted file as last read or written by us.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Fingerprint([u8; 32]);

impl Fingerprint {
    fn of(bytes: &[u8]) -> Self {
        Self(Sha256::digest(bytes).into())
    }
}

/// An unlocked vault. Holds the derived key and decrypted entries in memory;
/// the key is zeroized on drop.
pub struct Vault {
    path: PathBuf,
    key: SecretKey,
    salt: [u8; SALT_LEN],
    kdf: KdfParams,
    entries: Vec<Entry>,
    /// `None` until the file exists on disk.
    fingerprint: Option<Fingerprint>,
}

impl Vault {
    /// Creates a new vault and writes it to `path`. Fails if the path exists.
    pub fn create(path: impl AsRef<Path>, password: &str, kdf: KdfParams) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        if path.exists() {
            return Err(VaultError::AlreadyExists);
        }
        let salt = crypto::random_salt();
        let key = crypto::derive_key(password, &salt, &kdf)?;
        let mut vault = Vault {
            path,
            key,
            salt,
            kdf,
            entries: Vec::new(),
            fingerprint: None,
        };
        vault.write(false)?;
        Ok(vault)
    }

    pub fn open(path: impl AsRef<Path>, password: &str) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let bytes = fs::read(&path)?;
        let (header, body) = Header::parse(&bytes)?;
        let key = crypto::derive_key(password, &header.salt, &header.kdf)?;
        let aad = &bytes[..crate::format::HEADER_LEN];
        let plaintext = crypto::decrypt(&key, &header.nonce, aad, body)?;
        let data: VaultData = serde_json::from_slice(&plaintext)
            .map_err(|e| VaultError::Serialization(e.to_string()))?;
        Ok(Vault {
            path,
            key,
            salt: header.salt,
            kdf: header.kdf,
            entries: data.entries,
            fingerprint: Some(Fingerprint::of(&bytes)),
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }

    pub fn get(&self, id: Uuid) -> Option<&Entry> {
        self.entries.iter().find(|e| e.id == id)
    }

    pub fn search(&self, query: &str) -> Vec<&Entry> {
        self.entries.iter().filter(|e| e.matches(query)).collect()
    }

    /// In-memory changes only; call [`Vault::save`] to persist.
    pub fn add_entry(&mut self, entry: Entry) -> Uuid {
        let id = entry.id;
        self.entries.push(entry);
        id
    }

    /// Replaces the entry with the same id and bumps `modified_at`.
    pub fn update_entry(&mut self, mut entry: Entry) -> Result<()> {
        let slot = self
            .entries
            .iter_mut()
            .find(|e| e.id == entry.id)
            .ok_or(VaultError::EntryNotFound(entry.id))?;
        entry.touch();
        *slot = entry;
        Ok(())
    }

    pub fn remove_entry(&mut self, id: Uuid) -> Result<Entry> {
        let idx = self
            .entries
            .iter()
            .position(|e| e.id == id)
            .ok_or(VaultError::EntryNotFound(id))?;
        Ok(self.entries.remove(idx))
    }

    /// Persists the vault. Fails with [`VaultError::ExternalModification`] if
    /// the file on disk differs from what we last read or wrote.
    pub fn save(&mut self) -> Result<()> {
        self.write(false)
    }

    /// Persists the vault even if the file changed on disk (overwrites it).
    pub fn save_force(&mut self) -> Result<()> {
        self.write(true)
    }

    /// True if the file on disk no longer matches what we last read or wrote.
    pub fn changed_on_disk(&self) -> Result<bool> {
        match (&self.fingerprint, fs::read(&self.path)) {
            (Some(fp), Ok(bytes)) => Ok(*fp != Fingerprint::of(&bytes)),
            (Some(_), Err(e)) if e.kind() == std::io::ErrorKind::NotFound => Ok(true),
            (Some(_), Err(e)) => Err(e.into()),
            (None, Ok(_)) => Ok(true),
            (None, Err(e)) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
            (None, Err(e)) => Err(e.into()),
        }
    }

    fn write(&mut self, force: bool) -> Result<()> {
        if !force && self.changed_on_disk()? {
            return Err(VaultError::ExternalModification);
        }
        let header = Header {
            version: FORMAT_VERSION,
            kdf: self.kdf,
            salt: self.salt,
            nonce: crypto::random_nonce(), // fresh nonce on every save
        };
        let header_bytes = header.to_bytes();
        let plaintext = Zeroizing::new(
            serde_json::to_vec(&VaultData {
                entries: self.entries.clone(),
            })
            .map_err(|e| VaultError::Serialization(e.to_string()))?,
        );
        let ciphertext = crypto::encrypt(&self.key, &header.nonce, &header_bytes, &plaintext)?;

        let mut out = Vec::with_capacity(header_bytes.len() + ciphertext.len());
        out.extend_from_slice(&header_bytes);
        out.extend_from_slice(&ciphertext);

        atomic_write(&self.path, &out)?;
        self.fingerprint = Some(Fingerprint::of(&out));
        Ok(())
    }

    /// Copies the encrypted file (as currently on disk) to `dest`, replacing
    /// any existing file there. Refuses to target the vault file itself and
    /// to export a stale copy.
    pub fn export_to(&self, dest: impl AsRef<Path>) -> Result<()> {
        let dest = dest.as_ref();
        if let (Ok(a), Ok(b)) = (fs::canonicalize(dest), fs::canonicalize(&self.path)) {
            if a == b {
                return Err(VaultError::AlreadyExists);
            }
        }
        let bytes = fs::read(&self.path)?;
        if self.fingerprint != Some(Fingerprint::of(&bytes)) {
            return Err(VaultError::ExternalModification);
        }
        atomic_write(dest, &bytes)
    }
}

/// Write to a temp file in the same directory, fsync, then rename over `path`.
fn atomic_write(path: &Path, bytes: &[u8]) -> Result<()> {
    let dir = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let name = path.file_name().ok_or_else(|| {
        VaultError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "path has no file name",
        ))
    })?;
    let mut tmp_name = std::ffi::OsString::from(".");
    tmp_name.push(name);
    tmp_name.push(".tmp");
    let tmp = dir.join(tmp_name);

    let result = (|| -> std::io::Result<()> {
        let mut opts = OpenOptions::new();
        opts.write(true).create(true).truncate(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            opts.mode(0o600);
        }
        let mut f = opts.open(&tmp)?;
        f.write_all(bytes)?;
        f.sync_all()?;
        drop(f);
        fs::rename(&tmp, path)?;
        #[cfg(unix)]
        File::open(dir)?.sync_all()?; // persist the rename
        Ok(())
    })();

    if result.is_err() {
        let _ = fs::remove_file(&tmp);
    }
    Ok(result?)
}
