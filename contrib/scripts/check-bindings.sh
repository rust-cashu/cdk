#!/bin/bash

set -euo pipefail

# Check bindings
buildargs=(
    
)

for arg in "${buildargs[@]}"; do
    echo  "Checking '$arg'"
    cargo build $arg
    cargo clippy $arg -- -D warnings
    echo
done