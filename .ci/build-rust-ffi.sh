#! /bin/sh

# Run `cargo build --release` on all Rust projects.
set -e
CARGO_TOML_FILES=`find . -name "Cargo.toml"`
for f in $CARGO_TOML_FILES; do cargo build --manifest-path $f --release; done;
