# Soroban Scaffold Templates

This repository contains the contract templates used by the [Soroban Scaffold CLI](https://github.com/Soro-Bix/soroban-scaffold).

When you run `sorokit init my-project`, the CLI copies and renders one of these templates into your new project directory.

## Available Templates

| Template   | Path              | Description                                                    | Status       |
|------------|-------------------|----------------------------------------------------------------|--------------|
| `basic`    | `templates/basic/` | Minimal counter contract — great for learning Soroban basics   | ✅ Complete   |
| `token`    | `templates/token/` | Standard token contract implementing the Soroban token interface | 🚧 Stub only |
| `escrow`   | `templates/escrow/` | Milestone-based escrow (from Milestone Escrow patterns)        | 🚧 Stub only |

## How Templates Work

Each template is a directory containing files with a `.template` extension.
At generation time, the CLI:

1. Copies every `*.template` file into the new project
2. Strips the `.template` suffix
3. Substitutes template variables using Handlebars-style `{{VAR}}` syntax

### Template Variables

| Variable          | Example                  | Source                                   |
|-------------------|--------------------------|------------------------------------------|
| `{{PROJECT_NAME}}` | `my-counter`             | `sorokit init <project-name>`            |
| `{{AUTHOR}}`       | `Jane Doe <j@example.com>` | Prompted during init, or from git config |

## Contributing a Template

1. Fork this repository
2. Create `templates/<your-template-name>/`
3. Add `Cargo.toml.template`, `src/lib.rs.template`, `src/test.rs.template`, `.gitignore.template`, `README.md.template`
4. Use `{{PROJECT_NAME}}` and `{{AUTHOR}}` as placeholders
5. Add an entry to the table above
6. Open a PR with a conventional commit: `feat: add <name> template`

## License

MIT — part of the Soro-Bix ecosystem
