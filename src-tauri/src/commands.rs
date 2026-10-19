use std::path::PathBuf;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, State};
use tauri_plugin_dialog::DialogExt;
use time::format_description::well_known::Rfc3339;
use vault_core::{Entry, GeneratorOptions, KdfParams, Uuid, Vault};
use zeroize::Zeroizing;

use crate::error::{CmdResult, CommandError};
use crate::state::AppState;

type AppStateRef<'a> = State<'a, Arc<AppState>>;

// ---------- DTOs (mirrored by hand in src/shared/api/types.ts) ----------

#[derive(Serialize)]
pub struct VaultStatus {
    unlocked: bool,
    selected_path: Option<String>,
    auto_lock_minutes: u64,
}

#[derive(Serialize)]
pub struct VaultInfo {
    path: String,
    entry_count: usize,
}

/// List view: deliberately has no password and no notes.
#[derive(Serialize)]
pub struct EntrySummary {
    id: Uuid,
    title: String,
    username: String,
    url: String,
    modified_at: String,
}

/// Detail view: notes included, password still absent (see `reveal_password`).
#[derive(Serialize)]
pub struct EntryDetail {
    #[serde(flatten)]
    summary: EntrySummary,
    notes: String,
}

#[derive(Deserialize)]
pub struct EntryInput {
    id: Option<Uuid>,
    title: String,
    username: String,
    /// `None` or empty on update means "keep the current password".
    password: Option<String>,
    url: String,
    notes: String,
}

