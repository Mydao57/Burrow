use serde::Serialize;
use vault_core::VaultError;

/// Error shape sent to the front-end. `code` is stable and machine-readable;
/// `message` is for display and never contains secrets.
#[derive(Debug, Serialize)]
pub struct CommandError {
    pub code: &'static str,
    pub message: String,
}

impl CommandError {
    pub fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self { code, message: message.into() }
    }
    pub fn locked() -> Self {
        Self::new("locked", "The vault is locked")
    }
    pub fn no_file() -> Self {
        Self::new("no_file_selected", "No vault file selected")
    }
}

impl From<VaultError> for CommandError {
    fn from(e: VaultError) -> Self {
        let code = match &e {
            VaultError::Io(_) => "io",
            VaultError::InvalidMagic => "invalid_file",
            VaultError::UnsupportedVersion(_) => "unsupported_version",
            VaultError::Corrupted => "corrupted",
            VaultError::InvalidKdfParams(_) => "invalid_file",
            VaultError::Kdf => "kdf",
            VaultError::DecryptionFailed => "wrong_password",
            VaultError::EncryptionFailed => "encryption",
            VaultError::ExternalModification => "external_modification",
            VaultError::AlreadyExists => "already_exists",
            VaultError::Serialization(_) => "corrupted",
            VaultError::EntryNotFound(_) => "not_found",
            VaultError::InvalidGeneratorOptions(_) => "invalid_options",
        };
        Self::new(code, e.to_string())
    }
}

pub type CmdResult<T> = Result<T, CommandError>;
