mod common;

use std::fs;

use common::{fast_kdf, sample_entry};
use vault_core::{KdfParams, Uuid, Vault, VaultError, FORMAT_VERSION, HEADER_LEN, MAGIC};

const PW: &str = "correct horse battery staple";

fn new_vault(dir: &tempfile::TempDir) -> (std::path::PathBuf, Vault) {
    let path = dir.path().join("test.burrow");
    let v = Vault::create(&path, PW, fast_kdf()).unwrap();
    (path, v)
}

#[test]
fn roundtrip_preserves_entries() {
    let dir = tempfile::tempdir().unwrap();
    let (path, mut v) = new_vault(&dir);
    let e = sample_entry("Example");
    let id = v.add_entry(e.clone());
    v.save().unwrap();
    drop(v);

    let v = Vault::open(&path, PW).unwrap();
    assert_eq!(v.entries().len(), 1);
    let got = v.get(id).unwrap();
    assert_eq!(got.title, e.title);
    assert_eq!(got.username, e.username);
    assert_eq!(got.password, e.password);
    assert_eq!(got.url, e.url);
    assert_eq!(got.notes, e.notes);
    assert_eq!(
        got.modified_at.unix_timestamp(),
        e.modified_at.unix_timestamp()
    );
}

#[test]
fn file_has_magic_and_version_and_no_plaintext() {
    let dir = tempfile::tempdir().unwrap();
    let (path, mut v) = new_vault(&dir);
    v.add_entry(sample_entry("Example"));
    v.save().unwrap();
    let bytes = fs::read(&path).unwrap();
    assert_eq!(&bytes[..8], MAGIC);
    assert_eq!(u16::from_le_bytes([bytes[8], bytes[9]]), FORMAT_VERSION);
    let hay = String::from_utf8_lossy(&bytes);
    assert!(!hay.contains("s3cr3t-p@ss"));
    assert!(!hay.contains("alice@example.com"));
}

#[test]
fn wrong_password_is_rejected() {
    let dir = tempfile::tempdir().unwrap();
    let (path, _v) = new_vault(&dir);
    let err = Vault::open(&path, "wrong").err().unwrap();
    assert!(matches!(err, VaultError::DecryptionFailed), "{err:?}");
}

#[test]
fn nonce_changes_on_every_save() {
    let dir = tempfile::tempdir().unwrap();
    let (path, mut v) = new_vault(&dir);
    let nonce = |p: &std::path::Path| fs::read(p).unwrap()[38..62].to_vec();
    let n1 = nonce(&path);
    v.save().unwrap();
    let n2 = nonce(&path);
    v.save().unwrap();
    let n3 = nonce(&path);
    assert_ne!(n1, n2);
    assert_ne!(n2, n3);
}

#[test]
fn any_flipped_byte_is_detected() {
    let dir = tempfile::tempdir().unwrap();
    let (path, mut v) = new_vault(&dir);
    v.add_entry(sample_entry("Example"));
    v.save().unwrap();
    let original = fs::read(&path).unwrap();

    // Every byte of header and body is either authenticated or validated.
    for i in 0..original.len() {
        let mut bytes = original.clone();
        bytes[i] ^= 0x01;
        fs::write(&path, &bytes).unwrap();
        assert!(
            Vault::open(&path, PW).is_err(),
            "flip at byte {i} was not detected"
        );
    }
}

#[test]
fn truncated_and_garbage_files_are_rejected() {
    let dir = tempfile::tempdir().unwrap();
    let (path, _v) = new_vault(&dir);
    let original = fs::read(&path).unwrap();

    for len in [
        0,
        3,
        8,
        HEADER_LEN - 1,
        HEADER_LEN,
        HEADER_LEN + 5,
        original.len() - 1,
    ] {
        fs::write(&path, &original[..len]).unwrap();
        assert!(
            Vault::open(&path, PW).is_err(),
            "truncation to {len} accepted"
        );
    }

    fs::write(&path, b"this is definitely not a vault file at all, sorry").unwrap();
    assert!(matches!(
        Vault::open(&path, PW).err().unwrap(),
        VaultError::InvalidMagic
    ));
}

