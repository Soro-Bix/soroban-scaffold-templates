# Soroban Scaffold Templates — Project Context

## Session 1 — 2026-08-04

### What was built
- basic/ template: counter contract with tests
- token/ template: stub (to be populated)
- escrow/ template: stub (to be populated)

### Template variables
Templates use {{PROJECT_NAME}} and {{AUTHOR}} as placeholder variables that the CLI replaces at generation time

### Next session
- Populate token/ and escrow/ templates
- Wire CLI to actually copy and render these templates

## Session 2 — 2026-08-05

### What was validated
- `basic/` template validated end-to-end by the soroban-scaffold CLI: `sorokit init` now copies every file in `templates/basic/`, strips the `.template` suffix, and renders `{{PROJECT_NAME}}`/`{{AUTHOR}}` via Handlebars into `Cargo.toml`, `.gitignore`, `README.md`, `src/lib.rs`, `src/test.rs`
- Confirmed generated `Cargo.toml` and `README.md` render correctly with real project name and author values

### Next session
- Populate token/ and escrow/ templates
- Make the CLI's template path configurable instead of a hardcoded sibling-repo path

## Session 3 — 2026-08-05

### What was built
- Added `.github/workflows/ci.yml`: a 3-way matrix (`basic`, `token`, `escrow`) that renders each template with dummy `PROJECT_NAME`/`AUTHOR` values via `sed`, then runs `cargo build`, `cargo build --target wasm32-unknown-unknown`, and `cargo test` against the real generated project — the actual Rust deliverable, not just file-copy mechanics
- CI run green on all 3 legs: https://github.com/Soro-Bix/soroban-scaffold-templates/actions/runs/30999360048

### Bugs the CI validation surfaced and fixed
- `basic/test.rs.template` never actually compiled against any current `soroban-sdk` 22.x release: `env.register(&Contract, &address)` doesn't match the real API (second arg is constructor args, and `register` itself returns the assigned `Address` — there's no way to bind at a caller-chosen address), and `events[0].data` doesn't match the real return type (`Vec<(Address, Vec<Val>, Val)>`, not a struct with a `.data` field). Session 2's "validated end-to-end" claim only checked file copy + variable substitution, never `cargo test`. Rewrote `create_and_register` and the event assertion to match the current API. `token`/`escrow` stub tests were unaffected (they never call `register`/`events`).
- Renamed `[dev_dependencies]` → `[dev-dependencies]` (deprecated key, would break under the 2024 edition) in all three `Cargo.toml.template` files.
- Dropped an unused `Symbol` import in `basic/lib.rs.template`.

### Known issue, not yet fixed
- `soroban-env-host`'s `testutils` feature currently pulls in `ed25519-dalek` 3.0.0 alongside an older `ed25519-dalek` elsewhere in the dependency graph; the two disagree on which `rand_core` `CryptoRng` impl `ChaCha20Rng` satisfies, breaking `cargo test` on a fresh dependency resolve — independent of which `soroban-sdk` 22.x patch is pinned. CI works around it with `cargo update -p ed25519-dalek@3.0.0 --precise 2.2.0 || true`.
- **This same conflict affects real end users today**: `sorokit init` scaffolds `Cargo.toml` with an unpinned `soroban-sdk = "22.0.0"` and no committed `Cargo.lock`, so a freshly generated project's first `cargo test` will very likely hit the identical failure outside of CI. Needs a real fix — e.g. the CLI committing a working `Cargo.lock` as part of scaffolding, or the template pinning `ed25519-dalek` directly — tracked as a follow-up, not yet done.

### Next session
- Populate token/ and escrow/ templates
- Make the CLI's template path configurable instead of a hardcoded sibling-repo path
- Fix the ed25519-dalek/rand_core conflict for real generated projects (see above), not just CI
