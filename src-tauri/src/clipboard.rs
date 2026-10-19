//! Clipboard access with automatic clearing.
//!
//! We only clear the clipboard if it still holds what *we* put there, so we
//! never wipe something the user copied in the meantime.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use sha2::{Digest, Sha256};

pub const CLEAR_AFTER: Duration = Duration::from_secs(20);

#[derive(Default)]
struct Inner {
    /// Kept alive for the process lifetime: on X11 the owner must keep serving.
    clipboard: Option<arboard::Clipboard>,
    generation: u64,
    last_hash: Option<[u8; 32]>,
}

#[derive(Default)]
pub struct ClipboardGuard {
    inner: Mutex<Inner>,
}

fn hash(text: &str) -> [u8; 32] {
    Sha256::digest(text.as_bytes()).into()
}

impl ClipboardGuard {
    pub fn copy(self: &Arc<Self>, text: &str) -> Result<(), String> {
        let generation = {
            let mut g = self.inner.lock().unwrap_or_else(|e| e.into_inner());
            if g.clipboard.is_none() {
                g.clipboard = Some(arboard::Clipboard::new().map_err(|e| e.to_string())?);
            }
            let cb = g.clipboard.as_mut().expect("just initialised");
            set_text_excluded_from_history(cb, text).map_err(|e| e.to_string())?;
            g.generation += 1;
            g.last_hash = Some(hash(text));
            g.generation
        };

        let this = Arc::clone(self);
        thread::spawn(move || {
            thread::sleep(CLEAR_AFTER);
            this.clear_if_ours(Some(generation));
        });
        Ok(())
    }

    /// Clears the clipboard if it still contains our last copy. With
    /// `generation`, does nothing if a newer copy has happened since.
    pub fn clear_if_ours(&self, generation: Option<u64>) {
        let mut g = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        if generation.is_some_and(|gen| gen != g.generation) {
            return;
        }
        let Some(expected) = g.last_hash else { return };
        if let Some(cb) = g.clipboard.as_mut() {
            if cb.get_text().map(|t| hash(&t) == expected).unwrap_or(false) {
                let _ = cb.clear();
            }
        }
        g.last_hash = None;
    }
}

#[cfg(target_os = "linux")]
fn set_text_excluded_from_history(
    cb: &mut arboard::Clipboard,
    text: &str,
) -> Result<(), arboard::Error> {
    use arboard::SetExtLinux;
    cb.set().exclude_from_history().text(text.to_owned())
}

#[cfg(target_os = "windows")]
fn set_text_excluded_from_history(
    cb: &mut arboard::Clipboard,
    text: &str,
) -> Result<(), arboard::Error> {
    use arboard::SetExtWindows;
    cb.set()
        .exclude_from_cloud()
        .exclude_from_history()
        .text(text.to_owned())
}

#[cfg(target_os = "macos")]
fn set_text_excluded_from_history(
    cb: &mut arboard::Clipboard,
    text: &str,
) -> Result<(), arboard::Error> {
    use arboard::SetExtApple;
    cb.set().exclude_from_history().text(text.to_owned())
}
