#!/bin/bash
# test.sh - Run all tests for ZigZag

GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}Running ZigZag tests...${NC}"

echo -e "\n${BLUE}Running Zig tests:${NC}"
if ! zig build test; then
    echo -e "${RED}Zig tests failed${NC}"
fi

echo -e "\n${BLUE}Running Rust tests:${NC}"
if ! cargo test; then
    echo -e "${RED}Rust tests failed${NC}"
fi

echo -e "\n${BLUE}To run Julia tests:${NC}"
echo "Start julia and run:"
echo "include(\"src/julia/quantum/quantum_vector.jl\")"
echo "QuantumVector.run_tests()"
