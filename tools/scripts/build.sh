#!/bin/bash

# Build kernel
cargo build --release

# Check if build was successful
if [ $? -eq 0 ]; then
    echo "Build successful! Preparing to run in QEMU..."
else
    echo "Build failed. Please check the errors."
    exit 1
fi