#[test]
fn unknown_version_is_rejected() {
    let dir = tempfile::tempdir().unwrap();
    let (path, _v) = new_vault(&dir);
    let mut bytes = fs::read(&path).unwrap();
    bytes[8..10].copy_from_slice(&(FORMAT_VERSION + 1).to_le_bytes());
    fs::write(&path, &bytes).unwrap();
    match Vault::open(&path, PW).err().unwrap() {
        VaultError::UnsupportedVersion(v) => assert_eq!(v, FORMAT_VERSION + 1),
        e => panic!("unexpected error {e:?}"),
    }
}

#[test]
fn absurd_kdf_params_in_file_are_rejected_before_derivation() {
    let dir = tempfile::tempdir().unwrap();
    let (path, _v) = new_vault(&dir);
    let mut bytes = fs::read(&path).unwrap();
    bytes[10..14].copy_from_slice(&u32::MAX.to_le_bytes()); // m_cost ~ 4 TiB
    fs::write(&path, &bytes).unwrap();
    assert!(matches!(
        Vault::open(&path, PW).err().unwrap(),
        VaultError::InvalidKdfParams(_)
    ));
}

#[test]
fn kdf_param_validation() {
    assert!(KdfParams::new(0, 1, 1).is_err());
    assert!(KdfParams::new(8, 0, 1).is_err());
    assert!(KdfParams::new(8, 1, 0).is_err());
    assert!(KdfParams::new(u32::MAX, 1, 1).is_err());
    assert!(KdfParams::new(8, 1, 2).is_err()); // m < 8 * p
    assert!(KdfParams::new(64 * 1024, 3, 1).is_ok());
}

#[test]
fn default_kdf_params_work_end_to_end() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("default.burrow");
    let d = KdfParams::default();
    assert_eq!((d.m_cost(), d.t_cost(), d.parallelism()), (65536, 3, 1));
    let mut v = Vault::create(&path, PW, d).unwrap();
    v.add_entry(sample_entry("x"));
    v.save().unwrap();
    assert_eq!(Vault::open(&path, PW).unwrap().entries().len(), 1);
}

#[test]
fn external_modification_is_detected_and_not_overwritten() {
    let dir = tempfile::tempdir().unwrap();
    let (path, mut a) = new_vault(&dir);
    let mut b = Vault::open(&path, PW).unwrap();

    // Another instance (e.g. a sync tool delivering a change) saves first.
    b.add_entry(sample_entry("from B"));
    b.save().unwrap();
    let on_disk_after_b = fs::read(&path).unwrap();

    a.add_entry(sample_entry("from A"));
    assert!(a.changed_on_disk().unwrap());
    assert!(matches!(
        a.save().err().unwrap(),
        VaultError::ExternalModification
    ));
    assert_eq!(
        fs::read(&path).unwrap(),
        on_disk_after_b,
        "file must be untouched"
    );

    // Explicit override.
    a.save_force().unwrap();
    assert!(!a.changed_on_disk().unwrap());
    let reopened = Vault::open(&path, PW).unwrap();
    assert_eq!(reopened.entries().len(), 1);
    assert_eq!(reopened.entries()[0].title, "from A");
}

#[test]
fn deleted_file_counts_as_external_modification() {
    let dir = tempfile::tempdir().unwrap();
    let (path, mut v) = new_vault(&dir);
    fs::remove_file(&path).unwrap();
    assert!(matches!(
        v.save().err().unwrap(),
        VaultError::ExternalModification
    ));
    v.save_force().unwrap();
    assert!(path.exists());
}

#[test]
fn own_saves_do_not_trigger_false_positives() {
    let dir = tempfile::tempdir().unwrap();
    let (_path, mut v) = new_vault(&dir);
    for i in 0..3 {
        v.add_entry(sample_entry(&format!("e{i}")));
        v.save().unwrap();
    }
    assert!(!v.changed_on_disk().unwrap());
}

