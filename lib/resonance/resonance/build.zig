const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const lib = b.addStaticLibrary(.{
        .name = "resonance",
        .root_source_file = .{ .path = "src/zig/main.zig" },
        .target = target,
        .optimize = optimize,
    });

    // Julia integration
    const julia_include_path = "/usr/include/julia";
    const julia_lib_path = "/usr/lib";

    lib.addIncludePath(.{ .path = julia_include_path });
    lib.addLibraryPath(.{ .path = julia_lib_path });
    lib.linkSystemLibrary("julia");

    // Add custom build step for Julia files
    const julia_step = b.addSystemCommand(&[_][]const u8{
        "julia",
        "--project=src/julia",
        "-e",
        "using Pkg; Pkg.instantiate()",
    });

    b.getInstallStep().dependOn(&julia_step.step);
    b.installArtifact(lib);
}
