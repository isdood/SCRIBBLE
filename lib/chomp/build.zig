///! Chomp Build System
///! ===============
///! Author: isdood
///! Created: 2025-01-21 03:11:15 UTC
///! License: MIT

const std = @import("std");

pub fn build(b: *std.Build) !void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Main library
    const lib = b.addStaticLibrary(.{
        .name = "chomp",
        .root_source_file = .{ .cwd_relative = "src/main.zig" },
        .target = target,
        .optimize = optimize,
    });
    b.installArtifact(lib);

    // Build Rust components
    var rust_build = RustBuild.create(b);  // Changed to var
    try rust_build.build();

    // Test step
    const main_tests = b.addTest(.{
        .root_source_file = .{ .cwd_relative = "src/main.zig" },
        .target = target,
        .optimize = optimize,
    });

    const run_main_tests = b.addRunArtifact(main_tests);

    // Integration tests
    const integration_tests = b.addTest(.{
        .root_source_file = .{ .cwd_relative = "tests/main_test.zig" },
        .target = target,
        .optimize = optimize,
    });

    // Add the chomp module to integration tests
    integration_tests.addModule("chomp", lib.getModule("chomp"));

    const run_integration_tests = b.addRunArtifact(integration_tests);

    // Test step
    const test_step = b.step("test", "Run all tests");
    test_step.dependOn(&run_main_tests.step);
    test_step.dependOn(&run_integration_tests.step);

    // Examples
    const example_step = b.step("examples", "Build examples");

    inline for (.{
        "simple",
        "advanced",
        "safety",
    }) |example_name| {
        const example = b.addExecutable(.{
            .name = example_name,
            .root_source_file = .{ .cwd_relative = b.fmt("examples/{s}.zig", .{example_name}) },
                                        .target = target,
                                        .optimize = optimize,
        });
        example.addModule("chomp", lib.getModule("chomp"));

        const run_cmd = b.addRunArtifact(example);
        const run_step = b.step(
            b.fmt("run-{s}", .{example_name}),
                                b.fmt("Run the {s} example", .{example_name})
        );
        run_step.dependOn(&run_cmd.step);

        example_step.dependOn(&example.step);
    }
}

const RustBuild = struct {
    builder: *std.Build,

    pub fn create(builder: *std.Build) RustBuild {
        return .{ .builder = builder };
    }

    pub fn build(self: *RustBuild) !void {
        const cargo_args = [_][]const u8{
            "cargo",
            "build",
            "--release",
        };

        var child = try std.ChildProcess.init(&cargo_args, self.builder.allocator);
        defer child.deinit();

        child.cwd = "rust";
        try child.spawn();

        const result = try child.wait();
        switch (result) {
            .Exited => |code| {
                if (code != 0) {
                    return error.CargoFailed;
                }
            },
            else => return error.CargoFailed,
        }
    }
};
