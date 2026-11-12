#! /bin/sh

# Run all `saw.py` files in virtual environment for SAW Remote API.
set -e
export SAW_SOLVER_CACHE_PATH=${SAW_SOLVER_CACHE_PATH:=$(pwd)/saw-cache}
export VENV_SAW=${VENV_SAW:=/opt/venv/saw-remote-api}
find . -name "saw.py" -print0 | while IFS= read -r -d '' f; do
    "$VENV_SAW/bin/python3" "$f"
done
