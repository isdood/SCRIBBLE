const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const mathplz = b.addModule("mathplz", .{
        .root_source_file = .{ .cwd_relative = "src/zig/core/mathplz.zig" },
    });

    const bragg_module = b.addModule("bragg_cache", .{
        .root_source_file = .{ .cwd_relative = "src/zig/core/bragg_cache.zig" },
        .imports = &.{
            .{ .name = "mathplz", .module = mathplz },
        },
    });

    const bench = b.addExecutable(.{
        .name = "bench_bragg",
        .root_source_file = .{ .cwd_relative = "benches/zig/bench_bragg.zig" },
        .target = target,
        .optimize = optimize,
    });

    bench.root_module.addImport("bragg_cache", bragg_module);
    bench.root_module.addImport("mathplz", mathplz);
    
    const run_bench = b.addRunArtifact(bench);
    run_bench.stdio = .inherit; // Enable terminal control for spinner
    
    const bench_step = b.step("bench", "Run Bragg-enhanced cache benchmarks");
    bench_step.dependOn(&run_bench.step);
}
