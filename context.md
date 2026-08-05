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
