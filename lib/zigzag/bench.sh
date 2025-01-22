#!/bin/bash
set -e

GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}Running ZigZag Performance Benchmarks${NC}"
echo -e "${BLUE}=====================================${NC}"

# Rust benchmarks
echo -e "\n${BLUE}Rust Implementation:${NC}"
cargo bench

# Zig benchmarks
echo -e "\n${BLUE}Zig Implementation:${NC}"
zig build-exe src/zig/vector/bench.zig -O ReleaseFast
./bench

# Julia benchmarks
echo -e "\n${BLUE}Julia Implementation:${NC}"
julia --project=. -e '
    using Pkg
    Pkg.activate(".")
    Pkg.add("BenchmarkTools")
    include("benches/julia/bench_vector.jl")
'

echo -e "\n${GREEN}All benchmarks completed!${NC}"
