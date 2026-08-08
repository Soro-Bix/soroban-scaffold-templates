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
- `basic/` template validated end-to-end by the soroban-scaffold CLI: `sorobix init` now copies every file in `templates/basic/`, strips the `.template` suffix, and renders `{{PROJECT_NAME}}`/`{{AUTHOR}}` via Handlebars into `Cargo.toml`, `.gitignore`, `README.md`, `src/lib.rs`, `src/test.rs`
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
- **This same conflict affects real end users today**: `sorobix init` scaffolds `Cargo.toml` with an unpinned `soroban-sdk = "22.0.0"` and no committed `Cargo.lock`, so a freshly generated project's first `cargo test` will very likely hit the identical failure outside of CI. Needs a real fix — e.g. the CLI committing a working `Cargo.lock` as part of scaffolding, or the template pinning `ed25519-dalek` directly — tracked as a follow-up, not yet done.

### Next session
- Populate token/ and escrow/ templates
- Make the CLI's template path configurable instead of a hardcoded sibling-repo path
- Fix the ed25519-dalek/rand_core conflict for real generated projects (see above), not just CI

## Session 4 — 2026-08-06

### What was fixed
- Corrected org name `Soro-kiit` → `Soro-Bix` (matches the actual GitHub org) in `README.md` and all three `README.md.template` files
- Fixed the ed25519-dalek/rand_core conflict for real generated projects, not just CI (see Session 3's "known issue"): added a pre-generated, individually verified `Cargo.lock.template` to **all three** templates (`basic`, `token`, `escrow` — not just `basic`, since all three declare the same `testutils` dev-dependency and hit the identical failure), with `{{PROJECT_NAME}}` substituted into the root package's `name` field like any other template variable. `copyTemplate` in the CLI already copies and renders arbitrary `*.template` files generically, so this required zero CLI code changes.
- Removed `Cargo.lock` from all three `.gitignore.template` files — for a deployable smart contract (not a library), committing the lockfile is the idiomatic Rust practice, and it's the actual fix here: without it, generated projects keep re-resolving fresh and can hit the same conflict again after any dependency drift.
- Verified via the real CLI end-to-end: `sorobix init fixed-test-project` → `cargo test` passes 5/5 with zero manual intervention (no `cargo update`, no pinning) — this is the first time the actual literal `sorobix init` → `cargo test` path has been proven to work for a real user.

### Caveat
- Each `Cargo.lock.template` pins the dependency graph as resolved on 2026-08-06 (`soroban-sdk` 22.0.11, `soroban-env-host` 22.1.3, `ed25519-dalek` 2.2.0). This is a snapshot, not a permanently-correct answer — it will need periodic regeneration as `soroban-sdk` releases new versions, same as any committed lockfile.

### Next session
- Populate token/ and escrow/ templates with real contract logic
- Make the CLI's template path configurable instead of a hardcoded sibling-repo path
- Periodically regenerate the three `Cargo.lock.template` files against newer `soroban-sdk` releases

## Session 5 — 2026-08-07

### What was built
- `token/` template: real SEP-41-shaped fungible token contract (`initialize`, `mint`, `transfer`, `transfer_from`, `approve`, `allowance`, `balance`, `total_supply`, `name`, `symbol`, `decimals`, `burn`), persistent storage for balances/allowances, instance storage for metadata/admin, `require_auth()` on every state-changing call, events on transfer/approve/mint/burn, `TokenError` enum (`AlreadyInitialized`, `NotInitialized`, `InsufficientBalance`, `InsufficientAllowance`, `Unauthorized`, `InvalidAmount`) — **9 tests passing**
- `escrow/` template: real milestone escrow contract (`initialize`, `fund`, `mark_delivered`, `approve_milestone`, `raise_dispute`, `resolve_dispute`, `get_job`), backed by real cross-contract token transfers via `soroban_sdk::token::Client` (escrow holds funds at its own contract address between `fund` and release), `require_auth()` on every function, `EscrowError` enum (`AlreadyInitialized`, `NotInitialized`, `AlreadyFunded`, `NotFunded`, `Unauthorized`, `InvalidMilestone`, `InvalidStatus`) — **7 tests passing**, using `env.register_stellar_asset_contract_v2()` to deploy a real SAC test token per test (not a mock)
- Copied the existing, already-verified `Cargo.lock.template` from `basic/` to both `token/` and `escrow/` (same dependency tree — both declare the identical `testutils` dev-dependency)
- All three templates now compile and test cleanly with `--locked`: **basic 5/5, token 9/9, escrow 7/7 — 21 tests total**

### How this was verified (not just written and assumed correct)
- Every function was written, then iterated against the real pinned `soroban-sdk` 22.0.11 compiler in a scratch directory (`cargo test`) until clean — not guessed from memory. Both contracts compiled and passed all tests on the first real attempt once the exact API shapes were confirmed (`token::Client`/`token::StellarAssetClient` generated via `#[contractclient]`, `env.register_stellar_asset_contract_v2(admin) -> StellarAssetContract`, found by reading the actual crate source under `~/.cargo/registry`, not recalled from training data)
- Verified `cargo build --target wasm32-unknown-unknown` (the actual deployable artifact) for both contracts, not just native `cargo test`
- Re-verified the final committed template files by rendering them through the same `sed`-based pipeline CI uses (not just the scratch dev copies) and running `cargo test --locked --manifest-path` against each rendered project

### Known issues
- None. All 21 tests pass with `--locked`, no manual `cargo update`/pinning needed.

### Next session
- Add a `--template` flag to the CLI so users can choose `basic`/`token`/`escrow` at `sorobix init` time (currently hardcoded to `basic`)
- Make the CLI's template path configurable instead of a hardcoded sibling-repo path
- Periodically regenerate all three `Cargo.lock.template` files against newer `soroban-sdk` releases

## Session 6 — 2026-08-08

### What was fixed
- README previously listed `reset()` as a `basic` template function, but it didn't exist in the contract — a real inaccuracy caught before Drips Wave submission, not by inspection but by actually grepping the shipped source
- Added `reset(admin)` to `basic/lib.rs.template`, requiring admin auth and resetting `count` to 0, emitting a `reset` event. Since there was no prior concept of an admin at all in this contract, also added a minimal `initialize(admin)` function to store the admin address — a necessary implication of "admin stored at initialize time," not requested as its own line item but unavoidable to make `reset()` meaningful
- `increment()`/`decrement()`/`get_count()` are unchanged and still require no setup — only `reset()` needs `initialize()` to have been called first
- Added 2 new tests: `test_reset_sets_count_back_to_zero`, `test_unauthorized_reset_fails`. Basic template now has **7 tests** (was 5)
- README corrected: function list now includes `initialize(admin)` and `reset(admin)`, test count updated 5 → 7 in both the template description and the CI matrix table

### Known issues
- `soroban-scaffold/README.md` (the CLI repo, not this one) still lists `basic` as having 5 tests in its Templates table — same class of staleness as the one just fixed here, but out of scope for this session since it wasn't part of the requested fix. Flagged, not fixed.

### Next session
- Add a `--template` flag to the CLI so users can choose `basic`/`token`/`escrow` at `sorobix init` time (currently hardcoded to `basic`)
- Make the CLI's template path configurable instead of a hardcoded sibling-repo path
- Periodically regenerate all three `Cargo.lock.template` files against newer `soroban-sdk` releases
- Update `soroban-scaffold/README.md`'s Templates table to say 7 tests for `basic`, matching this fix
