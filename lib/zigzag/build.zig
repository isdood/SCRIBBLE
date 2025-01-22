const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Main Crystal Runtime Library
    const lib = b.addStaticLibrary(.{
        .name = "crystal_runtime",
        .root_source_file = .{ .cwd_relative = "zig/src/main.zig" },
        .target = target,
        .optimize = optimize,
    });

    // FFI Layer
    const ffi = b.addSharedLibrary(.{
        .name = "crystal_ffi",
        .root_source_file = .{ .cwd_relative = "zig/ffi/bridge.zig" },
        .target = target,
        .optimize = optimize,
    });

    // Julia Integration
    const julia_bridge = b.addSharedLibrary(.{
        .name = "crystal_julia",
        .root_source_file = .{ .cwd_relative = "zig/ffi/julia_bridge.zig" },
        .target = target,
        .optimize = optimize,
    });

    // Add include dirs for Rust and Julia
    lib.addIncludePath(.{ .cwd_relative = "src" });
    lib.addIncludePath(.{ .cwd_relative = "julia/src" });

    // Optimizations
    lib.want_lto = true;

    // Install artifacts
    b.installArtifact(lib);
    b.installArtifact(ffi);
    b.installArtifact(julia_bridge);

    // Add linking between components
    ffi.linkLibrary(lib);
    julia_bridge.linkLibrary(lib);
}
