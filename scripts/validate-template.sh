#!/usr/bin/env bash
#
# Validate that a template renders into a correct project.
#
#   scripts/validate-template.sh <template> [rendered-dir]
#
# Without a rendered-dir the template is rendered into a temp directory first.
# Passing one lets CI validate the exact tree it is about to compile, rather
# than a second render that might differ.
#
# Checks performed:
#   1. every expected file is present
#   2. no {{VARIABLE}} placeholder survived rendering
#   3. Cargo.toml's package name is the requested project name
#   4. the author appears in Cargo.toml and README.md
#   5. no template file was copied through with its .template suffix intact

set -euo pipefail

TEMPLATE="${1:?usage: validate-template.sh <template> [rendered-dir]}"
RENDERED_DIR="${2:-}"

PROJECT_NAME="test-project"
AUTHOR="Test Author"

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CLEANUP_DIR=""

if [[ -z "$RENDERED_DIR" ]]; then
  RENDERED_DIR="$(mktemp -d)"
  CLEANUP_DIR="$RENDERED_DIR"
  "$REPO_ROOT/scripts/render-template.sh" "$TEMPLATE" "$RENDERED_DIR" "$PROJECT_NAME" "$AUTHOR"
fi

# Must not end on a failed test: this runs as an EXIT trap, so its return
# value becomes the script's exit code when the script would otherwise
# succeed. `[[ -n "$x" ]] && rm ...` returns 1 whenever CLEANUP_DIR is empty.
cleanup() {
  if [[ -n "$CLEANUP_DIR" ]]; then
    rm -rf "$CLEANUP_DIR"
  fi
}
trap cleanup EXIT

failures=0

fail() {
  echo "  FAIL: $*" >&2
  failures=$((failures + 1))
}

pass() {
  echo "  ok: $*"
}

echo "Validating template '$TEMPLATE' rendered at $RENDERED_DIR"

# 1. Expected files
for required in Cargo.toml Cargo.lock README.md .gitignore src/lib.rs src/test.rs; do
  if [[ -f "$RENDERED_DIR/$required" ]]; then
    pass "$required present"
  else
    fail "$required is missing"
  fi
done

# 2. No unsubstituted placeholders. This is the check that would have caught a
# template referencing a variable the CLI does not actually provide.
if leftovers="$(grep -rIln '{{[A-Za-z_][A-Za-z0-9_]*}}' "$RENDERED_DIR" 2>/dev/null)"; then
  fail "unsubstituted template variables found in:"
  while IFS= read -r file; do
    echo "        ${file#"$RENDERED_DIR"/}: $(grep -oh '{{[A-Za-z_][A-Za-z0-9_]*}}' "$file" | sort -u | tr '\n' ' ')" >&2
  done <<< "$leftovers"
else
  pass "no unsubstituted template variables"
fi

# 3. Package name
if [[ -f "$RENDERED_DIR/Cargo.toml" ]]; then
  if grep -qE "^name = \"$PROJECT_NAME\"$" "$RENDERED_DIR/Cargo.toml"; then
    pass "Cargo.toml package name is \"$PROJECT_NAME\""
  else
    fail "Cargo.toml package name is not \"$PROJECT_NAME\" (got: $(grep -m1 '^name' "$RENDERED_DIR/Cargo.toml" || echo 'no name field'))"
  fi
fi

# 4. Author propagation
if [[ -f "$RENDERED_DIR/Cargo.toml" ]]; then
  if grep -q "$AUTHOR" "$RENDERED_DIR/Cargo.toml"; then
    pass "Cargo.toml credits \"$AUTHOR\""
  else
    fail "Cargo.toml does not contain the author \"$AUTHOR\""
  fi
fi

if [[ -f "$RENDERED_DIR/README.md" ]]; then
  if grep -q "$AUTHOR" "$RENDERED_DIR/README.md"; then
    pass "README.md credits \"$AUTHOR\""
  else
    fail "README.md does not contain the author \"$AUTHOR\""
  fi
fi

# 5. No .template suffix should survive into a generated project.
if stragglers="$(find "$RENDERED_DIR" -name '*.template' -print -quit)" && [[ -n "$stragglers" ]]; then
  fail "files still carrying a .template suffix: $stragglers"
else
  pass "no .template suffixes remain"
fi

if (( failures > 0 )); then
  echo "Template '$TEMPLATE' failed validation with $failures problem(s)." >&2
  exit 1
fi

echo "Template '$TEMPLATE' passed validation."
