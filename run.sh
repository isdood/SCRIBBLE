#!/bin/bash

# Exit on any error
set -e

clear;

# Colors for output
RED="\033[0;31m"
GREEN="\033[0;32m"
YELLOW="\033[1;33m"
NC="\033[0m" # No Color

# Function to print status messages
status() {
    echo -e "${YELLOW}>>> $1${NC}"
}

# Function to print success messages
success() {
    echo -e "${GREEN}✓ $1${NC}"
}

# Function to print error messages
error() {
    echo -e "${RED}✗ $1${NC}"
    exit 1
}

# Cleanup function
cleanup() {
    status "Cleaning up, pushing to isdood/scribble..."

    #Push to github
    echo ""
    git add -A
    git commit -m "dbg."
    git push
    killall qemu-system-x86_64 2>/dev/null || true
    rm -f serial.log combined.img qemu.log
}

# Set up cleanup on script exit
trap cleanup EXIT

# Build Gear1
status "Building Gear1..."
cd spinUP/gear1

# Check if NASM is installed
if ! command -v nasm &> /dev/null; then
    error "NASM is not installed. Please install it first."
fi

# Assemble Gear1
status "=== Building ScribbleOS SpinUP Gear1 ==="
nasm -f bin src/boot.asm -o gear1.bin || error "Failed to assemble Gear1"

# Verify Gear1
GEAR1_SIZE=$(stat -c%s "gear1.bin")
if [ "$GEAR1_SIZE" -ne 512 ]; then
    error "SpinUP size ($GEAR1_SIZE bytes) exceeds 512 bytes"
fi
status "Gear1 size: $GEAR1_SIZE bytes"
hexdump -C -n 512 gear1.bin
success "Gear1 built successfully"

# Build Gear2
status "Building Gear2..."
cd ../gear2
./build.sh || error "Failed to build Gear2"

# Verify Gear2 output
if [ ! -f "gear2.bin" ]; then
    error "gear2.bin not found. Build might have failed."
fi

# Create disk image
status "Creating disk image..."
cd ../..

# Combine Gear1 and Gear2 into one disk image
dd if=spinUP/gear1/gear1.bin of=combined.img bs=512 count=1
dd if=spinUP/gear2/gear2.bin of=combined.img bs=512 seek=1

success "Created disk image"

# Verify disk image
status "Verifying disk image..."
echo "First sector (Gear1):"
hexdump -C -n 512 combined.img
echo
echo "Second sector (Start of Gear2):"
hexdump -C -s 512 -n 512 combined.img
echo

success "Disk image verified"

# Run in QEMU
status "Starting QEMU..."

# Start QEMU with appropriate options
qemu-system-x86_64 \
    -drive file=combined.img,format=raw,if=floppy \
    -serial stdio \
    -display none \
    -d int,cpu_reset \
    -no-reboot &

QEMU_PID=$!

# Wait for QEMU to start
sleep 2

# Check if QEMU is still running
if ! kill -0 $QEMU_PID 2>/dev/null; then
    error "QEMU failed to start"
fi

# Wait a bit for output
sleep 3

# Kill QEMU gracefully
kill $QEMU_PID

# Check if we got any output indicating successful handoff
if dmesg | grep -q "Gear 2.*Starting" || grep -q "Gear 2.*Starting" qemu.log 2>/dev/null; then
    success "Bootloader handoff successful!"
else
    error "Bootloader handoff failed"
fi

success "Test completed"
