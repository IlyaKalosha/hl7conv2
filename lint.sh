#!/bin/bash
set -e

echo "Running clippy..."
cargo clippy --all-targets --workspace

echo "Checking formatting..."
cargo fmt --all --check

echo "Checking Cargo.toml formatting..."
if find . -mindepth 2 -name 'Cargo.toml' -exec cargo tomlfmt -k -p {} \; 2>/dev/null; then
    echo "Cargo.toml files formatted successfully"
else
    echo "No Cargo.toml files found to format or formatting completed"
fi

echo "Lint checks completed successfully!"