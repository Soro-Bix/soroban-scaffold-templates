#!/usr/bin/env bash
#
# Render one template into a directory, substituting template variables.
#
#   scripts/render-template.sh <template> <out-dir> [project-name] [author]
#
# This is the single definition of how a template is rendered. CI and
# validate-template.sh both call it, so the substitution rules cannot drift
# between what CI builds and what validation checks.

set -euo pipefail

TEMPLATE="${1:?usage: render-template.sh <template> <out-dir> [project-name] [author]}"
OUT_DIR="${2:?usage: render-template.sh <template> <out-dir> [project-name] [author]}"
PROJECT_NAME="${3:-test-project}"
AUTHOR="${4:-Test Author}"

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TEMPLATE_DIR="$REPO_ROOT/templates/$TEMPLATE"

if [[ ! -d "$TEMPLATE_DIR" ]]; then
  echo "error: no such template: $TEMPLATE (looked in $TEMPLATE_DIR)" >&2
  exit 1
fi

mkdir -p "$OUT_DIR"

while IFS= read -r file; do
  rel="${file#"$TEMPLATE_DIR"/}"
  rel="${rel%.template}"
  dest="$OUT_DIR/$rel"
  mkdir -p "$(dirname "$dest")"
  sed -e "s/{{PROJECT_NAME}}/$PROJECT_NAME/g" -e "s/{{AUTHOR}}/$AUTHOR/g" "$file" > "$dest"
done < <(find "$TEMPLATE_DIR" -type f)
