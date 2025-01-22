#!/bin/bash
# setup_benchmarks.sh - Create benchmarking infrastructure
# Created: 2025-01-21 21:39:42 UTC
# Author: isdood

GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}Setting up ZigZag benchmarking suite...${NC}"

# 1. Create Rust benchmarks
echo -e "\n${BLUE}1. Creating Rust benchmarks...${NC}"
mkdir -p benches/rust
cat > benches/vector_ops.rs << 'END_RUST_BENCH'
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use zigzag::Vector3D;

fn bench_vector_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("vector_operations");

    // Dot product benchmarks
    let v1 = Vector3D::new(1.0, 2.0, 3.0);
    let v2 = Vector3D::new(4.0, 5.0, 6.0);

    group.bench_function("dot_product", |b| {
        b.iter(|| v1.dot(black_box(&v2)))
    });

    // Magnitude benchmarks
    group.bench_function("magnitude", |b| {
        b.iter(|| v1.magnitude())
    });

    // Coherence effects
    for coherence in [0.0, 0.25, 0.5, 0.75, 1.0].iter() {
        let mut v_coh = v1;
        v_coh.set_coherence(*coherence);

        group.bench_with_input(
            BenchmarkId::new("quantum_dot", coherence),
            coherence,
            |b, &_c| b.iter(|| v_coh.dot(black_box(&v2)))
        );
    }

    group.finish();
}

criterion_group!(benches, bench_vector_ops);
criterion_main!(benches);
END_RUST_BENCH

# 2. Create Zig benchmarks
echo -e "\n${BLUE}2. Creating Zig benchmarks...${NC}"
mkdir -p benches/zig
cat > src/zig/vector/bench.zig << 'END_ZIG_BENCH'
const std = @import("std");
const Vector3D = @import("vector3d.zig").Vector3D;
const time = std.time;
const print = std.debug.print;

pub fn main() !void {
    // Setup
    var timer = try time.Timer.start();
    const iterations: u32 = 1_000_000;

    const v1 = Vector3D.init(1.0, 2.0, 3.0);
    const v2 = Vector3D.init(4.0, 5.0, 6.0);

    // Benchmark dot product
    timer.reset();
    var result: f64 = 0;
    var i: u32 = 0;
    while (i < iterations) : (i += 1) {
        result += v1.dot(v2);
    }
    const dot_time = @as(f64, @floatFromInt(timer.lap())) / @as(f64, @floatFromInt(time.ns_per_s));

    // Benchmark magnitude
    timer.reset();
    i = 0;
    while (i < iterations) : (i += 1) {
        result += v1.magnitude();
    }
    const mag_time = @as(f64, @floatFromInt(timer.lap())) / @as(f64, @floatFromInt(time.ns_per_s));

    print(
        \\Benchmark Results ({d} iterations):
        \\Dot Product: {d:.6}s ({d:.2} ns/iter)
        \\Magnitude:   {d:.6}s ({d:.2} ns/iter)
        \\
    , .{
        iterations,
        dot_time,
        dot_time * 1e9 / @as(f64, @floatFromInt(iterations)),
        mag_time,
        mag_time * 1e9 / @as(f64, @floatFromInt(iterations)),
    });
}
END_ZIG_BENCH

# 3. Create Julia benchmarks
echo -e "\n${BLUE}3. Creating Julia benchmarks...${NC}"
mkdir -p benches/julia
cat > benches/julia/bench_vector.jl << 'END_JULIA_BENCH'
using BenchmarkTools
include("../../src/julia/quantum/quantum_vector.jl")
using .QuantumVector

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
    for coherence in [0.0, 0.25, 0.5, 0.75, 1.0]
        v_coh = create_quantum_vector(1.0, 2.0, 3.0)
        v_coh.state = QuantumState([Complex{Float64}(1.0, 0.0)], coherence)
        bench = @benchmark quantum_dot($v_coh, $v2)
        println("\nCoherence = $coherence:")
        display(bench)
    end
end

run_benchmarks()
END_JULIA_BENCH

# 4. Create benchmark runner script
echo -e "\n${BLUE}4. Creating benchmark runner...${NC}"
cat > run_benchmarks.sh << 'END_BENCH_SCRIPT'
#!/bin/bash
set -e

GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}Running ZigZag benchmarks...${NC}"

# Rust benchmarks
echo -e "\n${BLUE}Running Rust benchmarks:${NC}"
cargo bench

# Zig benchmarks
echo -e "\n${BLUE}Running Zig benchmarks:${NC}"
zig build-exe src/zig/vector/bench.zig -O ReleaseFast
./bench

# Julia benchmarks
echo -e "\n${BLUE}Running Julia benchmarks:${NC}"
julia --project=. -e '
    using Pkg
    Pkg.add("BenchmarkTools")
    include("benches/julia/bench_vector.jl")
'

echo -e "\n${GREEN}All benchmarks completed!${NC}"
END_BENCH_SCRIPT

# 5. Update build.zig to include benchmark
echo -e "\n${BLUE}5. Updating Zig build system...${NC}"
cat > build.zig << 'END_BUILD'
const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Tests
    const tests = b.addTest(.{
        .root_source_file = .{ .path = "src/zig/vector/vector3d.zig" },
        .target = target,
        .optimize = optimize,
    });

    const test_step = b.step("test", "Run all tests");
    test_step.dependOn(&b.addRunArtifact(tests).step);

    // Benchmark executable
    const bench = b.addExecutable(.{
        .name = "bench",
        .root_source_file = .{ .path = "src/zig/vector/bench.zig" },
        .target = target,
        .optimize = .ReleaseFast,
    });

    const run_bench = b.addRunArtifact(bench);
    const bench_step = b.step("bench", "Run benchmarks");
    bench_step.dependOn(&run_bench.step);
}
END_BUILD

# 6. Set permissions
echo -e "\n${BLUE}6. Setting permissions...${NC}"
chmod +x run_benchmarks.sh

echo -e "\n${GREEN}Benchmark suite setup complete!${NC}"
echo -e "${BLUE}To run all benchmarks, execute:${NC} ./run_benchmarks.sh"
