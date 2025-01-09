#!/bin/bash
set -e

# Build Gear 1
cargo +nightly build \
    --release \
    --target i686-scribble.json \
    -Z build-std=core,compiler_builtins \
    -Z build-std-features=compiler-builtins-mem

# Convert to flat binary
objcopy -O binary \
    target/i686-scribble/release/spinUP-gear1 \
    target/i686-scribble/release/gear1.bin

# Get size
size=$(wc -c < target/i686-scribble/release/gear1.bin)
echo "Gear 1 size: $size bytes"

# Verify size
if [ $size -gt 512 ]; then
    echo "Error: Gear 1 is too large (must be <= 512 bytes)"
    exit 1
fi

# Create disk image
dd if=/dev/zero of=disk.img bs=512 count=2880
dd if=target/i686-scribble/release/gear1.bin of=disk.img conv=notrunc bs=512 count=1
dd if=../gear2/target/x86_64-scribble/release/spinUP.bin of=disk.img conv=notrunc bs=512 seek=1

echo "Boot disk created successfully"
hexdump -C disk.img | head -n 16
