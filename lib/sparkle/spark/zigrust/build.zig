const std = @import("std");

pub fn build(b: *std.build.Builder) void {
    // Standard target options allows the person running `zig build` to choose
    // what target to build for. Here we do not override the defaults, which
    // means any target is allowed, and the default is native. Other options
    // for restricting supported target set are available.
    const target = b.standardTargetOptions(.{});

    // Standard optimization options allow the person running `zig build` to select
    // between Debug, ReleaseSafe, ReleaseFast, and ReleaseSmall. Here we do not
    // set a preferred release mode, allowing the user to decide how to optimize.
    const optimize = b.standardOptimizeOption(.{});

    const exe = b.addExecutable(.{
        .name = "zigrust",
        .root_source_file = .{ .path = "src/zig/main.zig" },
        .target = target,
        .optimize = optimize,
    });

    // Link with Rust safety library
    exe.addIncludePath("src/c");
    exe.linkSystemLibrary("safety_bridge");
    exe.linkLibC();

    // Install the executable in the prefix
    b.installArtifact(exe);

    // Create a run step
    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());

    // Add it as the "run" step
    const run_step = b.step("run", "Run the safety bridge");
    run_step.dependOn(&run_cmd.step);

    // Add tests
    const tests = b.addTest(.{
        .root_source_file = .{ .path = "src/zig/main.zig" },
        .target = target,
        .optimize = optimize,
    });

    const test_step = b.step("test", "Run the tests");
    test_step.dependOn(&tests.step);
}
