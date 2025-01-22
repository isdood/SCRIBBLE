#!/bin/bash

echo "Running Zig benchmark..."
zig build -Drelease-fast
./zig-out/bin/zig_benchmark

echo "Running Rust benchmark..."
cargo bench

echo "Running Julia benchmark..."
julia benchmarks/julia/benchmark.jl
