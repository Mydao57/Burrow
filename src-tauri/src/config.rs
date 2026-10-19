//! Tiny non-secret settings file: last vault path and auto-lock delay.

use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

pub const DEFAULT_AUTO_LOCK_MINUTES: u64 = 5;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub last_vault_path: Option<PathBuf>,
    pub auto_lock_minutes: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            last_vault_path: None,
            auto_lock_minutes: DEFAULT_AUTO_LOCK_MINUTES,
        }
    }
}

impl Config {
    pub fn load(file: &Path) -> Self {
        fs::read(file)
            .ok()
            .and_then(|b| serde_json::from_slice(&b).ok())
            .unwrap_or_default()
    }

    /// Best effort: failing to persist a preference must never break the app.
    pub fn save(&self, file: &Path) {
        if let Some(dir) = file.parent() {
            let _ = fs::create_dir_all(dir);
        }
        if let Ok(json) = serde_json::to_vec_pretty(self) {
            let _ = fs::write(file, json);
        }
    }
}
