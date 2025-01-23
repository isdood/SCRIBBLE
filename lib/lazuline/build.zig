const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Create main library
    const lib = b.addStaticLibrary(.{
        .name = "lazuline",
        .root_source_file = .{ .cwd_relative = "src/lib.zig" },
        .target = target,
        .optimize = optimize,
    });
    b.installArtifact(lib);

    // Create module for others to import
    const module = b.createModule(.{
        .root_source_file = .{ .cwd_relative = "src/lib.zig" },
    });

    // Add library tests
    const lib_tests = b.addTest(.{
        .root_source_file = .{ .cwd_relative = "src/lib.zig" },
        .target = target,
        .optimize = optimize,
    });

    // Create test step
    const test_step = b.step("test", "Run library tests");
    test_step.dependOn(&b.addRunArtifact(lib_tests).step);

    // Create benchmark executable
    const bench = b.addExecutable(.{
        .name = "lazuline-bench",
        .root_source_file = .{ .cwd_relative = "bench/main.zig" },
        .target = target,
        .optimize = .ReleaseFast,  // Use maximum optimization for benchmarks
    });
    bench.root_module.addImport("lazuline", module);

    // Create benchmark installation
    const install_bench = b.addInstallArtifact(bench, .{});

    // Add benchmark step
    const bench_step = b.step("bench", "Run performance benchmarks");
    const run_bench = b.addRunArtifact(bench);
    bench_step.dependOn(&install_bench.step);
    bench_step.dependOn(&run_bench.step);

    // Optional: Add example executable
    const example = b.addExecutable(.{
        .name = "lazuline-example",
        .root_source_file = .{ .cwd_relative = "examples/main.zig" },
        .target = target,
        .optimize = optimize,
    });
    example.root_module.addImport("lazuline", module);

    const install_example = b.addInstallArtifact(example, .{});
    const example_step = b.step("example", "Build example");
    example_step.dependOn(&install_example.step);
}
