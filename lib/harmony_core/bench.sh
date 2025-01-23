#!/usr/bin/env bash
set -euo pipefail

echo "Running Harmony Core Benchmarks"
echo "==============================="
echo "Started: $(date -u +"%Y-%m-%d %H:%M:%S UTC")"
echo "Runner: $(whoami)"
echo

echo "Building benchmarks..."
zig build bench

echo
echo "Complete benchmark results saved to bench_results.txt"
