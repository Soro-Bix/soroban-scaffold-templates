# Soroban Scaffold Templates

> Contract templates used by the [Sorobix CLI](https://github.com/Soro-Bix/soroban-scaffold).

This repository contains the Soroban smart contract templates that `sorobix init` generates from. Each template is a real, working, tested Soroban contract — not pseudo-code or stubs.

## Templates

### basic — Counter Contract
A minimal Soroban contract implementing a simple counter with increment, decrement, and get functions. Ideal starting point for developers new to Soroban.
- **Tests**: 7 passing
- **Functions**: initialize(admin), increment(), decrement(), get_count(), reset(admin)
- **Events**: emitted on every state change (increment, decrement, reset)

### token — SEP-41 Fungible Token
A fully SEP-41 compliant fungible token contract compatible with Stellar wallets, DEXes, and other Soroban contracts.
- **Tests**: 9 passing
- **Functions**: initialize(), mint(), transfer(), transfer_from(), approve(), allowance(), balance(), total_supply(), burn()
- **Events**: Transfer, Approval, Mint, Burn

### escrow — Milestone-Based Escrow
A trustless escrow contract supporting milestone-based fund release and dispute resolution — the same pattern powering [Milestone Escrow](https://github.com/Goldii-locks/escrow-contract).
- **Tests**: 7 passing
- **Functions**: initialize(), fund(), mark_delivered(), approve_milestone(), raise_dispute(), resolve_dispute(), get_job()
- **Events**: emitted on every state transition

### nft — Non-Fungible Token
An NFT contract with per-token ownership, metadata URIs, and single-token approvals for delegated transfers.
- **Tests**: 13 passing
- **Functions**: initialize(), mint(), transfer(), approve(), transfer_from(), burn(), owner_of(), token_uri(), get_approved(), balance_of(), total_supply(), name(), symbol()
- **Events**: Mint, Transfer, Approve, Burn

## CI

Every template is compiled, wasm32-built, and tested on every push via a 4-way CI matrix:

| Template | Build | WASM | Tests |
|---|---|---|---|
| basic | ✅ | ✅ | 7/7 |
| token | ✅ | ✅ | 9/9 |
| escrow | ✅ | ✅ | 7/7 |
| nft | ✅ | ✅ | 13/13 |

## Adding a New Template

See the [Template Authoring Guide](./docs/TEMPLATE_AUTHORING.md) for a full walkthrough: required structure, template variables, why every template ships a `Cargo.lock.template`, testing requirements, and the review criteria a template PR is judged against.

Quick summary:
1. Create a new directory under `templates/` with the template name
2. Add `Cargo.toml.template`, `src/lib.rs.template`, `src/test.rs.template`, `README.md.template`
3. Copy `Cargo.lock.template` from `templates/basic/` (required for dependency pinning)
4. Add your template to the CI matrix in `.github/workflows/ci.yml`
5. Open a PR — all CI legs must pass

## Template Variables

Templates support these variables via Handlebars substitution:

| Variable | Description |
|---|---|
| `{{PROJECT_NAME}}` | The project name provided to `sorobix init` |
| `{{AUTHOR}}` | The author name from `--author` flag or git config |

## Contributing

This project participates in the [Drips Wave Stellar contributor program](https://drips.network/wave/stellar). Open issues include new template types (multisig, timelock, vesting) and CI improvements — great opportunities for Rust/Soroban developers.

See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines, and the [Sorobix roadmap](https://github.com/Soro-Bix/soroban-scaffold/blob/main/ROADMAP.md) for what is planned across both repositories through November 2026.

## License

MIT
