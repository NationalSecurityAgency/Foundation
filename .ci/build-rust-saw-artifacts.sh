#! /bin/sh

# Run `cargo saw-build` on all Rust projects.
set -e
find . -name "Cargo.toml" -print0 | while IFS= read -r -d '' f; do
    cargo saw-build --manifest-path "$f" --release
done

# Rename mir json files and move them next to associated `Cargo.toml`.
find . -name "*.linked-mir.json" -exec sh -c 'mv "$1" "$(dirname "$1")/../../../../linked-mir.json"' _ {} \;
