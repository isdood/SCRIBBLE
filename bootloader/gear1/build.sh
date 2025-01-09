#!/bin/bash
set -e

echo "Building bootloader..."
cargo +nightly build \
    --release \
    --target i686-scribble.json \
    -Z build-std=core,compiler_builtins \
    -Z build-std-features=compiler-builtins-mem

# Verify bootloader size and signature
BOOTLOADER="target/i686-scribble/release/spinUP-gear1"
BOOTLOADER_SIZE=$(stat -f %z "$BOOTLOADER" 2>/dev/null || stat -c %s "$BOOTLOADER")
if [ "$BOOTLOADER_SIZE" -gt 512 ]; then
    echo "Error: Bootloader is larger than 512 bytes ($BOOTLOADER_SIZE bytes)"
    exit 1
fi

# Check boot signature
if ! tail -c 2 "$BOOTLOADER" | xxd -p | grep -q "55aa"; then
    echo "Warning: Boot signature not found in bootloader"
fi

echo "Creating disk image..."
# Create a disk image (1.44MB)
dd if=/dev/zero of=disk.img bs=1024 count=1440
# Make it bootable
dd if="$BOOTLOADER" of=disk.img conv=notrunc bs=512 count=1
# Add Gear 2
dd if=../gear2/target/x86_64-scribble/release/spinUP.bin of=disk.img conv=notrunc bs=512 seek=1

echo "Verifying disk image..."
# Show first sector
echo "Boot sector contents:"
xxd -l 512 disk.img | head

# Run QEMU
echo "Starting QEMU..."
qemu-system-i386 \
    -drive file=disk.img,format=raw,if=ide,index=0 \
    -boot c \
    -vga std \
    -display gtk \
    -serial stdio \
    -monitor telnet:127.0.0.1:55555,server,nowait \
    -d int,cpu_reset,guest_errors \
    -D qemu.log \
    -no-reboot
