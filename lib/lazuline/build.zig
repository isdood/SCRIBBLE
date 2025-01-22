const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Create main module
    const main_module = b.addModule("lazuline", .{
        .root_source_file = .{ .cwd_relative = "src/lib.zig" },
    });

    // Create hardware module
    const hardware_module = b.addModule("hardware", .{
        .root_source_file = .{ .cwd_relative = "src/hardware/mod.zig" },
    });

    // Create the library
    const lib = b.addStaticLibrary(.{
        .name = "lazuline",
        .root_source_file = .{ .cwd_relative = "src/lib.zig" },
        .target = target,
        .optimize = optimize,
    });
    b.installArtifact(lib);

    // Create and configure tests
    const main_tests = b.addTest(.{
        .root_source_file = .{ .cwd_relative = "tests/main.zig" },
        .target = target,
        .optimize = optimize,
    });
    main_tests.root_module.addImport("lazuline", main_module);

    // Create hardware tests
    const hardware_tests = b.addTest(.{
        .root_source_file = .{ .cwd_relative = "tests/hardware_test.zig" },
        .target = target,
        .optimize = optimize,
    });
    hardware_tests.root_module.addImport("hardware", hardware_module);

    // Create test step
    const test_step = b.step("test", "Run library tests");
    const run_main_tests = b.addRunArtifact(main_tests);
    const run_hardware_tests = b.addRunArtifact(hardware_tests);
    test_step.dependOn(&run_main_tests.step);
    test_step.dependOn(&run_hardware_tests.step);

    // Create harmonic tests
    const harmonic_tests = b.addTest(.{
        .root_source_file = .{ .cwd_relative = "tests/harmonic_test.zig" },
        .target = target,
        .optimize = optimize,
    });
    harmonic_tests.root_module.addImport("lazuline", main_module);
    const run_harmonic_tests = b.addRunArtifact(harmonic_tests);
    test_step.dependOn(&run_harmonic_tests.step);

    // Create benchmark executable
    const bench = b.addExecutable(.{
        .name = "bench",
        .root_source_file = .{ .cwd_relative = "bench/main.zig" },
        .target = target,
        .optimize = .ReleaseFast,
    });
    bench.root_module.addImport("lazuline", main_module);
    bench.root_module.addImport("hardware", hardware_module);

    // Create benchmark step
    const bench_step = b.step("bench", "Run wave pattern benchmarks");
    const run_bench = b.addRunArtifact(bench);
    bench_step.dependOn(&run_bench.step);

    // Install benchmark binary
    b.installArtifact(bench);

    // Create channel benchmark executable
    const channel_bench = b.addExecutable(.{
        .name = "channel_bench",
        .root_source_file = .{ .cwd_relative = "bench/channels_bench.zig" },
        .target = target,
        .optimize = .ReleaseFast,
    });
    channel_bench.root_module.addImport("lazuline", main_module);
    channel_bench.root_module.addImport("hardware", hardware_module);

    // Create channel benchmark step
    const channel_bench_step = b.step("bench-channels", "Run channel benchmarks");
    const run_channel_bench = b.addRunArtifact(channel_bench);
    channel_bench_step.dependOn(&run_channel_bench.step);

    // Install channel benchmark binary
    b.installArtifact(channel_bench);
}
