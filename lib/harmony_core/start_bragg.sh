#!/bin/bash
# Bragg-Enhanced Quantum-Crystal Cache System v1.0.49
# Created: 2025-01-24 00:45:21
# Author: isdood

set -euo pipefail

# Update build.zig with correct path syntax
cat > build.zig << 'EOL'
const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const bragg_cache = b.createModule(.{
        .root_source_file = .{ .cwd_relative = "src/zig/core/bragg_cache.zig" },
    });

    const mathplz = b.createModule(.{
        .root_source_file = .{ .cwd_relative = "src/zig/core/mathplz.zig" },
    });

    const bench = b.addExecutable(.{
        .name = "bench_bragg",
        .root_source_file = .{ .cwd_relative = "benches/zig/bench_bragg.zig" },
        .target = target,
        .optimize = optimize,
    });

    bench.addModule("bragg_cache", bragg_cache);
    bench.addModule("mathplz", mathplz);

    const run_bench = b.addRunArtifact(bench);

    const bench_step = b.step("bench", "Run the Bragg cache benchmarks");
    bench_step.dependOn(&run_bench.step);

    b.default_step.dependOn(bench_step);
}
EOL

# Rest of the file setup remains the same
mkdir -p src/zig/core benches/zig

# Previous bench_bragg.zig, bragg_cache.zig, and mathplz.zig content remains unchanged
# ...

chmod +x "$0"

echo "Bragg-Enhanced Quantum-Crystal Cache system initialized!"
echo "Run benchmarks with: zig build bench"
