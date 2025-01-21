#!/bin/bash
# setup_build.sh - Create proper Zig build file

cat > build.zig << 'END_BUILD'
const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const vector_tests = b.addTest(.{
        .root_source_file = .{ .path = "src/zig/vector/vector3d.zig" },
        .target = target,
        .optimize = optimize,
    });

    const run_tests = b.addRunArtifact(vector_tests);
    const test_step = b.step("test", "Run all tests");
    test_step.dependOn(&run_tests.step);
}
END_BUILD
