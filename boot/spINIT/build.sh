#!/bin/bash
set -e

echo "=== Building ScribbleOS SpinUP Gear1 ==="

# Ensure we're using nightly
echo "Checking Rust toolchain..."
rustup default nightly

# Build with explicit target
echo "Building Gear1..."
RUSTFLAGS="-C link-arg=-n" cargo +nightly build \
    --release \
    --target ./i686-spinup.json \
    -Z build-std=core,compiler_builtins \
    -Z build-std-features=compiler-builtins-mem

# Check size - handle both BSD and GNU stat variants
BINARY="target/i686-spinup/release/spinUP-gear1"

if [ ! -f "$BINARY" ]; then
    echo "Error: Binary file not found at $BINARY"
    exit 1
fi

# Try different stat commands until one works
if SIZE=$(stat -f%z "$BINARY" 2>/dev/null); then
    : # BSD stat worked
elif SIZE=$(stat --printf="%s" "$BINARY" 2>/dev/null); then
    : # GNU stat worked
elif SIZE=$(wc -c < "$BINARY" 2>/dev/null); then
    : # Fallback to wc -c
else
    echo "Error: Could not determine file size using available tools"
    exit 1
fi

if ! [[ "$SIZE" =~ ^[0-9]+$ ]]; then
    echo "Error: Invalid file size obtained: $SIZE"
    exit 1
fi

if [ "$SIZE" -gt 512 ]; then
    echo "Error: SpinUP size ($SIZE bytes) exceeds 512 bytes"
    exit 1
fi

echo "SpinUP build successful! Size: $SIZE bytes"
