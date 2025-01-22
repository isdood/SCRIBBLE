#!/bin/bash
set -e

GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}Running ZigZag Benchmarks${NC}"
echo -e "${BLUE}=====================${NC}\n"

# Build and run Zig benchmarks
echo -e "${BLUE}Building Zig benchmarks...${NC}"
zig build --build-file build_bench.zig
echo -e "${BLUE}Running Zig benchmarks...${NC}"
./zig-out/bin/zigbench

# Run Rust benchmarks
echo -e "\n${BLUE}Running Rust benchmarks...${NC}"
cargo bench

# Run Julia benchmarks
echo -e "\n${BLUE}Running Julia benchmarks...${NC}"
julia --project=. benches/julia/bench_vector.jl

