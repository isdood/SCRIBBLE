const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});

    // Force ReleaseFast for benchmarking
    const optimize = std.builtin.OptimizeMode.ReleaseFast;

    // Create the benchmark executable
    const bench_viz = b.addExecutable(.{
        .name = "bench_viz",
        .root_source_file = .{ .cwd_relative = "benches/zig/bench_viz.zig" },
        .target = target,
        .optimize = optimize,
    });

    // Add required dependencies
    bench_viz.root_module.addImport("bragg_cache", b.createModule(.{
        .root_source_file = .{ .cwd_relative = "src/zig/core/bragg_cache.zig" },
    }));

    bench_viz.root_module.addImport("mathplz", b.createModule(.{
        .root_source_file = .{ .cwd_relative = "src/zig/core/mathplz.zig" },
    }));

    // Add tracy conditionally
    if (b.option(bool, "tracy", "Enable Tracy profiler integration") orelse false) {
        bench_viz.root_module.addImport("tracy", b.createModule(.{
            .root_source_file = .{ .cwd_relative = "deps/tracy/tracy.zig" },
        }));
        bench_viz.defineCMacro("TRACY_ENABLE", "1");
    }

    // Create run step
    const run_viz = b.addRunArtifact(bench_viz);

    // Add viz step
    const viz_step = b.step("viz", "Run benchmark visualization");
    viz_step.dependOn(&run_viz.step);

    // Add default run step
    const run_step = b.step("run", "Run the benchmark");
    run_step.dependOn(&run_viz.step);
}
