# Template Authoring Guide

This guide explains how to add a new Soroban contract template to this repository.

## Directory layout

Each template lives under `templates/<name>/` and follows a fixed shape:

```
templates/<name>/
  Cargo.toml.template
  Cargo.lock.template
  README.md.template
  .gitignore.template
  src/
    lib.rs.template
    test.rs.template
```

The `.template` suffix marks files that the [Sorobix CLI](https://github.com/Soro-Bix/soroban-scaffold)
renders at generation time. Two placeholders are substituted before the project is scaffolded:

- `{{PROJECT_NAME}}` — the user-supplied crate name
- `{{AUTHOR}}` — the user-supplied author

## Steps to add a template

1. Create `templates/<name>/` with the files above. Copy `Cargo.toml.template`,
   `.gitignore.template` and `Cargo.lock.template` from `templates/basic/` and adjust
   the `description` line.
2. Write the contract in `src/lib.rs.template`. Use `#![no_std]`, the
   `#[contract]` / `#[contractimpl]` / `#[contracttype]` / `#[contracterror]`
   attributes, and keep storage under typed `DataKey` variants.
3. Write unit tests in `src/test.rs.template` using `env.register(...)` and the
   auto-generated `<Contract>Client`. Use `env.mock_all_auths()` for auth-heavy paths.
4. Add the template name to the `matrix.template` list in `.github/workflows/ci.yml`
   so it is compiled and tested on every push.
5. Run `cargo test` inside the template locally before opening a PR.

## Conventions

- Storage: instance storage for admin/config, persistent storage for per-user state.
- Errors: a single `#[contracterror]` enum with `#[repr(u32)]` variants.
- Tests must not require network access — mock addresses and auth.
