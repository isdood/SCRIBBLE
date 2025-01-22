#!/bin/bash
# fix_run.sh - Fix benchmark runner script
# Created: 2025-01-21 21:54:15
# Author: isdood

GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}Fixing benchmark runner...${NC}"

echo -e "${BLUE}Checking Zig version...${NC}"
zig version

cat > run_bench.sh << 'END_SCRIPT'
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
zig build --build-file build_bench.zig -O ReleaseFast
echo -e "${BLUE}Running Zig benchmarks...${NC}"
./zig-out/bin/zigbench

# Run Rust benchmarks
echo -e "\n${BLUE}Running Rust benchmarks...${NC}"
cargo bench

# Run Julia benchmarks
echo -e "\n${BLUE}Running Julia benchmarks...${NC}"
julia --project=. benches/julia/bench_vector.jl

END_SCRIPT

chmod +x run_bench.sh

echo -e "${GREEN}Benchmark runner fixed!${NC}"
echo -e "${BLUE}Now try running:${NC} ./run_bench.sh"
