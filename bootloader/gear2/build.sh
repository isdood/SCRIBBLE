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

# Convert ELF to binary format
objcopy_cmd="llvm-objcopy"
if ! command -v $objcopy_cmd &> /dev/null; then
    objcopy_cmd="objcopy"
fi

$objcopy_cmd -O binary \
    target/x86_64-scribble/release/spinUP \
    target/x86_64-scribble/release/spinUP.bin

# Get bootloader size
bootloader_size=$(wc -c < target/x86_64-scribble/release/spinUP.bin)
echo "Second-stage bootloader size: $bootloader_size bytes"

# Calculate number of sectors needed (512 bytes per sector)
sectors=$(( ($bootloader_size + 511) / 512 ))

# Create disk image (2880 sectors = 1.44MB floppy)
dd if=/dev/zero of=disk.img bs=512 count=2880
dd if=target/x86_64-scribble/release/spinUP.bin of=disk.img conv=notrunc bs=512 seek=1 count=$sectors

echo "Bootloader written to disk image ($sectors sectors)"
echo "Bootloader hexdump:"
hexdump -C disk.img | head -n 8
