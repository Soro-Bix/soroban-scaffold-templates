# Template Authoring Guide

How to add a new contract template to this repository.

- [How templates work](#how-templates-work)
- [Directory structure](#directory-structure)
- [Template variables](#template-variables)
- [The Cargo.lock.template requirement](#the-cargolocktemplate-requirement)
- [Adding a template, step by step](#adding-a-template-step-by-step)
- [Testing requirements](#testing-requirements)
- [Wiring into CI](#wiring-into-ci)
- [Review criteria](#review-criteria)

## How templates work

A template is a directory of files whose names end in `.template`. When a user
runs `sorobix init my-project --template <name>`, the CLI:

1. Copies every file from `templates/<name>/` into the new project directory,
   preserving subdirectories
2. Strips the `.template` suffix from each filename
3. Substitutes `{{VARIABLE}}` placeholders in the file contents

So `templates/nft/src/lib.rs.template` becomes `my-project/src/lib.rs`, with
`{{PROJECT_NAME}}` replaced by `my-project`.

Substitution applies to file *contents* only. Filenames are not templated — a
generated project always has the same file layout as the template it came from.

## Directory structure

Every template must contain exactly these files:

```
templates/<name>/
├── Cargo.toml.template
├── Cargo.lock.template
├── .gitignore.template
├── README.md.template
└── src/
    ├── lib.rs.template
    └── test.rs.template
```

CI validates this list, so a template missing any of them fails the build.

## Template variables

Two variables are available:

| Variable | Replaced with |
|---|---|
| `{{PROJECT_NAME}}` | The name passed to `sorobix init <project-name>` |
| `{{AUTHOR}}` | `--author`, else `git config user.name`, else a placeholder |

Use them in `Cargo.toml.template` and `README.md.template`:

```toml
[package]
name = "{{PROJECT_NAME}}"
authors = ["{{AUTHOR}}"]
```

**Do not invent new variables.** The CLI only substitutes the two above;
anything else survives into the generated project as a literal `{{FOO}}` and
CI will reject it. Adding a variable requires a matching change in the CLI's
`renderTemplate` context — open an issue on
[soroban-scaffold](https://github.com/Soro-Bix/soroban-scaffold) first.

Note that `{{PROJECT_NAME}}` becomes a *crate name*. Rust crate names cannot
contain characters that are invalid in an identifier once normalized, so avoid
using it anywhere that requires a valid Rust identifier directly (a struct name,
for example) — `my-project` is a legal crate name but `my-projectContract` is
not a legal struct name.

## The Cargo.lock.template requirement

Every template ships a committed lockfile. This is not optional and it is not
an oversight.

Without a pinned lockfile, a freshly generated project resolves its dependency
graph from scratch. `soroban-env-host`'s `testutils` feature currently pulls in
`ed25519-dalek` 3.0.0 alongside an older `ed25519-dalek` used elsewhere in the
graph, and the two disagree on which `rand_core` `CryptoRng` implementation
`ChaCha20Rng` satisfies. The result is that `cargo test` fails to compile on a
clean machine — not in this repository, but in the user's brand-new project,
which is a considerably worse place to discover it.

Shipping the lockfile is what makes `sorobix init` produce something that
works on the first `cargo test`.

Two consequences for template authors:

- `.gitignore.template` must **not** list `Cargo.lock`. A scaffolded contract
  is a deployable artifact rather than a library, so committing the lockfile is
  correct practice independently of the conflict above.
- The lockfile's root package entry contains the crate name, so it needs
  `{{PROJECT_NAME}}` substituted like any other template file:

  ```toml
  [[package]]
  name = "{{PROJECT_NAME}}"
  version = "0.1.0"
  ```

In practice you do not need to hand-write this. Copy the existing one:

```bash
cp templates/basic/Cargo.lock.template templates/<name>/Cargo.lock.template
```

All templates currently share an identical dependency set, so the same lockfile
is valid for each. If your template adds a dependency the others do not have,
generate a fresh lockfile instead — see step 4 below.

## Adding a template, step by step

### 1. Create the directory

```bash
mkdir -p templates/<name>/src
```

### 2. Start from an existing template

```bash
cp templates/basic/Cargo.toml.template   templates/<name>/
cp templates/basic/Cargo.lock.template   templates/<name>/
cp templates/basic/.gitignore.template   templates/<name>/
```

Edit the `description` field in `Cargo.toml.template` to describe your contract.
Leave everything else — the release profile is tuned for WASM output and should
not be changed per-template.

### 3. Write the contract and its tests

`src/lib.rs.template` and `src/test.rs.template`. See
[Testing requirements](#testing-requirements) below.

**Develop against a real compiler, not from memory.** The Soroban SDK's API has
changed across 22.x releases, and code that looks correct frequently is not.
Render your template to a scratch directory and iterate there:

```bash
./scripts/render-template.sh <name> /tmp/dev-<name>
cd /tmp/dev-<name>
cargo test --locked
```

This repository has already shipped a template whose tests never compiled
against any real SDK release, because they were written by inspection rather
than run. Do not repeat that.

### 4. If your template needs different dependencies

Generate a lockfile from your actual `Cargo.toml`, then templatize the crate
name:

```bash
cd /tmp/dev-<name>
cargo generate-lockfile
cargo update -p ed25519-dalek@3.0.0 --precise 2.2.0   # the conflict above
cargo test --locked                                    # confirm it works
sed 's/^name = "dev-<name>"$/name = "{{PROJECT_NAME}}"/' Cargo.lock \
  > "$OLDPWD/templates/<name>/Cargo.lock.template"
```

Verify only the root package name was changed — dependency entries must keep
their real names.

### 5. Write `README.md.template`

It becomes the generated project's README, so write it for someone who has just
run `sorobix init`. Document the contract's functions, its errors, and how to
build, test and deploy. Use `{{PROJECT_NAME}}` in paths and `{{AUTHOR}}` in the
credit line.

### 6. Validate before opening a PR

```bash
./scripts/validate-template.sh <name>
```

This checks that every required file exists, that no `{{VARIABLE}}` survived
substitution, that the project name and author reached the right files, and
that nothing kept a `.template` suffix. Then confirm it actually builds:

```bash
./scripts/render-template.sh <name> /tmp/check-<name>
cd /tmp/check-<name>
cargo build --locked
cargo build --locked --target wasm32-unknown-unknown
cargo test --locked
```

All three must pass. CI runs exactly these commands.

## Testing requirements

`src/test.rs.template` must contain **at least 7 tests**, and they must cover:

- **The happy path** for every public function
- **Every error variant** your error enum declares. An error nobody can trigger
  is either dead code or a missing test
- **Authorization failures** — for every function guarded by `require_auth()` or
  an admin check, a test proving an unauthorized caller is rejected
- **State after failure** — a rejected operation must leave state unchanged.
  Assert that, rather than only asserting the error

Use `try_<function>()` to assert on errors:

```rust
let result = client.try_mint(&not_admin, &user, &1000);
assert_eq!(result, Err(Ok(TokenError::Unauthorized)));
```

Tests run under `env.mock_all_auths()`, which satisfies `require_auth()` calls.
That means `require_auth()` alone is *not* what your unauthorized-caller tests
exercise — they exercise your explicit identity checks. Both matter: keep the
`require_auth()` for production, and the explicit check for a testable,
well-typed error.

If your contract moves tokens, deploy a real Stellar Asset Contract rather than
mocking one:

```rust
let sac = env.register_stellar_asset_contract_v2(admin.clone());
let token_client = token::Client::new(&env, &sac.address());
```

The `escrow` template does this and is worth reading before writing your own.

## Wiring into CI

Add your template to the matrix in `.github/workflows/ci.yml`:

```yaml
    strategy:
      fail-fast: false
      matrix:
        template: [basic, token, escrow, nft, <name>]
```

Each leg renders the template, validates the output, then runs a native build,
a `wasm32-unknown-unknown` build, and the test suite. There is nothing else to
configure.

Also add your template to the CLI's list so users can actually select it —
`VALID_TEMPLATES` and `TEMPLATE_DESCRIPTIONS` in
[`src/utils/templates.ts`](https://github.com/Soro-Bix/soroban-scaffold/blob/main/src/utils/templates.ts).
A template that exists here but is missing there cannot be scaffolded.

## Review criteria

A template PR is merged when:

1. **All CI legs pass**, including the new one
2. **At least 7 tests**, covering the cases in
   [Testing requirements](#testing-requirements)
3. **`require_auth()` on every state-changing function**, plus explicit identity
   checks returning typed errors
4. **A declared error enum** — no `panic!`, `unwrap()` on user input, or
   `unimplemented!()` in a shipped template
5. **Events published on state changes**, so indexers can follow the contract
6. **`README.md.template` documents every public function and error**
7. **`Cargo.lock.template` present** and proven to work via `--locked`
8. **The contract does something real.** Stub templates that `unimplemented!()`
   are not accepted; this repository previously carried two and they were
   replaced precisely because a scaffold that does not compile into anything
   useful wastes the user's time

Templates are read by people learning Soroban. Clear, conventional code that
demonstrates good practice is worth more here than clever code.

## Related

- [Sorobix CLI](https://github.com/Soro-Bix/soroban-scaffold) — consumes these templates
- [CONTRIBUTING.md](../CONTRIBUTING.md) — branch naming and commit conventions
- [Stellar Soroban documentation](https://developers.stellar.org/docs/smart-contracts)