#[derive(Deserialize)]
pub struct GeneratorInput {
    length: usize,
    lowercase: bool,
    uppercase: bool,
    digits: bool,
    symbols: bool,
    exclude_ambiguous: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FileDialogMode {
    Open,
    Create,
}

fn summary(e: &Entry) -> EntrySummary {
    EntrySummary {
        id: e.id,
        title: e.title.clone(),
        username: e.username.clone(),
        url: e.url.clone(),
        modified_at: e.modified_at.format(&Rfc3339).unwrap_or_default(),
    }
}

fn vault_info(v: &Vault) -> VaultInfo {
    VaultInfo { path: v.path().display().to_string(), entry_count: v.entries().len() }
}

// ---------- vault lifecycle ----------

#[tauri::command(async)]
pub fn vault_status(state: AppStateRef) -> VaultStatus {
    VaultStatus {
        unlocked: state.is_unlocked(),
        selected_path: state.selected_path().map(|p| p.display().to_string()),
        auto_lock_minutes: state.auto_lock_minutes(),
    }
}

// Argon2 runs on Tauri's worker pool: `command(async)` keeps it off the main thread.

/// Native file dialog, run on the Rust side so the webview never chooses paths.
#[tauri::command(async)]
pub fn choose_vault_file(app: AppHandle, state: AppStateRef, mode: FileDialogMode) -> CmdResult<Option<String>> {
    let builder = app.dialog().file().add_filter("Burrow vault", &["burrow"]);
    let picked = match mode {
        FileDialogMode::Open => builder.blocking_pick_file(),
        FileDialogMode::Create => builder.set_file_name("vault.burrow").blocking_save_file(),
    };
    let Some(picked) = picked else { return Ok(None) };
    let mut path: PathBuf = picked
        .into_path()
        .map_err(|e| CommandError::new("io", e.to_string()))?;
    if matches!(mode, FileDialogMode::Create) && path.extension().is_none() {
        path.set_extension("burrow");
    }
    let shown = path.display().to_string();
    state.select_path(Some(path));
    Ok(Some(shown))
}

#[tauri::command(async)]
pub fn create_vault(state: AppStateRef, master_password: String) -> CmdResult<VaultInfo> {
    let password = Zeroizing::new(master_password);
    let path = state.selected_path().ok_or_else(CommandError::no_file)?;
    let vault = Vault::create(path, &password, KdfParams::default())?;
    let info = vault_info(&vault);
    state.install_vault(vault);
    state.persist();
    Ok(info)
}

#[tauri::command(async)]
pub fn unlock_vault(state: AppStateRef, master_password: String) -> CmdResult<VaultInfo> {
    let password = Zeroizing::new(master_password);
    let path = state.selected_path().ok_or_else(CommandError::no_file)?;
    let vault = Vault::open(path, &password)?;
    let info = vault_info(&vault);
    state.install_vault(vault);
    state.persist();
    Ok(info)
}

/// Re-reads the file from disk (after an external change), discarding the
/// in-memory copy. Requires the master password again.
#[tauri::command(async)]
pub fn reload_vault(state: AppStateRef, master_password: String) -> CmdResult<VaultInfo> {
    let password = Zeroizing::new(master_password);
    let path = state.with_vault(|v| Ok(v.path().to_path_buf()))?;
    let vault = Vault::open(path, &password)?;
    let info = vault_info(&vault);
    state.install_vault(vault);
    Ok(info)
}

#[tauri::command(async)]
pub fn lock_vault(state: AppStateRef) {
    state.lock_vault();
}

#[tauri::command(async)]
pub fn touch_activity(state: AppStateRef) {
    if state.is_unlocked() {
        state.touch();
    }
}

#[tauri::command(async)]
pub fn set_auto_lock_minutes(state: AppStateRef, minutes: u64) -> CmdResult<()> {
    if minutes == 0 || minutes > 24 * 60 {
        return Err(CommandError::new("invalid_options", "auto-lock delay must be 1 minute to 24 hours"));
    }
    state.set_auto_lock_minutes(minutes);
    Ok(())
}

// ---------- entries ----------

#[tauri::command(async)]
pub fn list_entries(state: AppStateRef, query: Option<String>) -> CmdResult<Vec<EntrySummary>> {
    state.with_vault(|v| {
        let mut list: Vec<_> = v.search(query.as_deref().unwrap_or("")).into_iter().map(summary).collect();
        list.sort_by_key(|e| e.title.to_lowercase());
        Ok(list)
    })
}

#[tauri::command(async)]
pub fn get_entry(state: AppStateRef, id: Uuid) -> CmdResult<EntryDetail> {
    state.with_vault(|v| {
        let e = v.get(id).ok_or(vault_core::VaultError::EntryNotFound(id))?;
        Ok(EntryDetail { summary: summary(e), notes: e.notes.clone() })
    })
}

/// The only command that returns a stored password to the webview.
#[tauri::command(async)]
pub fn reveal_password(state: AppStateRef, id: Uuid) -> CmdResult<String> {
    state.with_vault(|v| {
        let e = v.get(id).ok_or(vault_core::VaultError::EntryNotFound(id))?;
        Ok(e.password.clone())
    })
}

fn copy_field(state: &AppState, id: Uuid, pick: impl FnOnce(&Entry) -> &str) -> CmdResult<()> {
    let text = state.with_vault(|v| {
        let e = v.get(id).ok_or(vault_core::VaultError::EntryNotFound(id))?;
        Ok(Zeroizing::new(pick(e).to_owned()))
    })?;
    state
        .clipboard
        .copy(&text)
        .map_err(|m| CommandError::new("clipboard", m))
}

#[tauri::command(async)]
pub fn copy_password(state: AppStateRef, id: Uuid) -> CmdResult<()> {
    copy_field(&state, id, |e| &e.password)
}

#[tauri::command(async)]
pub fn copy_username(state: AppStateRef, id: Uuid) -> CmdResult<()> {
    copy_field(&state, id, |e| &e.username)
}

/// Adds or updates an entry and saves. With `force`, overwrites a file that
/// changed on disk; without it, returns `external_modification`.
#[tauri::command(async)]
pub fn save_entry(state: AppStateRef, input: EntryInput, force: bool) -> CmdResult<EntrySummary> {
    let title = input.title.trim().to_owned();
    if title.is_empty() {
        return Err(CommandError::new("invalid_input", "Title is required"));
    }
    state.with_vault(|v| {
        let saved = match input.id {
            Some(id) => {
                let mut e = v.get(id).cloned().ok_or(vault_core::VaultError::EntryNotFound(id))?;
                e.title = title;
                e.username = input.username;
                e.url = input.url;
                e.notes = input.notes;
                if let Some(p) = input.password.filter(|p| !p.is_empty()) {
                    e.password = p;
                }
                v.update_entry(e)?;
                v.get(id).map(summary).expect("entry was just updated")
            }
            None => {
                let mut e = Entry::new(title);
                e.username = input.username;
                e.password = input.password.unwrap_or_default();
                e.url = input.url;
                e.notes = input.notes;
                let s = summary(&e);
                v.add_entry(e);
                s
            }
        };
        if force { v.save_force()? } else { v.save()? }
        Ok(saved)
    })
}

#[tauri::command(async)]
pub fn delete_entry(state: AppStateRef, id: Uuid, force: bool) -> CmdResult<()> {
    state.with_vault(|v| {
        v.remove_entry(id)?;
        if force { v.save_force() } else { v.save() }
    })
}

// ---------- tools ----------

#[tauri::command(async)]
pub fn generate_password(opts: GeneratorInput) -> CmdResult<String> {
    Ok(vault_core::generate_password(&GeneratorOptions {
        length: opts.length,
        lowercase: opts.lowercase,
        uppercase: opts.uppercase,
        digits: opts.digits,
        symbols: opts.symbols,
        exclude_ambiguous: opts.exclude_ambiguous,
    })?)
}

/// Copies the encrypted vault file to a location chosen in a native dialog.
/// Returns the destination, or `None` if the user cancelled.
#[tauri::command(async)]
pub fn export_vault(app: AppHandle, state: AppStateRef) -> CmdResult<Option<String>> {
    let picked = app
        .dialog()
        .file()
        .add_filter("Burrow vault", &["burrow"])
        .set_file_name("burrow-backup.burrow")
        .blocking_save_file();
    let Some(picked) = picked else { return Ok(None) };
    let dest = picked.into_path().map_err(|e| CommandError::new("io", e.to_string()))?;
    state.with_vault(|v| v.export_to(&dest))?;
    Ok(Some(dest.display().to_string()))
}
