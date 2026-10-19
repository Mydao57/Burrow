// Hand-written mirror of the DTOs in src-tauri/src/commands.rs.

export interface VaultStatus {
  unlocked: boolean;
  selected_path: string | null;
  auto_lock_minutes: number;
}

export interface VaultInfo {
  path: string;
  entry_count: number;
}

/** List view: no password, no notes. */
export interface EntrySummary {
  id: string;
  title: string;
  username: string;
  url: string;
  modified_at: string;
}

/** Detail view: notes included; the password is fetched with `revealPassword`. */
export interface EntryDetail extends EntrySummary {
  notes: string;
}

export interface EntryInput {
  id?: string;
  title: string;
  username: string;
  /** Omitted or empty on update = keep the current password. */
  password?: string;
  url: string;
  notes: string;
}

export interface GeneratorOptions {
  length: number;
  lowercase: boolean;
  uppercase: boolean;
  digits: boolean;
  symbols: boolean;
  exclude_ambiguous: boolean;
}

export type FileDialogMode = "open" | "create";

export type ApiErrorCode =
  | "locked"
  | "no_file_selected"
  | "io"
  | "invalid_file"
  | "unsupported_version"
  | "corrupted"
  | "kdf"
  | "wrong_password"
  | "encryption"
  | "external_modification"
  | "already_exists"
  | "not_found"
  | "invalid_options"
  | "invalid_input"
  | "clipboard"
  | "internal";
