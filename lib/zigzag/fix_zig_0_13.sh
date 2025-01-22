#!/bin/bash
# fix_zig_0_13.sh - Fix Zig 0.13.0 specific build configuration
# Created: 2025-01-21 21:55:12
# Author: isdood

GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}Fixing Zig 0.13.0 configuration...${NC}"

# First fix the build script
cat > build_bench.zig << 'END_BUILD'
const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});

    const exe = b.addExecutable(.{
        .name = "zigbench",
        .root_source_file = .{ .cwd_relative = "src/zig/bench_main.zig" },
        .target = target,
        .optimize = std.builtin.OptimizeMode.ReleaseFast,
    });

    b.installArtifact(exe);
}
END_BUILD

# Then fix the run script
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
zig build --build-file build_bench.zig
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

echo -e "${GREEN}Build and run scripts fixed for Zig 0.13.0!${NC}"
echo -e "${BLUE}Now try running:${NC} ./run_bench.sh"
