const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Main library
    const lib = b.addStaticLibrary(.{
        .name = "scribble",
        .root_source_file = .{ .cwd_relative = "src/cache.zig" },
        .target = target,
        .optimize = optimize,
    });

    // Tests
    const main_tests = b.addTest(.{
        .root_source_file = .{ .cwd_relative = "tests/main_test.zig" },
        .target = target,
        .optimize = optimize,
    });

    const run_main_tests = b.addRunArtifact(main_tests);

    // Create a test step
    const test_step = b.step("test", "Run library tests");
    test_step.dependOn(&run_main_tests.step);

    b.installArtifact(lib);
}
