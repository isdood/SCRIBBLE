#!/bin/bash
# Scribble Benchmark Runner
# =======================
#
# Author: Caleb J.D. Terkovics <isdood>
# Current User: isdood
# Created: 2025-01-20
# Last Updated: 2025-01-20 17:32:54 UTC
# Version: 0.1.0
# License: MIT

set -e

echo "Running Ziggy benchmarks..."
cd lib/ziggy
RUSTFLAGS="-C target-cpu=native -C opt-level=3" cargo bench
cd ../..

echo "Running MagicMath benchmarks..."
cd lib/magicmath
RUSTFLAGS="-C target-cpu=native -C opt-level=3" cargo bench
cd ../..

echo "Comparing results..."
cargo run --bin bench_compare
