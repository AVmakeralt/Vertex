#!/bin/bash
# This script is used for testing vertexC right in your ide
cargo build
if [ $? -eq 0 ]; then
    ./target/debug/vertexC "$@"
fi
