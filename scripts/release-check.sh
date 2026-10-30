#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

echo "=================================================================="
echo "🚀 PRE-FLIGHT RELEASE GAUNTLET (tradingview-streamer)"
echo "=================================================================="

echo "[1/6] Running full test suite (Unit + Hermetic + Live Matrix)..."
cargo test --all-targets

echo "[2/6] Running Compilation & Linters..."
cargo check --all-targets
if cargo clippy --help >/dev/null 2>&1; then
    cargo clippy --all-targets -- -D warnings
fi
if cargo fmt --version >/dev/null 2>&1; then
    cargo fmt --check
fi

echo "[3/6] Auditing Single Responsibility & Line Limits (< 350L)..."
"$REPO_ROOT/scripts/check-line-limits.sh"

echo "[4/6] Verifying Zero-Mock Invariant in production paths..."
if command -v detect-mocks >/dev/null 2>&1; then
    detect-mocks "$REPO_ROOT"
else
    echo "⚠️  detect-mocks CLI not found, skipping static mock check."
fi

echo "[5/6] Auditing arc42 Architecture Freshness & Provenance..."
if command -v arc42-lint >/dev/null 2>&1; then
    arc42-lint "$REPO_ROOT/docs/architecture"
else
    echo "⚠️  arc42-lint CLI not found, skipping architecture drift check."
fi

echo "[6/6] Verifying Cargo Packaging & Dry-Run Upload..."
cargo publish --dry-run --allow-dirty

echo "=================================================================="
echo "✅ PRE-FLIGHT RELEASE GAUNTLET PASSED: READY FOR PUBLISHING"
echo "=================================================================="
