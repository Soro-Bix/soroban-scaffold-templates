#!/bin/bash
# Template validation tests for soroban-scaffold-templates
set -euo pipefail

TEMPLATES_DIR="templates"
FAILED=0
PASSED=0

echo "=== Soroban Scaffold Templates Validation ==="

for template_dir in "$TEMPLATES_DIR"/*/; do
    name=$(basename "$template_dir")
    echo ""
    echo "--- Validating $name template ---"

    # Check required files
    for required_file in ".gitignore.template" "Cargo.toml.template" "README.md.template" "src/lib.rs.template" "src/test.rs.template"; do
        if [ -f "$template_dir/$required_file" ]; then
            echo "  PASS: $required_file exists"
            PASSED=$((PASSED + 1))
        else
            echo "  FAIL: $required_file missing"
            FAILED=$((FAILED + 1))
        fi
    done

    # Check Cargo.toml has required fields
    cargo_toml="$template_dir/Cargo.toml.template"
    if [ -f "$cargo_toml" ]; then
        for field in "name" "version" "soroban-sdk"; do
            if grep -q "$field" "$cargo_toml"; then
                echo "  PASS: Cargo.toml contains '$field'"
                PASSED=$((PASSED + 1))
            else
                echo "  FAIL: Cargo.toml missing '$field'"
                FAILED=$((FAILED + 1))
            fi
        done
    fi

    # Check lib.rs compiles with soroban-sdk
    lib_rs="$template_dir/src/lib.rs.template"
    if [ -f "$lib_rs" ]; then
        for required_pattern in "#!\[no_std\]" "soroban_sdk" "contract" "contractimpl"; do
            if grep -q "$required_pattern" "$lib_rs"; then
                echo "  PASS: lib.rs contains '$required_pattern'"
                PASSED=$((PASSED + 1))
            else
                echo "  FAIL: lib.rs missing '$required_pattern'"
                FAILED=$((FAILED + 1))
            fi
        done
    fi
done

echo ""
echo "=== Results: $PASSED passed, $FAILED failed ==="
exit $FAILED
