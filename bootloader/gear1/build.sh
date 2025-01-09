#!/bin/bash
set -e

# Clean previous build
echo "Cleaning previous build..."
cargo clean

# Build Gear 1
echo "Building Gear 1..."
cargo +nightly build \
    --release \
    --target i686-scribble.json \
    -Z build-std=core,compiler_builtins \
    -Z build-std-features=compiler-builtins-mem

# Create disk image
echo "Creating disk image..."
dd if=/dev/zero of=disk.img bs=512 count=2880
dd if=target/i686-scribble/release/spinUP-gear1 of=disk.img conv=notrunc bs=512 count=1
dd if=../gear2/target/x86_64-scribble/release/spinUP.bin of=disk.img conv=notrunc bs=512 seek=1

# Run QEMU with debugging options
echo "Testing in QEMU..."
qemu-system-i386 \
    -drive file=disk.img,format=raw,if=ide,index=0 \
    -boot order=c \
    -vga std \
    -display gtk \
    -serial stdio \
    -monitor telnet:127.0.0.1:55555,server,nowait \
    -d cpu_reset,int,guest_errors,in_asm \
    -D qemu.log \
    -no-reboot

# Provide instructions for debugging
echo "
Debugging Information:
- Serial output will appear in this terminal
- VGA output will appear in the QEMU window
- Connect to QEMU monitor using: telnet localhost 55555
- CPU resets, interrupts, and guest errors will be logged
- Check qemu.log for detailed debug information
"
