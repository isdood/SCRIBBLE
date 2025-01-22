#!/bin/bash
# cleanup.sh - Fix directory structure and permissions

GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}Cleaning up ZigZag directory structure...${NC}"

# Remove nested lib/zigzag directory
rm -rf lib/

# Make sure source directories exist
mkdir -p src/{rust,zig,julia}/{core,bridge}
mkdir -p src/zig/vector
mkdir -p src/julia/quantum

# Set proper permissions
chmod 644 build.zig
chmod 644 src/zig/vector/vector3d.zig
chmod 644 src/rust/lib.rs

# Create new test script
cat > test-zig.sh << 'END_TEST'
#!/bin/bash
set -e  # Exit on error
echo "Running Zig tests..."
zig test src/zig/vector/vector3d.zig
END_TEST

chmod +x test-zig.sh
