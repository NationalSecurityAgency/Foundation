#! /bin/sh

# Run `saw` on all `all.saw` files.
set -e
export SAW_SOLVER_CACHE_PATH=${SAW_SOLVER_CACHE_PATH:=$(pwd)/saw-cache}
SAWFILES=`find . -name "all.saw"`
for f in $SAWFILES; do saw $f; done;
