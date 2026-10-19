# Burrow

A small, local-only password manager for macOS, Windows and Linux, built with
[Tauri 2](https://tauri.app) (Rust backend) and React + TypeScript.

> **Status: early, unaudited software.** Burrow has not been reviewed by an
> independent security professional. It uses well-known cryptographic
> libraries, but that does not make the application as a whole secure. Do not
> make it your only copy of credentials you cannot afford to lose, and keep
> backups of the vault file.

## Design

- **No network.** No HTTP plugin, no telemetry, no update check. The webview's
  Content-Security-Policy is `default-src 'none'` with `connect-src` limited to
  Tauri's local IPC, and the webview is only granted Burrow's own commands (no
  file-system, shell, HTTP or dialog permissions).
- **One encrypted file**, stored wherever you choose, so you can sync it with
  Syncthing or a cloud folder. Burrow does not sync anything itself.
- **Secrets stay in Rust.** Key derivation, encryption and decryption happen in
  the `vault-core` crate. The front-end never receives the key or the
  decrypted vault: the entry list has no passwords, and a password is sent to
  the UI only when you click *Show* (it is hidden again after 15 s). Copying
  happens in Rust.
- **No home-made crypto.** Argon2id (`argon2`), XChaCha20-Poly1305
  (`chacha20poly1305`), `rand` with `OsRng`, and `zeroize`.

### Vault file format (version 1)

```
offset  size  field
0       8     magic "BURROWPW"
8       2     format version (u16 LE) = 1
10      4     Argon2id memory cost, KiB (u32 LE)   default 65536
14      4     Argon2id time cost (u32 LE)          default 3
18      4     Argon2id parallelism (u32 LE)        default 1
22      16    Argon2id salt (random, per vault)
38      24    XChaCha20-Poly1305 nonce (random, new on every save)
62      ...   ciphertext || 16-byte Poly1305 tag
```

The key is `Argon2id(master password, salt)`. The 62-byte header is passed as
AEAD associated data, so tampering with the version, KDF parameters, salt or
nonce makes decryption fail. The plaintext is a JSON document with the
entries (title, username, password, URL, notes, UUID, modification time).
Parameters read from a file are range-checked before key derivation so a
hostile file cannot request absurd amounts of memory.

Saving writes a temporary file next to the vault, `fsync`s it and renames it
over the vault (atomic on a single volume). Before every save Burrow compares
the file on disk with what it last read or wrote (SHA-256); if it changed,
the save is refused and you choose between **reloading** the disk version
(discarding your pending change) or **overwriting** it. There is no merge.

## Threat model

**Burrow tries to protect against:**

- Someone obtaining the vault file (stolen laptop backup, cloud provider,
  sync server): the contents are encrypted and authenticated; the strength of
  the protection is that of your master password plus Argon2id.
- Accidental overwrite of changes made on another device (the external
  modification check).
- Passwords lingering on the clipboard (cleared after 20 s, only if the
  clipboard still holds what Burrow copied) or in an unattended unlocked
  session (auto-lock after inactivity, on window close and on quit).
- Network exfiltration by the app itself (no network code, strict CSP).

**Burrow does not protect against:**

- Malware or a malicious user running as you on the same machine: keyloggers,
  screen capture, clipboard readers, debuggers, memory dumps. While unlocked,
  secrets are in process memory.
- A weak master password. There is no recovery: forget it and the vault is
  lost.
- Traffic analysis of the file: its size reveals roughly how much you store,
  and modification times reveal when you save.
- Rollback: someone able to replace the file with an older valid copy of
  your vault (same password) will not be detected.
- A compromised build or dependency chain. Build from source and review it if
  that matters to you.

## Known limitations

- **Not audited.** See above.
- **Memory hygiene is best effort.** Keys and entries are zeroized when
  dropped, but copies made by the OS (swap, hibernation files, core dumps),
  the webview, the IPC layer and `serde` buffers are outside Burrow's control.
  Passwords you type or reveal exist as JavaScript strings until garbage
  collected.
- **Clipboard.** Clipboard managers and history features may keep what you
  copy. Burrow asks the OS to exclude its content from clipboard history
  where an API exists (Windows, macOS, and some Linux managers), but this is a
  request, not a guarantee. On Linux, clearing relies on Burrow still being
  the clipboard owner; X11 and Wayland behave differently.
- **Auto-lock** is driven by activity inside the Burrow window. It does not
  detect OS-level events such as screen lock or suspend.
- **No merge** of concurrent edits; last explicit decision wins.
- Sync tools may create "conflicted copy" files next to the vault; Burrow does
  not look for them.
- Only version 1 of the file format exists, with no migrations yet.
- Argon2 parameters (64 MiB, 3 passes) are set when the vault is created. Changing the master
  password re-encrypts with a fresh salt and the current defaults, but there is no UI to pick
  other parameters. Old copies of the vault file (backups, other devices) keep the old password.
- No browser extension, sharing, mobile app or built-in sync (out of scope).

## Installation

### Download

Installers for macOS (universal), Windows and Linux (`.deb`, `.rpm`, AppImage) are attached to the
[GitHub releases](../../releases) once a maintainer publishes one. They are **unsigned and unaudited**:

- **macOS**: the app is not notarized, so macOS blocks it the first time. On macOS 15 or later, try to open it
  once, then go to *System Settings, Privacy & Security* and click *Open Anyway*. On macOS 14 or earlier,
  right-click the app, then *Open*.
- **Windows**: SmartScreen may warn about an unknown publisher (*More info*, then *Run anyway*).
- **Linux**: install the `.deb`/`.rpm` with your package manager, or `chmod +x` the AppImage and run it.

Each release also has a `SHA256SUMS.txt` (`shasum -a 256 -c SHA256SUMS.txt`). Prefer building from source if you want
to know exactly what you run.

### Build from source

Requirements: Rust (stable), Node.js 20+, [pnpm](https://pnpm.io), and the
[Tauri 2 system prerequisites](https://tauri.app/start/prerequisites/) for
your platform. On Debian/Ubuntu:

```sh
sudo apt install libwebkit2gtk-4.1-dev libsoup-3.0-dev libgtk-3-dev \
  libayatana-appindicator3-dev librsvg2-dev libxdo-dev libssl-dev build-essential
```

```sh
pnpm install
pnpm tauri dev      # development
pnpm tauri build    # release bundle for your platform
```

## Development

```sh
cargo test -p vault-core   # core: crypto, format, model, generator
cargo clippy --workspace --all-targets
pnpm typecheck && pnpm test
pnpm lint:fsd              # Feature-Sliced Design import rules (steiger)
```

### Layout

```
crates/vault-core/   pure Rust: file format, crypto, model, password generator
src-tauri/           Tauri app: commands, clipboard, auto-lock, window, ACL/CSP
src/                 React front-end, Feature-Sliced Design:
  app → pages → widgets → features → entities → shared   (imports go down only)
  shared/api         the only code that calls the backend (typed wrappers)
```

`vault-core` has no dependency on Tauri or any UI code. The front-end talks to
Rust only through the typed commands in `src/shared/api`. File paths are
chosen with native dialogs opened by the Rust side, so the webview cannot ask
the backend to read or write arbitrary paths.

## License

[MIT](LICENSE) © Hugo THIEBAUT
