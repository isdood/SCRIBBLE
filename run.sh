#!/bin/bash

# Exit on any error
set -e

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

    # Push to GitHub
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

# Ensure the build script uses the -C strip flag to reduce binary size
RUSTFLAGS="-C opt-level=s -C lto -C codegen-units=1 -C panic=abort -C strip=debuginfo" cargo build --release || error "Failed to build Gear1"

# Strip debug symbols from Gear1 binary
strip --strip-all target/i686-spinup/release/spinUP-gear1

# Verify Gear1
GEAR1_SIZE=$(stat -c%s "target/i686-spinup/release/spinUP-gear1")
if [ "$GEAR1_SIZE" -gt 512 ]; then
    error "Gear1 is larger than 512 bytes ($GEAR1_SIZE bytes)"
fi
status "Gear1 size: $GEAR1_SIZE bytes"
hexdump -C -n 512 target/i686-spinup/release/spinUP-gear1
success "Gear1 built successfully"

# Build Gear2
status "Building Gear2..."
cd ../gear2
./build.sh || error "Failed to build Gear2"

# Create disk image
status "Creating disk image..."
cd ../..

# Create empty disk image (1.44MB)
dd if=/dev/zero of=combined.img bs=512 count=2880 2>/dev/null

# Write Gear1 to first sector
dd if=spinUP/gear1/target/i686-spinup/release/spinUP-gear1 of=combined.img conv=notrunc bs=512 count=1 2>/dev/null

# Write Gear2 starting at second sector
dd if=spinUP/gear2/disk.img of=combined.img conv=notrunc bs=512 seek=1 2>/dev/null

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
