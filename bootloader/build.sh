#!/bin/bash
set -e

# Clean previous builds
cargo clean

# Build with release profile
cargo +nightly build \
    --release \
    --target x86_64-scribble.json \
    -Z build-std=core,compiler_builtins \
    -Z build-std-features=compiler-builtins-mem

# Check binary size
bootloader_size=$(wc -c < target/x86_64-scribble/release/spinUP)
if [ "$bootloader_size" -gt 512 ]; then
    echo "Error: Bootloader size ($bootloader_size bytes) exceeds 512 bytes!"
    exit 1
fi

# Create disk image
dd if=/dev/zero of=disk.img bs=512 count=2880
dd if=target/x86_64-scribble/release/spinUP of=disk.img conv=notrunc bs=512 count=1

# Show the first bytes of the disk image
echo "Bootloader hexdump:"
hexdump -C disk.img | head -n 8

echo "Bootloader size: $bootloader_size bytes"
