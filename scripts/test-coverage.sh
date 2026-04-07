#!/bin/bash
# Unified test coverage script for TypeScript and Rust
# Runs both TypeScript (via Vitest) and Rust (via grcov) coverage

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
COVERAGE_DIR="$PROJECT_ROOT/coverage"

echo "=============================================="
echo "Running Test Coverage for Ark Server Manager"
echo "=============================================="
echo ""

# Clean coverage directories
rm -rf "$COVERAGE_DIR/ts" "$COVERAGE_DIR/rs"
mkdir -p "$COVERAGE_DIR/ts" "$COVERAGE_DIR/rs"

# Track overall success
TS_SUCCESS=true
RS_SUCCESS=true

# -------------------------
# TypeScript Coverage
# -------------------------
echo "[1/2] Running TypeScript coverage..."
echo "----------------------------------------------"

cd "$PROJECT_ROOT"

if pnpm vitest run --coverage 2>&1; then
    echo "TypeScript coverage completed successfully"
else
    echo "TypeScript coverage completed with failures"
    TS_SUCCESS=false
fi

echo ""

# -------------------------
# Rust Coverage
# -------------------------
echo "[2/2] Running Rust coverage..."
echo "----------------------------------------------"

cd "$PROJECT_ROOT/src-tauri"

# Set up coverage environment
export CARGO_TARGET_DIR="/tmp/cargo-coverage-grcov"
export RUSTFLAGS="-C instrument-coverage"
export LLVM_PROFILE_FILE="/tmp/cargo-coverage-grcov/coverage-%p-%m.profraw"

# Clean and run tests with coverage
rm -rf "$CARGO_TARGET_DIR"

if cargo test --lib 2>&1; then
    echo "Rust tests passed"

    # Generate coverage report using grcov
    grcov "$CARGO_TARGET_DIR" \
        --binary-path "$CARGO_TARGET_DIR/debug/deps/ark_server_manager_lib-53ecb3a99edf9ec7" \
        --llvm-path "$HOME/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin" \
        -t html,lcov \
        -o "$COVERAGE_DIR/rs" \
        -s . 2>&1 || {
        echo "Warning: grcov report generation had issues"
    }

    # Move HTML report to correct location
    if [ -d "$COVERAGE_DIR/rs/html" ]; then
        mv "$COVERAGE_DIR/rs/html/index.html" "$COVERAGE_DIR/rs/index.html"
        rm -rf "$COVERAGE_DIR/rs/html"
    fi

    echo "Rust coverage report generated at $COVERAGE_DIR/rs/"
else
    echo "Rust tests failed"
    RS_SUCCESS=false
fi

echo ""
echo "=============================================="
echo "Coverage Summary"
echo "=============================================="
echo ""
echo "TypeScript: $([ "$TS_SUCCESS" = true ] && echo "PASS" || echo "FAIL")"
echo "Rust:       $([ "$RS_SUCCESS" = true ] && echo "PASS" || echo "FAIL")"
echo ""
echo "Reports:"
echo "  TypeScript: $COVERAGE_DIR/ts/index.html"
echo "  Rust:       $COVERAGE_DIR/rs/index.html"
echo ""

# Exit with failure if either failed
if [ "$TS_SUCCESS" = false ] || [ "$RS_SUCCESS" = false ]; then
    echo "Coverage run completed with failures"
    exit 1
fi

echo "Coverage run completed successfully"
exit 0
