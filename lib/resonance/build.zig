const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const lib = b.addStaticLibrary(.{
        .name = "resonance",
        .root_source_file = .{ .cwd_relative = "src/zig/main.zig" },
        .target = target,
        .optimize = optimize,
    });

    b.installArtifact(lib);
}
