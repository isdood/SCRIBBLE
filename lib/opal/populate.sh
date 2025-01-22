#!/bin/bash

echo "Creating OPAL benchmark suite..."

# Ensure all necessary directories are created
mkdir -p benchmarks/zig
mkdir -p benchmarks/rust
mkdir -p benchmarks/julia

# Populate Zig benchmark file
cat > benchmarks/zig/benchmark.zig << 'EOL'
const std = @import("std");

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();

    const start = std.time.timestamp();
    // Call the function you want to benchmark
    const result = try benchmark_resonance_field();
    const end = std.time.timestamp();

    const elapsed = end - start;
    try stdout.print("Benchmark completed in {d} ns\n", .{ elapsed });
}

fn benchmark_resonance_field() !void {
    const harmony = @import("../src/zig/core/harmony.zig");
    var field = try harmony.ResonanceField.init();
    try field.optimize();
}
EOL

# Populate Rust benchmark file
cat > benchmarks/rust/benchmark.rs << 'EOL'
extern crate test;
use test::Bencher;
use opal::harmony::HarmonyCore;

#[bench]
fn benchmark_harmony_core(b: &mut Bencher) {
    b.iter(|| {
        let mut core = HarmonyCore::new();
        core.optimize();
    });
}
EOL

# Populate Julia benchmark file
cat > benchmarks/julia/benchmark.jl << 'EOL'
using BenchmarkTools
include("../src/julia/core.jl")

using .OpalCore

function benchmark_resonance_field()
    field = ResonanceField()
    @btime optimize!($field)
end

function benchmark_crystal_lattice()
    lattice = CrystalLattice()
    @btime optimize!($lattice)
end

println("Benchmarking Resonance Field:")
benchmark_resonance_field()

println("Benchmarking Crystal Lattice:")
benchmark_crystal_lattice()
EOL

# Create Cargo.toml for Rust benchmark dependencies
cat >> Cargo.toml << 'EOL'

[dev-dependencies]
test = "0.1.0"
EOL

# Create `test` feature in `Cargo.toml`
cat >> Cargo.toml << 'EOL'

[features]
test = []
EOL

# Create a helper script to run benchmarks
cat > run_benchmarks.sh << 'EOL'
#!/bin/bash

echo "Running Zig benchmark..."
zig build-exe benchmarks/zig/benchmark.zig
./benchmark

echo "Running Rust benchmark..."
cargo bench

echo "Running Julia benchmark..."
julia benchmarks/julia/benchmark.jl
EOL

# Make the helper script executable
chmod +x run_benchmarks.sh

echo "OPAL benchmark suite created successfully!"
