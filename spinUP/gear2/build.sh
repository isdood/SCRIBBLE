#!/bin/bash
set -e

# Ensure we're using nightly
rustup override set nightly

# Build the bootloader
RUSTFLAGS="-C link-arg=-nostartfiles" cargo build --release --target x86_64-unknown-none

# Get the binary size
BOOTLOADER_SIZE=$(stat -c%s "target/x86_64-unknown-none/release/spinUP")

echo "Second-stage bootloader size: $BOOTLOADER_SIZE bytes"

# Create disk image
dd if=/dev/zero of=disk.img bs=512 count=2880
dd if=target/x86_64-unknown-none/release/spinUP of=disk.img conv=notrunc bs=512 seek=1

# Show first few sectors
echo "Bootloader written to disk image (2 sectors)"
echo "Bootloader hexdump:"
hexdump -C -n 512 disk.img
