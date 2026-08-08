# Template Authoring Guide

## Overview

This repository contains Soroban smart contract templates used by the Sorobix CLI (`soroban-scaffold`).
Each template is a self-contained directory under `templates/` with the files needed to scaffold a new project.

## Template Structure

```
templates/<name>/
  .gitignore.template       # Git ignore rules
  Cargo.toml.template       # Rust package manifest
  Cargo.lock.template       # Locked dependency versions
  README.md.template        # Project README
  src/
    lib.rs.template         # Contract implementation
    test.rs.template        # Unit tests
```

## Placeholder Variables

Templates use Handlebars-style placeholders that are replaced at scaffold time:

| Placeholder | Description |
|-------------|-------------|
| `{{PROJECT_NAME}}` | The user-provided project name |
| `{{AUTHOR}}` | The author name (from --author flag or git config) |

## Adding a New Template

1. **Create the directory**: `templates/<name>/` with the standard structure above.
2. **Implement the contract** in `src/lib.rs.template`:
   - Must use `#![no_std]`
   - Must import from `soroban_sdk`
   - Must define a contract struct with `#[contract]` and `#[contractimpl]`
   - Use the `soroban-sdk = "22.0.0"` dependency
3. **Write tests** in `src/test.rs.template`:
   - Use `#[cfg(test)]` and `#[test]` attributes
   - Use `soroban_sdk::testutils` for test addresses
4. **Update the CLI**: Add the template name to `VALID_TEMPLATES` in `soroban-scaffold/src/commands/init.ts`.
5. **Run validation**: `bash scripts/validate.sh`

## Design Guidelines

- **Separation of concerns**: Each template should implement ONE contract type
- **Error handling**: Use `#[contracterror]` enums with descriptive variants
- **Events**: Emit events with `env.events().publish()` for important state changes
- **Storage**: Use `env.storage().instance()` for singleton data, `persistent()` for collections
- **Tests**: Cover initialization, happy path, and error cases
- **No external deps**: Templates should only depend on `soroban-sdk`

## Template Catalog

| Template | Description | Stellar Wave Issue |
|----------|-------------|-------------------|
| basic | Minimal contract skeleton | — |
| token | SEP-41 fungible token | #1 |
| escrow | Time-locked escrow | — |
| nft | Non-fungible token (ERC-721 style) | #2 |
| multisig | Multi-signature wallet | #3 |
| timelock | Time-based token lock | #4 |
| vesting | Token vesting with cliff | #7 |

## Testing

Run the validation script to check all templates:

```bash
bash scripts/validate.sh
```

This checks that each template has all required files and meets structural requirements.
