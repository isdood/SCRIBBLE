//! Facet Build Configuration
//! Author: isdood
//! Created: 2025-01-21 14:03:01 UTC

const std = @import("std");

pub fn build(b: *std.Build) void {
    // Standard target options
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Main library
    const lib = b.addStaticLibrary(.{
        .name = "facet",
        .target = target,
        .optimize = optimize,
    });
    lib.addSourceFile("src/main.zig");
    b.installArtifact(lib);

    // Tests
    const test_step = b.step("test", "Run all tests");

    const test_files = &[_][]const u8{
        "src/main.zig",
        "tests/core_test.zig",
        "tests/crystal_test.zig",
        "tests/bridge_test.zig",
        "tests/quantum_test.zig",
        "tests/integration_test.zig",
    };

    for (test_files) |test_file| {
        const test_case = b.addTest(.{
            .target = target,
            .optimize = optimize,
        });
        test_case.addSourceFile(test_file);
        test_step.dependOn(&test_case.step);
    }

    // Rust tests
    const rust_tests = b.addSystemCommand(&[_][]const u8{
        "cargo",
        "test",
        "--manifest-path",
        "rust/Cargo.toml",
    });
    test_step.dependOn(&rust_tests.step);

    // Add benchmarks
    const bench_step = b.step("bench", "Run benchmarks");
    const rust_bench = b.addSystemCommand(&[_][]const u8{
        "cargo",
        "bench",
        "--manifest-path",
        "rust/Cargo.toml",
    });
    bench_step.dependOn(&rust_bench.step);
}
