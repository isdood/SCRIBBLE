#!/bin/bash

echo "[$(date -u '+%Y-%m-%d %H:%M:%S UTC')] Starting Lazuline test suite..."

# Build and test Rust components
echo "Building Rust components..."
cargo clean && cargo build --release

if [ $? -ne 0 ]; then
    echo "[$(date -u '+%Y-%m-%d %H:%M:%S UTC')] Error: Failed to build Rust components"
    exit 1
fi

# Build and test Zig components
echo "Building Zig components..."
cd zig/zig-bind && zig build && zig build test

if [ $? -ne 0 ]; then
    echo "[$(date -u '+%Y-%m-%d %H:%M:%S UTC')] Error: Failed to build Zig components"
    exit 1
fi

cd ../..
echo "[$(date -u '+%Y-%m-%d %H:%M:%S UTC')] ✨ All tests completed successfully!"
