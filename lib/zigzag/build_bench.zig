const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});

    const exe = b.addExecutable(.{
        .name = "zigbench",
        .root_source_file = .{ .cwd_relative = "src/zig/bench_main.zig" },
        .target = target,
        .optimize = std.builtin.OptimizeMode.ReleaseFast,
    });

    b.installArtifact(exe);
}
