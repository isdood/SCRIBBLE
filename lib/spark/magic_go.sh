#!/usr/bin/env bash
# fix_forge_output_simple.sh - Fix Forge output formatting
# Author: isdood
# Created: 2025-01-25 15:19:12

set -uxo pipefail

# Use tput for color codes
PURPLE=$(tput setaf 5)
RESET=$(tput sgr0)

echo_magic() {
    # Simple single echo with color
    echo "${PURPLE}✨ $1${RESET}"
}

# Update safety module
cat > forge/src/compiler/safety/mod.rs << 'EOF'
#[allow(dead_code)]  // Allow unused variants since they'll be used in the future
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SafetyLevel {
    Calm,
    Balanced,
    Wild,
}

impl Default for SafetyLevel {
    fn default() -> Self {
        SafetyLevel::Calm
    }
}
EOF

# Rebuild the project
echo_magic "Rebuilding forge compiler..."
cargo clean
cargo build --release

# Reinstall the binary
echo_magic "Updating forge binary..."
cp "target/release/forge" "bin/forge"
chmod +x "bin/forge"

# Test to ensure it still works
echo_magic "Testing forge compiler..."
./bin/forge compile tests/forge/safety_test.spk --test
