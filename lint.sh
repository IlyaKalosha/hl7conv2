cargo clippy --all-targets --workspace
cargo fmt --all --check
find . -mindepth 2 -name 'Cargo.toml' -exec cargo tomlfmt -k -p {} \;