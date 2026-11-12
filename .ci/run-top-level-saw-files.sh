#! /bin/sh

# Run `saw` on all `all.saw` files.
set -e
export SAW_SOLVER_CACHE_PATH=${SAW_SOLVER_CACHE_PATH:=$(pwd)/saw-cache}
find . -name "all.saw" -print0 | while IFS= read -r -d '' f; do
    saw "$f"
done
