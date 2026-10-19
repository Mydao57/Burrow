use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Instant;

use tauri::{AppHandle, Emitter};
use vault_core::Vault;

use crate::clipboard::ClipboardGuard;
use crate::config::Config;
use crate::error::{CmdResult, CommandError};

pub struct AppState {
    vault: Mutex<Option<Vault>>,
    /// File chosen through the native dialog (or restored from settings).
    /// The webview never supplies paths: it can only pick them via a dialog.
    selected_path: Mutex<Option<PathBuf>>,
    last_activity: Mutex<Instant>,
    auto_lock_secs: AtomicU64,
    pub clipboard: Arc<ClipboardGuard>,
    pub config_file: PathBuf,
}

fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    // A panic while holding the lock must not make the vault unusable (or
    // un-lockable), so recover from poisoning.
    m.lock().unwrap_or_else(|e| e.into_inner())
}

impl AppState {
    pub fn new(config_file: PathBuf) -> Self {
        let cfg = Config::load(&config_file);
        let selected = cfg.last_vault_path.filter(|p| p.is_file());
        Self {
            vault: Mutex::new(None),
            selected_path: Mutex::new(selected),
            last_activity: Mutex::new(Instant::now()),
            auto_lock_secs: AtomicU64::new(cfg.auto_lock_minutes * 60),
            clipboard: Arc::new(ClipboardGuard::default()),
            config_file,
        }
    }

    pub fn touch(&self) {
        *lock(&self.last_activity) = Instant::now();
    }

    pub fn idle_for_too_long(&self) -> bool {
        let limit = self.auto_lock_secs.load(Ordering::Relaxed);
        limit > 0 && lock(&self.last_activity).elapsed().as_secs() >= limit
    }

    pub fn auto_lock_minutes(&self) -> u64 {
        self.auto_lock_secs.load(Ordering::Relaxed) / 60
    }

    pub fn set_auto_lock_minutes(&self, minutes: u64) {
        self.auto_lock_secs.store(minutes * 60, Ordering::Relaxed);
        self.persist();
    }

    pub fn selected_path(&self) -> Option<PathBuf> {
        lock(&self.selected_path).clone()
    }

    pub fn select_path(&self, path: Option<PathBuf>) {
        *lock(&self.selected_path) = path;
    }

    pub fn persist(&self) {
        Config {
            last_vault_path: self.selected_path(),
            auto_lock_minutes: self.auto_lock_minutes(),
        }
        .save(&self.config_file);
    }

    pub fn is_unlocked(&self) -> bool {
        lock(&self.vault).is_some()
    }

    pub fn install_vault(&self, vault: Vault) {
        *lock(&self.vault) = Some(vault);
        self.touch();
    }

    /// Runs `f` on the unlocked vault; counts as user activity.
    pub fn with_vault<T>(
        &self,
        f: impl FnOnce(&mut Vault) -> Result<T, vault_core::VaultError>,
    ) -> CmdResult<T> {
        let mut guard = lock(&self.vault);
        let vault = guard.as_mut().ok_or_else(CommandError::locked)?;
        self.touch();
        Ok(f(vault)?)
    }

    /// Drops the vault (zeroizing key and entries) and clears our clipboard
    /// content. Returns whether a vault was actually unlocked.
    pub fn lock_vault(&self) -> bool {
        let was_unlocked = lock(&self.vault).take().is_some();
        self.clipboard.clear_if_ours(None);
        was_unlocked
    }
}

/// Background watcher that locks the vault after inactivity.
pub fn spawn_auto_lock(app: AppHandle, state: Arc<AppState>) {
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        if state.is_unlocked() && state.idle_for_too_long() && state.lock_vault() {
            let _ = app.emit("vault-locked", "inactivity");
        }
    });
}
