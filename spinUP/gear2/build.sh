#!/bin/bash
set -e

# Colors
RED="\033[0;31m"
GREEN="\033[0;32m"
YELLOW="\033[1;33m"
NC="\033[0m"

# Print functions
status() { echo -e "${YELLOW}>>> $1${NC}"; }
success() { echo -e "${GREEN}✓ $1${NC}"; }
error() { echo -e "${RED}✗ $1${NC}"; exit 1; }

# Ensure we're using nightly
status "Setting up toolchain..."
rustup override set nightly
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu

# Build gear2
status "Building gear2..."
RUSTFLAGS="-C link-arg=-N" \
cargo build --release \
    -Z build-std=core,compiler_builtins \
    -Z build-std-features=compiler-builtins-mem \
    --target x86_64-spinUP.json || error "Build failed"

success "Build completed successfully"

# Show size info
size=$(stat -f %z "target/x86_64-spinUP/release/spinUP")
echo "Binary size: $size bytes"
