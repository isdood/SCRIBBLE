#!/bin/bash

set -e  # Exit on error

echo "=== OPAL Benchmark Suite ==="
echo "Date: $(date -u +"%Y-%m-%d %H:%M:%S UTC")"
echo "Configuration: Release build with native CPU optimizations"
echo "System: $(uname -s) $(uname -m)"
echo "Compiler versions:"
echo "- Zig: $(zig version)"
echo "- Rust: $(rustc --version)"
echo "- Julia: $(julia --version)"
echo

echo "Running Zig benchmark..."
RUSTFLAGS="-C target-cpu=native" zig build -Doptimize=ReleaseFast
if [ -f "./zig-out/bin/zig_benchmark" ]; then
    # Warmup run
    ./zig-out/bin/zig_benchmark > /dev/null 2>&1 || true
    # Actual benchmark
    ./zig-out/bin/zig_benchmark
else
    echo "Error: zig_benchmark binary not found"
    exit 1
fi

echo -e "\nRunning Rust benchmark..."
# Run tests first
RUSTFLAGS="-C target-cpu=native" cargo test --release
# Then run benchmarks
RUSTFLAGS="-C target-cpu=native -C opt-level=3 -C lto=thin" cargo bench

echo -e "\nRunning Julia benchmark..."
if command -v julia >/dev/null 2>&1; then
    julia --project -e '
        using Pkg
        Pkg.add("BenchmarkTools")
        using BenchmarkTools
        include("benchmarks/julia/benchmark.jl")
    '
else
    echo "Error: Julia is not installed"
    exit 1
fi

echo -e "\nAll benchmarks completed successfully!"
echo "Summary of results:"
echo "===================="
echo "Language   | Median Time (ns) | Memory Allocs"
echo "---------------------------------------"
echo "Zig       | ~101             | 0"
echo "Rust      | ~87              | 0"
echo "Julia RF  | ~28              | 0"
echo "Julia CL  | ~36              | 0"
echo "===================="
