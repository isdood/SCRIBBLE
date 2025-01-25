const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const exe = b.addExecutable(.{
        .name = "seed",
        .root_source_file = .{ .cwd_relative = "seed.zig" },
        .target = target,
        .optimize = optimize,
    });

    b.installArtifact(exe);
}
