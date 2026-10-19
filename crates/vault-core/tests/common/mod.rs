#![allow(dead_code)]
use vault_core::{Entry, KdfParams};

/// Cheap parameters so tests run fast. The default is tested separately.
pub fn fast_kdf() -> KdfParams {
    KdfParams::new(8, 1, 1).unwrap()
}

pub fn sample_entry(title: &str) -> Entry {
    let mut e = Entry::new(title);
    e.username = "alice@example.com".into();
    e.password = "s3cr3t-p@ss".into();
    e.url = "https://example.com".into();
    e.notes = "some notes\nsecond line é 🔐".into();
    e
}
