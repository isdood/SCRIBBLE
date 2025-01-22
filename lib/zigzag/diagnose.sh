#!/bin/bash
# diagnose.sh - Check ZigZag setup

GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}Checking ZigZag setup...${NC}"

# Check build.zig
echo -e "\n${BLUE}Checking build.zig:${NC}"
if [ -f "build.zig" ]; then
    echo -e "${GREEN}build.zig exists${NC}"
    echo "Content:"
    cat build.zig
else
    echo -e "${RED}build.zig is missing${NC}"
fi

# Check vector3d.zig
echo -e "\n${BLUE}Checking src/zig/vector/vector3d.zig:${NC}"
if [ -f "src/zig/vector/vector3d.zig" ]; then
    echo -e "${GREEN}vector3d.zig exists${NC}"
    echo "Content:"
    cat src/zig/vector/vector3d.zig
else
    echo -e "${RED}vector3d.zig is missing${NC}"
fi

# Check directory structure
echo -e "\n${BLUE}Directory structure:${NC}"
tree -L 3

echo -e "\n${BLUE}Zig version:${NC}"
zig version
