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

# Ensure unstable_matter is accessible to gear2
status "Setting up dependencies..."
if [ ! -L "spinUP/gear2/lib/unstable_matter" ]; then
    mkdir -p spinUP/gear2/lib
    ln -sf ../../lib/unstable_matter spinUP/gear2/lib/unstable_matter
fi
success "Dependencies set up"

# Build the kernel
status "Building kernel..."
cargo build --release || error "Failed to build kernel"
success "Kernel built successfully"

# Build Gear1
status "Building Gear1..."
cd spinUP/gear1
./build.sh || error "Failed to build Gear1"

# Verify Gear1
GEAR1_SIZE=$(stat -c%s "target/i686-spinup/release/spinUP-gear1")
if [ "$GEAR1_SIZE" -gt 512 ]; then
    error "Gear1 is larger than 512 bytes ($GEAR1_SIZE bytes)"
fi
success "Gear1 built successfully"

# Build Gear2
status "Building Gear2..."
cd ../gear2
./build.sh || error "Failed to build Gear2"
success "Gear2 built successfully"

# Create disk image
status "Creating disk image..."
cd ../..

# Calculate sizes and offsets
KERNEL_SIZE=$(stat -c%s "target/x86_64-unknown-none/release/scribble")
GEAR2_SIZE=$(stat -c%s "spinUP/gear2/target/x86_64-spinUP/release/spinUP")

# Create empty disk image (1.44MB)
dd if=/dev/zero of=combined.img bs=512 count=2880 2>/dev/null

# Write Gear1 to first sector
dd if=spinUP/gear1/target/i686-spinup/release/spinUP-gear1 of=combined.img bs=512 conv=notrunc

# Write Gear2 starting at second sector
dd if=spinUP/gear2/target/x86_64-spinUP/release/spinUP of=combined.img bs=512 seek=1 conv=notrunc

# Write kernel after Gear2 (starting at sector 33 to leave room for Gear2)
dd if=target/x86_64-unknown-none/release/scribble of=combined.img bs=512 seek=33 conv=notrunc

success "Created disk image"

# Run in QEMU
status "Starting QEMU..."

# Start QEMU with appropriate options
qemu-system-x86_64 \
    -drive file=combined.img,format=raw,if=floppy \
    -serial stdio \
    -monitor unix:qemu-monitor-socket,server,nowait \
    -d int,cpu_reset \
    -no-shutdown \
    -no-reboot \
    -display gtk \  # Add this line to see the output
    2>&1 | tee qemu.log

success "QEMU started"
