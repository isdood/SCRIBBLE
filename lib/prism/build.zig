// build.zig
const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Main library
    const lib = b.addStaticLibrary(.{
        .name = "prism",
        .root_source_file = .{ .path = "src/zig/core/scheduler.zig" },
        .target = target,
        .optimize = optimize,
    });

    // Crystal pattern module
    const crystal_module = b.createModule(.{
        .source_file = .{ .path = "src/zig/crystal/lattice.zig" },
    });
    lib.addModule("crystal", crystal_module);

    // Harmony system module
    const harmony_module = b.createModule(.{
        .source_file = .{ .path = "src/zig/core/harmony.zig" },
    });
    lib.addModule("harmony", harmony_module);

    // Tests
    const main_tests = b.addTest(.{
        .root_source_file = .{ .path = "tests/zig/core_test.zig" },
        .target = target,
        .optimize = optimize,
    });

    const test_step = b.step("test", "Run library tests");
    test_step.dependOn(&main_tests.step);

    // Install
    b.installArtifact(lib);
}
