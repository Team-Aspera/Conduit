#!/bin/bash
set -e

echo "=== 1. cargo fmt --check ==="
cargo fmt --check
echo "OK"

echo "=== 2. cargo clippy ==="
cargo clippy -- -D warnings
echo "OK"

echo "=== 3. cargo build --release ==="
cargo build --release
echo "OK"

echo "=== 4. cargo test ==="
cargo test
echo "OK"

echo ""
echo "All CI checks passed!"
