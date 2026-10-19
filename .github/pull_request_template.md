## What and why

<!-- What does this change, and why? Link the issue. -->

## Checklist

- [ ] `cargo fmt`, `cargo clippy -D warnings`, `cargo test -p vault-core` pass
- [ ] `pnpm typecheck`, `pnpm test`, `pnpm lint:fsd`, `pnpm build` pass
- [ ] Tests added or updated for the behaviour change
- [ ] No network code, no new webview permissions
- [ ] Secrets stay in Rust (no key or full vault sent to the front-end)
- [ ] New dependencies are justified below (or: none)
- [ ] File format change? Version bumped and migration/tests added (or: none)
- [ ] No vault files, real credentials or personal data committed

## Notes for reviewers

<!-- Security considerations, trade-offs, what is not tested. -->
