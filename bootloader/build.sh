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

# Create disk image
dd if=/dev/zero of=disk.img bs=512 count=2880
dd if=target/x86_64-scribble/release/spinUP of=disk.img conv=notrunc bs=512 count=1

# Show the first bytes of the disk image
echo "Bootloader hexdump:"
hexdump -C disk.img | head -n 8
