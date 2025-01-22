#!/bin/bash
# fix_benchmarks.sh - Fix benchmark issues
# Created: 2025-01-21 21:47:53
# Author: isdood

GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}Fixing benchmark issues...${NC}"

# 1. Fix Julia benchmarks
echo -e "\n${BLUE}1. Fixing Julia benchmarks...${NC}"
mkdir -p benches/julia
cat > benches/julia/bench_vector.jl << 'END_JULIA_BENCH'
using BenchmarkTools
include("../../src/julia/quantum/quantum_vector.jl")
using .QuantumVector
using .QuantumVector: QuantumState  # Explicitly import QuantumState

function run_benchmarks()
    # Setup vectors
    v1 = create_quantum_vector(1.0, 2.0, 3.0)
    v2 = create_quantum_vector(4.0, 5.0, 6.0)

    println("ZigZag Julia Benchmarks")
    println("======================")

    # Benchmark dot product
    dot_bench = @benchmark quantum_dot($v1, $v2)
    println("\nDot Product:")
    display(dot_bench)

    # Benchmark with different coherence values
    println("\nQuantum Dot Product with varying coherence:")
    coherence_results = Dict{Float64, BenchmarkTools.Trial}()

    for coh in [0.0, 0.25, 0.5, 0.75, 1.0]
        v_coh = create_quantum_vector(1.0, 2.0, 3.0)
        # Create new quantum state with specific coherence
        new_state = QuantumState([Complex{Float64}(1.0, 0.0)], convert(Float64, coh))
        # Create new vector with modified state
        v_test = QuantumVector3D(v_coh.x, v_coh.y, v_coh.z, new_state)

        bench = @benchmark quantum_dot($v_test, $v2)
        println("\nCoherence = $coh:")
        display(bench)
        coherence_results[coh] = bench
    end
end

run_benchmarks()
END_JULIA_BENCH

# 2. Fix Zig benchmarks
echo -e "\n${BLUE}2. Fixing Zig benchmarks...${NC}"
mkdir -p src/zig
cat > src/zig/bench_main.zig << 'END_ZIG'
const std = @import("std");
const Vector3D = @import("vector/vector3d.zig").Vector3D;
const print = std.debug.print;

pub fn main() !void {
    // Warm up the CPU
    var warmup: u64 = 0;
    for (0..1000000) |_| {
        warmup += 1;
    }
    _ = warmup;

    const iterations: u64 = 10_000_000;
    var timer = try std.time.Timer.start();

    // Setup test vectors
    const v1 = Vector3D.init(1.0, 2.0, 3.0);
    const v2 = Vector3D.init(4.0, 5.0, 6.0);
    var result: f64 = 0.0;

    // Benchmark dot product
    timer.reset();
    for (0..iterations) |_| {
        result += v1.dot(v2);
    }
    const dot_time = @as(f64, @floatFromInt(timer.lap())) / std.time.ns_per_s;

    // Benchmark magnitude
    timer.reset();
    for (0..iterations) |_| {
        result += v1.magnitude();
    }
    const mag_time = @as(f64, @floatFromInt(timer.lap())) / std.time.ns_per_s;

    // Print results (use result to prevent optimization)
    if (result == 0.0) {
        print("Unexpected zero result\n", .{});
    }

    print(
        \\
        \\Zig Benchmark Results ({d} iterations):
        \\----------------------------------------
        \\Dot Product: {d:.9}s ({d:.3} ns/iter)
        \\Magnitude:   {d:.9}s ({d:.3} ns/iter)
        \\
        \\
    , .{
        iterations,
        dot_time,
        dot_time * 1e9 / @as(f64, @floatFromInt(iterations)),
        mag_time,
        mag_time * 1e9 / @as(f64, @floatFromInt(iterations)),
    });
}
END_ZIG

# 3. Create build script for benchmarks
echo -e "\n${BLUE}3. Creating build script...${NC}"
cat > build_bench.zig << 'END_BUILD'
const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const exe = b.addExecutable(.{
        .name = "zigbench",
        .root_source_file = .{ .path = "src/zig/bench_main.zig" },
        .target = target,
        .optimize = .ReleaseFast,
    });

    b.installArtifact(exe);

    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());

    const run_step = b.step("bench", "Run the benchmarks");
    run_step.dependOn(&run_cmd.step);
}
END_BUILD

# 4. Create benchmark runner
echo -e "\n${BLUE}4. Creating benchmark runner...${NC}"
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
zig build -Doptimize=ReleaseFast --build-file build_bench.zig
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

echo -e "\n${GREEN}Benchmark fixes applied!${NC}"
echo -e "${BLUE}To run the benchmarks, execute:${NC} ./run_bench.sh"
