/// Every command the front-end may call. Anything not listed here is denied
/// by Tauri's ACL even if it is registered in the invoke handler.
const COMMANDS: &[&str] = &[
    "vault_status",
    "choose_vault_file",
    "create_vault",
    "unlock_vault",
    "reload_vault",
    "lock_vault",
    "list_entries",
    "get_entry",
    "reveal_password",
    "copy_password",
    "copy_username",
    "save_entry",
    "delete_entry",
    "generate_password",
    "export_vault",
    "touch_activity",
    "set_auto_lock_minutes",
];

fn main() {
    tauri_build::try_build(
        tauri_build::Attributes::new()
            .app_manifest(tauri_build::AppManifest::new().commands(COMMANDS)),
    )
    .expect("failed to run tauri-build");
}
