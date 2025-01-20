const std = @import("std");

pub fn build(b: *std.Build) !void {
    // Standard target options allows the person running `zig build` to choose
    // what target to build for. Here we do not override the defaults, which
    // means any target is allowed, and the default is native.
    const target = b.standardTargetOptions(.{});

    // Standard optimization options allow the person running `zig build` to select
    // between Debug, ReleaseSafe, ReleaseFast, and ReleaseSmall.
    const optimize = b.standardOptimizeOption(.{});

    const lib = b.addStaticLibrary(.{
        .name = "ziggy",
        .root_source_file = .{ .cwd_relative = "src/vector3d.zig" },
        .target = target,
        .optimize = optimize,
    });

    // This declares intent for the library to be installed into the standard
    // location when the user invokes the "install" step (the default step when
    // running `zig build`).
    b.installArtifact(lib);

    // Creates a step for unit testing.
    const main_tests = b.addTest(.{
        .root_source_file = .{ .cwd_relative = "src/vector3d.zig" },
        .target = target,
        .optimize = optimize,
    });

    const run_main_tests = b.addRunArtifact(main_tests);

    // This creates a build step. It will be visible in the `zig build --help` menu,
    // and can be selected like this: `zig build test`
    const test_step = b.step("test", "Run library tests");
    test_step.dependOn(&run_main_tests.step);

    // Add a custom step that prints a completion message
    const print_step = b.step("complete", "Print completion message");
    const print_cmd = b.addSystemCommand(&[_][]const u8{
        "sh", "-c",
        \\echo -e "\033[32mBuild completed successfully!\033[0m"
        \\echo "Time: 2025-01-20 16:23:55 UTC"
        \\echo "User: isdood"
        \\echo -e "Ziggy is ready to rock! 🚀\n"
    });
    print_step.dependOn(&print_cmd.step);
    b.getInstallStep().dependOn(print_step);
}
