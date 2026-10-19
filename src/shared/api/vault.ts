import { listen } from "@tauri-apps/api/event";

import { call } from "./invoke";
import type {
  EntryDetail,
  EntryInput,
  EntrySummary,
  FileDialogMode,
  GeneratorOptions,
  VaultInfo,
  VaultStatus,
} from "./types";

// Top-level argument names are camelCase (Tauri converts them to the Rust
// snake_case parameters); fields of nested objects keep their serde names.

export const vaultStatus = () => call<VaultStatus>("vault_status");

export const chooseVaultFile = (mode: FileDialogMode) =>
  call<string | null>("choose_vault_file", { mode });

export const createVault = (masterPassword: string) =>
  call<VaultInfo>("create_vault", { masterPassword });

export const unlockVault = (masterPassword: string) =>
  call<VaultInfo>("unlock_vault", { masterPassword });

export const reloadVault = (masterPassword: string) =>
  call<VaultInfo>("reload_vault", { masterPassword });

export const changeMasterPassword = (currentPassword: string, newPassword: string) =>
  call<void>("change_master_password", { currentPassword, newPassword });

export const lockVault = () => call<void>("lock_vault");

export const touchActivity = () => call<void>("touch_activity");

export const setAutoLockMinutes = (minutes: number) =>
  call<void>("set_auto_lock_minutes", { minutes });

export const listEntries = (query?: string) =>
  call<EntrySummary[]>("list_entries", { query: query ?? null });

export const getEntry = (id: string) => call<EntryDetail>("get_entry", { id });

/** The only call that returns a stored password. */
export const revealPassword = (id: string) => call<string>("reveal_password", { id });

export const copyPassword = (id: string) => call<void>("copy_password", { id });

export const copyUsername = (id: string) => call<void>("copy_username", { id });

export const saveEntry = (input: EntryInput, force = false) =>
  call<EntrySummary>("save_entry", { input, force });

export const deleteEntry = (id: string, force = false) =>
  call<void>("delete_entry", { id, force });

export const generatePassword = (opts: GeneratorOptions) =>
  call<string>("generate_password", { opts });

export const exportVault = () => call<string | null>("export_vault");

/** Emitted by the backend when the vault is locked for inactivity. */
export function onVaultAutoLocked(handler: (reason: string) => void): Promise<() => void> {
  return listen<string>("vault-locked", (e) => handler(e.payload));
}
