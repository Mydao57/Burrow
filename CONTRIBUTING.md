# Contributing to Burrow

Thanks for helping. Burrow is a password manager, so the bar for changes is
higher than usual: small, reviewable, tested, and boring wherever it touches
secrets.

## Ground rules

- **No network code, ever.** No HTTP clients, telemetry, update checks, remote
  fonts or CDN assets. The CSP and the Tauri ACL must stay as strict as they are.
- **No home-made cryptography.** Use the existing crates (`argon2`,
  `chacha20poly1305`, `zeroize`, `rand`/`OsRng`). Anything that changes the file
  format, key derivation or nonce handling needs a design discussion in an issue
  first, a format version bump if it is not backward compatible, and tests.
- **Secrets stay in Rust.** The front-end must never receive the key or the full
  decrypted vault. A password may reach the UI only when it is explicitly
  revealed or typed in by the user. Copying happens in Rust.
- **New Tauri commands** must be added in three places: `src-tauri/src/commands.rs`
  and `main.rs`, the `COMMANDS` list in `src-tauri/build.rs`, and the permission
  list in `src-tauri/capabilities/default.json`. Do not widen capabilities
  (fs, shell, http, dialog for the webview) to make something easier.
- **New dependencies** need a justification in the PR (what, why, maintenance
  and license). Prefer fewer, well-known crates/packages. Lockfiles
  (`Cargo.lock`, `pnpm-lock.yaml`) are committed and must be updated with the
  change.
- **Front-end architecture is Feature-Sliced Design**: layers
  `app → pages → widgets → features → entities → shared`, imports go downward
  only, and the front-end calls Rust only through `src/shared/api`.
  `pnpm lint:fsd` enforces it.
- `vault-core` must stay independent of Tauri and of any UI code.

## Development setup

See the *Installation* section of the [README](README.md) for system
prerequisites. Then:

```sh
pnpm install
pnpm tauri dev
```

## Checks to run before opening a PR

These are the same checks CI runs; all must pass.

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p vault-core
pnpm typecheck
pnpm test
pnpm lint:fsd
pnpm build
```

Tests are expected for behaviour changes, in particular for anything in
`vault-core` (format, crypto, save/open, external-change detection). Bug fixes
should come with a test that fails without the fix.

## Workflow

1. Open an issue for anything non-trivial (new feature, format change,
   dependency) before writing code.
2. Fork or branch from `release`, using a descriptive name
   (`fix/clipboard-clear`, `feat/change-master-password`).
3. Keep PRs small and focused; one concern per PR. Do not mix refactors with
   behaviour changes.
4. Write clear commit messages (imperative mood, explain the *why*).
5. Open a PR against `release` and fill in the template.
6. Address review comments with new commits; the PR is merged by a maintainer.

Do not commit vault files (`*.burrow`), real credentials, or personal data in
tests, fixtures or screenshots. Use obviously fake values.

## Branch policy: `release`

`release` is the only long-lived branch. It is what gets built and shipped, so
it is protected. The rules below are encoded in
[`.github/rulesets/release.json`](.github/rulesets/release.json) and must be
enabled in the repository settings (GitHub does not apply files from the repo
automatically).

**Enforced by the ruleset**

- No direct pushes: every change lands through a pull request.
- No force-push and no deletion of `release`; history is linear (squash or
  rebase merge, no merge commits).
- Required status checks, with the branch up to date before merging:
  `core`, `frontend`, `backend`.
- Review required, stale approvals dismissed on new pushes, the latest push
  approved by someone other than its author, all review conversations
  resolved, and code-owner review for the paths listed in
  [`CODEOWNERS`](.github/CODEOWNERS).
- Rules apply to administrators too (no bypass actors).

**Recommended additional repository settings**

- *Require signed commits* on `release` (add `required_signatures` to the
  ruleset once every maintainer has commit signing set up).
- Require two-factor authentication for all collaborators.
- Restrict who can push tags and create releases; build releases only from
  `release` and from CI.
- Enable Dependabot alerts and security updates, secret scanning with push
  protection, and private vulnerability reporting.
- Keep GitHub Actions workflow permissions at *read-only* by default
  (workflows here already declare `contents: read`), and require approval for
  workflows from first-time contributors.

**Solo-maintainer note.** With a single maintainer, "one approval from someone
other than the last pusher" would make it impossible to merge your own PRs.
The shipped ruleset therefore requires a PR and passing checks but sets the
required approvals to `0`. As soon as there is a second maintainer, raise
`required_approving_review_count` to `1` and set `require_code_owner_review`
and `require_last_push_approval` to `true` in the ruleset.

## Reporting security issues

Do **not** open a public issue for a vulnerability. Use GitHub's private
vulnerability reporting ("Security" tab → "Report a vulnerability") or contact
the maintainer directly. Please include the version, platform, impact and steps
to reproduce. Burrow is unaudited software; reports are welcome but response
times are best-effort.

## License

By contributing you agree that your contributions are licensed under the
[MIT License](LICENSE).
