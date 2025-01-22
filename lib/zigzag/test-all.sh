#!/bin/bash
set -e  # Exit on error

GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}Running ZigZag full test suite...${NC}"

# Zig tests
echo -e "\n${BLUE}1. Running Zig tests:${NC}"
zig test src/zig/vector/vector3d.zig
echo -e "${GREEN}✓ Zig tests passed${NC}"

# Rust tests
echo -e "\n${BLUE}2. Running Rust tests:${NC}"
cargo test
echo -e "${GREEN}✓ Rust tests passed${NC}"

# Julia tests
echo -e "\n${BLUE}3. Running Julia tests:${NC}"
julia --project=. -e '
    include("src/julia/quantum/quantum_vector.jl")
    using .QuantumVector
    exit(run_tests() ? 0 : 1)
'
echo -e "${GREEN}✓ Julia tests passed${NC}"

echo -e "\n${GREEN}All tests completed successfully!${NC}"
