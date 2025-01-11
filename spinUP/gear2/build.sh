#!/bin/bash
set -e

# Ensure we're using nightly
rustup override set nightly

# Get the script's directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

# Build the bootloader using the custom target from the same directory
cargo build --release --target "$SCRIPT_DIR/x86_64-spinUP.json"

# Get the binary size
BOOTLOADER_SIZE=$(stat -c%s "target/x86_64-spinUP/release/spinUP")

echo "Second-stage bootloader size: $BOOTLOADER_SIZE bytes"

# Create disk image
dd if=/dev/zero of=disk.img bs=512 count=2880
dd if=target/x86_64-spinUP/release/spinUP of=disk.img conv=notrunc bs=512 seek=1

# Show first few sectors
echo "Bootloader written to disk image (2 sectors)"
echo "Bootloader hexdump:"
hexdump -C -n 512 disk.img
