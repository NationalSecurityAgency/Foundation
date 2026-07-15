#! /bin/sh

# Run `cargo saw-build` on all Rust projects.
set -e
CARGO_TOML_FILES=`find . -name "Cargo.toml"`
for f in $CARGO_TOML_FILES; do cargo saw-build --manifest-path $f --release; done

# Rename mir json files and move them next to associated `Cargo.toml`.
find . -name "*.linked-mir.json" -exec sh -c "mv '{}' \$(dirname '{}')/../../../../linked-mir.json" \;
