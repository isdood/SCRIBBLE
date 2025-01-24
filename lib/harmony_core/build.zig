const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = std.builtin.OptimizeMode.ReleaseFast;

    // Create the benchmark executable
    const bench_verify = b.addExecutable(.{
        .name = "bench_verify",
        .root_source_file = .{ .cwd_relative = "benches/zig/bench_verify.zig" },
        .target = target,
        .optimize = optimize,
    });

    // Add required dependencies for bench_verify
    bench_verify.root_module.addImport("bragg_cache", b.createModule(.{
        .root_source_file = .{ .cwd_relative = "src/zig/core/bragg_cache.zig" },
    }));

    bench_verify.root_module.addImport("mathplz", b.createModule(.{
        .root_source_file = .{ .cwd_relative = "src/zig/core/mathplz.zig" },
    }));

    // Create run step for bench_verify
    const run_verify = b.addRunArtifact(bench_verify);

    // Add verify step
    const verify_step = b.step("verify", "Run benchmark verification");
    verify_step.dependOn(&run_verify.step);

    // Create the visualization executable
    const bench_viz = b.addExecutable(.{
        .name = "bench_viz",
        .root_source_file = .{ .cwd_relative = "benches/zig/bench_viz.zig" },
        .target = target,
        .optimize = optimize,
    });

    // Add required dependencies for bench_viz
    bench_viz.root_module.addImport("bragg_cache", b.createModule(.{
        .root_source_file = .{ .cwd_relative = "src/zig/core/bragg_cache.zig" },
    }));

    bench_viz.root_module.addImport("mathplz", b.createModule(.{
        .root_source_file = .{ .cwd_relative = "src/zig/core/mathplz.zig" },
    }));

    // Create run step for bench_viz
    const run_viz = b.addRunArtifact(bench_viz);

    // Add viz step
    const viz_step = b.step("viz", "Run benchmark visualization");
    viz_step.dependOn(&run_viz.step);

    // Set default run step
    const run_step = b.step("run", "Run all benchmarks");
    run_step.dependOn(&run_verify.step);
    run_step.dependOn(&run_viz.step);
}
