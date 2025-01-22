const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Create main module
    const main_module = b.addModule("lazuline", .{
        .root_source_file = .{ .cwd_relative = "src/lib.zig" },
    });

    // Create the library
    const lib = b.addStaticLibrary(.{
        .name = "lazuline",
        .root_source_file = .{ .cwd_relative = "src/wave_runtime.zig" },
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

    // Add module dependency to tests
    main_tests.root_module.addImport("lazuline", main_module);

    // Create test step
    const test_step = b.step("test", "Run library tests");
    const run_main_tests = b.addRunArtifact(main_tests);
    test_step.dependOn(&run_main_tests.step);

    // Create benchmark executable
    const bench = b.addExecutable(.{
        .name = "bench",
        .root_source_file = .{ .cwd_relative = "bench/main.zig" },
        .target = target,
        .optimize = .ReleaseFast,
    });

    // Add module dependency to benchmark
    bench.root_module.addImport("lazuline", main_module);

    // Create benchmark step
    const bench_step = b.step("bench", "Run benchmarks");
    const run_bench = b.addRunArtifact(bench);
    bench_step.dependOn(&run_bench.step);

    // Install benchmark binary
    b.installArtifact(bench);
}
