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

# Build Garnet
status "Building Garnet..."
cargo build --examples || error "Failed to build Garnet"
success "Garnet built successfully"

# Run the simple example
status "Running simple example..."
RUST_BACKTRACE=1 cargo run --example simple || error "Failed to run simple example"
success "Simple example completed"

# Run with QEMU integration
status "Testing QEMU integration..."
qemu-system-x86_64 \
    -display gtk \
    -serial stdio \
    -device virtio-serial \
    -chardev stdio,id=garnet \
    -device virtconsole,chardev=garnet \
    -monitor unix:qemu-monitor-socket,server,nowait \
    -kernel target/debug/examples/simple
