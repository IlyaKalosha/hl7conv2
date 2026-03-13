#!/bin/bash
set -e

if ! command -v cargo-llvm-cov &>/dev/null; then
    echo "cargo-llvm-cov is not installed. Install with:"
    echo "  rustup component add llvm-tools-preview"
    echo "  cargo install cargo-llvm-cov"
    exit 1
fi

cargo llvm-cov test --all-features --lcov --output-path lcov.info
cargo llvm-cov report
echo ""
echo "HTML report: cargo llvm-cov report --html (open target/llvm-cov/html/index.html)"
