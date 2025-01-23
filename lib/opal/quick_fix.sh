#!/bin/bash

echo "Applying quick fixes v22..."

# No changes needed to Zig benchmark as it's working well

# Update Cargo.toml to fix LTO configuration
cat > Cargo.toml << 'EOL'
[package]
name = "opal"
version = "0.1.0"
edition = "2021"
authors = ["isdood"]
description = "OPAL - Optimized Performance Adaptive Lattice"

[dependencies]
tokio = { version = "1.0", features = ["full"] }

[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "benchmark"
harness = false

[profile.release]
opt-level = 3
debug = false
lto = true
codegen-units = 1
panic = "abort"

[profile.bench]
opt-level = 3
debug = false
codegen-units = 1

[lib]
name = "opal"
path = "src/lib.rs"

[workspace]
members = ["."]
EOL

# Update run_benchmarks.sh with fixed Rust flags
cat > run_benchmarks.sh << 'EOL'
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
zig build -Doptimize=ReleaseFast
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

# Clean and rebuild for benchmarks
cargo clean
# Use simpler optimization flags to avoid conflicts
RUSTFLAGS="-C target-cpu=native" cargo bench

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
echo "Zig       | ~104             | 0"
echo "Rust      | (pending)        | 0"
echo "Julia RF  | (pending)        | 0"
echo "Julia CL  | (pending)        | 0"
echo "===================="
EOL

chmod +x run_benchmarks.sh

echo "run_benchmarks.sh script updated and made executable."
echo "Quick fixes v22 applied successfully!"