#[test]
fn save_is_atomic_and_leaves_no_temp_file() {
    let dir = tempfile::tempdir().unwrap();
    let (_path, mut v) = new_vault(&dir);
    v.add_entry(sample_entry("x"));
    v.save().unwrap();
    let names: Vec<_> = fs::read_dir(dir.path())
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    assert_eq!(
        names,
        vec!["test.burrow".to_string()],
        "unexpected files: {names:?}"
    );
}

#[cfg(unix)]
#[test]
fn vault_file_is_private_on_unix() {
    use std::os::unix::fs::PermissionsExt;
    let dir = tempfile::tempdir().unwrap();
    let (path, _v) = new_vault(&dir);
    assert_eq!(
        fs::metadata(&path).unwrap().permissions().mode() & 0o777,
        0o600
    );
}

#[test]
fn create_refuses_to_overwrite_existing_file() {
    let dir = tempfile::tempdir().unwrap();
    let (path, _v) = new_vault(&dir);
    assert!(matches!(
        Vault::create(&path, PW, fast_kdf()).err().unwrap(),
        VaultError::AlreadyExists
    ));
}

#[test]
fn crud_and_search() {
    let dir = tempfile::tempdir().unwrap();
    let (path, mut v) = new_vault(&dir);
    let id = v.add_entry(sample_entry("GitHub"));
    v.add_entry(sample_entry("Bank"));

    assert_eq!(v.search("git").len(), 1);
    assert_eq!(v.search("ALICE").len(), 2); // username, case-insensitive
    assert_eq!(v.search("s3cr3t").len(), 0); // never matches password
    assert_eq!(v.search("").len(), 2);

    let mut e = v.get(id).unwrap().clone();
    let old_ts = e.modified_at;
    e.title = "GitHub (work)".into();
    std::thread::sleep(std::time::Duration::from_millis(5));
    v.update_entry(e).unwrap();
    assert!(v.get(id).unwrap().modified_at >= old_ts);
    assert_eq!(v.get(id).unwrap().title, "GitHub (work)");

    v.remove_entry(id).unwrap();
    assert!(v.get(id).is_none());
    assert!(matches!(
        v.remove_entry(id).err().unwrap(),
        VaultError::EntryNotFound(_)
    ));
    assert!(matches!(
        v.update_entry(sample_entry("ghost")).err().unwrap(),
        VaultError::EntryNotFound(_)
    ));

    v.save().unwrap();
    assert_eq!(Vault::open(&path, PW).unwrap().entries().len(), 1);
}

#[test]
fn entry_ids_are_unique_uuids() {
    let a = sample_entry("a");
    let b = sample_entry("b");
    assert_ne!(a.id, b.id);
    assert_ne!(a.id, Uuid::nil());
}

#[test]
fn export_copies_encrypted_file_and_refuses_self_or_stale() {
    let dir = tempfile::tempdir().unwrap();
    let (path, mut v) = new_vault(&dir);
    v.add_entry(sample_entry("x"));
    v.save().unwrap();

    let dest = dir.path().join("backup.burrow");
    v.export_to(&dest).unwrap();
    assert_eq!(fs::read(&dest).unwrap(), fs::read(&path).unwrap());
    assert_eq!(Vault::open(&dest, PW).unwrap().entries().len(), 1);

    // Re-exporting replaces the previous backup; exporting onto itself is refused.
    v.export_to(&dest).unwrap();
    assert!(matches!(
        v.export_to(&path).err().unwrap(),
        VaultError::AlreadyExists
    ));

    let mut other = Vault::open(&path, PW).unwrap();
    other.save().unwrap(); // changes the file on disk behind `v`
    assert!(matches!(
        v.export_to(dir.path().join("stale.burrow")).err().unwrap(),
        VaultError::ExternalModification
    ));
}
