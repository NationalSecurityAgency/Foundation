#! /bin/sh

# Run `cargo build --release` on all Rust projects.
set -e
find . -name "Cargo.toml" -print0 | while IFS= read -r -d '' f; do
    cargo build --manifest-path "$f" --release
done
