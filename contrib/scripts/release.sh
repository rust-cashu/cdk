#!/bin/bash

set -euo pipefail

args=(
    "-p cdk"
)

for arg in "${args[@]}"; 
do
    echo "Publishing '$arg'"
    cargo publish $arg
    echo
done