const std = @import("std");

pub fn build(b: *std.build.Builder) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const opal = b.addStaticLibrary(.{
        .name = "opal",
        .root_source_file = .{ .path = "src/zig/core/main.zig" },
        .target = target,
        .optimize = optimize,
    });

    // Harmony (previously quantum) system bindings
    const harmony_module = b.addModule("harmony", .{
        .source_file = .{ .path = "src/zig/core/harmony.zig" },
    });

    // Crystal lattice optimization
    const crystal_module = b.addModule("crystal", .{
        .source_file = .{ .path = "src/zig/core/crystal.zig" },
    });

    b.installArtifact(opal);
}
