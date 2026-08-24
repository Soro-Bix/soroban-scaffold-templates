# Contributing

Thanks for considering a contribution to the Sorobix contract templates.

## What lives here

This repository holds the Soroban contract templates that the
[Sorobix CLI](https://github.com/Soro-Bix/soroban-scaffold) generates projects
from. Each template is a real, compiling, tested Soroban contract — not
pseudo-code.

If you want to change the CLI itself (flags, prompts, commands), that belongs in
the [soroban-scaffold](https://github.com/Soro-Bix/soroban-scaffold) repository
instead.

## Adding or changing a template

Read the [Template Authoring Guide](./docs/TEMPLATE_AUTHORING.md) first. It
covers the required directory structure, the template variables, why every
template ships a `Cargo.lock.template`, the testing requirements, and the review
criteria a template PR is judged against.

## Prerequisites

- Rust 1.76 or newer
- The `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- Bash (for the render and validation scripts)

## Working locally

Templates are not directly buildable — they contain `{{PROJECT_NAME}}`
placeholders and `.template` filename suffixes. Render one first:

```bash
./scripts/render-template.sh basic /tmp/dev-basic
cd /tmp/dev-basic
cargo test --locked
```

Validate that a template renders into a correct project:

```bash
./scripts/validate-template.sh basic
```

This checks for missing files, unsubstituted `{{VARIABLE}}` placeholders, a
wrong package name, a missing author, and stray `.template` suffixes. CI runs
the same script against the exact tree it is about to compile.

## Before opening a pull request

Run the full check that CI will run, for every template you touched:

```bash
./scripts/validate-template.sh <template>
./scripts/render-template.sh <template> /tmp/check
cd /tmp/check
cargo build --locked
cargo build --locked --target wasm32-unknown-unknown
cargo test --locked
```

All must pass. `--locked` matters: without it cargo silently re-resolves the
dependency graph and reintroduces the `ed25519-dalek` conflict the committed
lockfile exists to prevent.

## Branch naming

| Prefix | For |
|---|---|
| `feat/` | A new template or new contract functionality |
| `fix/` | Correcting a bug in an existing template |
| `docs/` | Documentation only |
| `test/` | Test coverage or CI changes |

For example: `feat/multisig-template`, `fix/escrow-dispute-auth`.

## Commit messages

Conventional commits, with a body explaining *why* where it is not obvious:

```
feat: add multisig contract template with approval tests

M-of-N approval with a configurable threshold. Approvals are tracked
per-signer rather than as a count, so the same signer approving twice
cannot push a proposal over the threshold on its own.
```

State what you actually verified. "13 tests passing, verified on wasm32" is
useful to a reviewer; "should work" is not.

## Reporting problems

Open an issue describing what you ran, what you expected, and what happened,
including the real output. If a generated project fails to compile, the
contents of its `Cargo.lock` and the output of `cargo test --locked` are the
most useful things to include.

## Drips Wave

This project participates in the
[Drips Wave Stellar contributor program](https://drips.network/wave/stellar).
Issues labelled `Stellar Wave` are open for contribution — several are new
template types (multisig, timelock, vesting) that are well-scoped for anyone
comfortable with Rust and Soroban.
