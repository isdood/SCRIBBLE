//! Ziggy Build Configuration
//! ========================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-20 16:49:21 UTC
//! Version: 0.1.0
//! License: MIT

const std = @import("std");

pub fn build(b: *std.Build) !void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const lib = b.addStaticLibrary(.{
        .name = "ziggy",
        .root_source_file = .{ .cwd_relative = "src/vector3d.zig" },
        .target = target,
        .optimize = optimize,
    });

    // Make sure the library is built as a static library
    lib.linkage = .static;

    // Install the library artifact
    const install_lib = b.addInstallArtifact(lib, .{});
    b.getInstallStep().dependOn(&install_lib.step);

    // Create test step
    const main_tests = b.addTest(.{
        .root_source_file = .{ .cwd_relative = "src/vector3d.zig" },
        .target = target,
        .optimize = optimize,
    });

    const run_main_tests = b.addRunArtifact(main_tests);

    const test_step = b.step("test", "Run library tests");
    test_step.dependOn(&run_main_tests.step);

    // Add completion message
    const print_step = b.step("complete", "Print completion message");
    const print_cmd = b.addSystemCommand(&[_][]const u8{
        "sh", "-c",
        \\echo -e "\033[32mBuild completed successfully!\033[0m"
        \\echo "Time: 2025-01-20 16:49:21 UTC"
        \\echo "User: isdood"
        \\echo -e "Ziggy is ready to rock! 🚀\n"
    });
    print_step.dependOn(&print_cmd.step);
    b.getInstallStep().dependOn(print_step);
}
