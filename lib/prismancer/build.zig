const std = @import("std");

pub fn build(b: *std.build.Builder) void {
    // Standard release options
    const mode = b.standardReleaseOptions();
    const target = b.standardTargetOptions(.{});

    // Library
    const lib = b.addStaticLibrary("prismancer-zig", "src/low_level/main.zig");
    lib.setBuildMode(mode);
    lib.setTarget(target);
    lib.install();
}
