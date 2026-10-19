# CLAUDE.md

Burrow: a local-only password manager (Tauri 2 + Rust backend, React + TypeScript
front-end). Unaudited, security-sensitive software: keep changes small and
tested. Docs and UI are in English.

## Layout

- `crates/vault-core/`: pure Rust (no Tauri/UI): file format, Argon2id +
  XChaCha20-Poly1305, entry model, atomic save, external-change detection, password generator.
- `src-tauri/`: Tauri app. `commands.rs` (IPC), `state.rs` (unlocked vault, idle
  timer), `clipboard.rs` (copy + clear after 20 s), `config.rs`, `build.rs` (command ACL).
- `src/`: front-end in Feature-Sliced Design.
- `CONTRIBUTING.md` has the human-facing rules; `README.md` has the threat model.

## Commands

```sh
cargo test -p vault-core                              # core tests (fast)
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
pnpm typecheck && pnpm test && pnpm lint:fsd && pnpm build
pnpm tauri build --debug --no-bundle                  # binary with the front-end embedded
```

These match the CI jobs (`core`, `frontend`, `backend`). Run them before pushing.

## Non-negotiable invariants

- No network code (no HTTP crates/plugins, telemetry, CDN/remote fonts). Keep the CSP and
  `capabilities/default.json` minimal; never grant the webview fs/shell/http/dialog.
- No home-made crypto; only `argon2`, `chacha20poly1305`, `zeroize`, `rand` (`OsRng`).
- Secrets stay in Rust. The front-end never gets the key or the decrypted vault.
  Lists/details omit passwords; only `reveal_password` returns one. Copying is done in Rust.
- The webview never supplies file paths: paths come from native dialogs opened in Rust.
- File format changes need a version bump, tests, and a README update. The header
  is AEAD associated data; a fresh nonce is generated on every save.
- `vault-core` must not depend on Tauri or UI code.
- Never commit `*.burrow` files or real credentials; use fake values in tests.

## Adding a Tauri command (all steps required)

1. Implement it in `src-tauri/src/commands.rs`, register it in `main.rs`.
2. Add its name to `COMMANDS` in `src-tauri/build.rs` and `allow-<name-with-dashes>`
   to `src-tauri/capabilities/default.json` (otherwise the call is denied at runtime).
3. Add the typed wrapper in `src/shared/api/vault.ts` and DTOs in `types.ts`
   (hand-written mirror of the Rust structs).

Tauri converts top-level argument names to camelCase (`masterPassword`), but fields
inside objects keep their serde names (`exclude_ambiguous`). Use `#[tauri::command(async)]`
so Argon2 and disk I/O do not run on the main thread.

## Front-end conventions (FSD)

- Layers `app → pages → widgets → features → entities → shared`; imports go down only;
  no imports between slices of the same layer. `pnpm lint:fsd` (steiger) enforces it
  (`insignificant-slice` is deliberately off). Every slice needs segments (`ui/`, `model/`) and an `index.ts`.
- Only `src/shared/api` may call `invoke`/`listen`.
- Features that need another feature's UI take it via props (see `renderPasswordTools`), composed in a widget.
- CSP is `style-src 'self'`: no inline `style` attributes; use CSS files.
- Import with the `@/` alias.
- Styling uses design tokens (CSS variables) defined in `src/app/styles/global.css` for light and dark; never hard-code colours in slices. The theme (light/dark/system) lives in `features/theme-switcher` (`data-theme` on `<html>`, stored in `localStorage`). Icons are bundled JSX in `shared/ui/Icon.tsx`; no external fonts/assets.

## Testing notes

- vault-core tests use `KdfParams::new(8, 1, 1)` for speed; one test covers the defaults.
  The workspace builds dependencies at `opt-level = 3` in dev so Argon2 stays fast.
- A plain `cargo build` binary loads the Vite dev URL; use `pnpm tauri build --debug --no-bundle` to run the real app.
- Headless Linux smoke test: `xvfb-run` with `WEBKIT_DISABLE_COMPOSITING_MODE=1
  WEBKIT_DISABLE_DMABUF_RENDERER=1`; drive with `xdotool`, read the clipboard with
  `xclip -o -t UTF8_STRING -selection clipboard`. Linux system deps are listed in the README.
- Not yet verified: macOS, Windows, Wayland; clipboard-history exclusion outside Linux;
  UI flows for create/edit/delete/export/conflict dialog (core logic is tested).

## Releases

- The version lives in `package.json`, `Cargo.toml` (`[workspace.package]`) and `src-tauri/tauri.conf.json`;
  `pnpm check:version [vX.Y.Z]` checks they match. A `v*` tag triggers `.github/workflows/release.yml`
  (draft release with unsigned installers; see `CONTRIBUTING.md`). Never create or push tags unless asked.
- The draft is published by the manual `Publish release` workflow (`scripts/publish-release.sh`; never publish or run it unless asked).
- Local packaging check: `pnpm tauri build --debug --bundles deb` (or `appimage`).

## Git workflow

- `release` is the only long-lived branch. Work on a feature branch and open a PR against
  `release`; never push to `release` directly.
- Do not push or open PRs unless asked. Wait for CI to be green before merging.
- Commit messages: imperative mood, explain the why.
