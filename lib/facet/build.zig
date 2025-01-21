//! Facet Build Configuration
//! Author: isdood
//! Created: 2025-01-21 15:17:06 UTC

const std = @import("std");

pub fn build(b: *std.Build) void {
    // Standard target options
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Main library
    const exe = b.addStaticLibrary(.{
        .name = "facet",
        .target = target,
        .optimize = optimize,
    });

    // Add source file
    const main_path = b.pathFromRoot("src/main.zig");
    exe.addCSourceFile(.{
        .file = .{ .cwd_relative = main_path },
        .flags = &.{},
    });
    b.installArtifact(exe);

    // Tests
    const test_step = b.step("test", "Run Zig tests only");  // Changed description to clarify

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
            .root_source_file = .{ .cwd_relative = b.pathFromRoot(test_file) },
                                    .target = target,
                                    .optimize = optimize,
        });
        test_step.dependOn(&test_case.step);
    }

    // Create a separate step for Rust tests
    const rust_test_step = b.step("test-rust", "Run Rust tests");
    const rust_tests = b.addSystemCommand(&[_][]const u8{
        "cargo",
        "test",
        "--manifest-path",
        "rust/Cargo.toml",
    });
    rust_test_step.dependOn(&rust_tests.step);

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
