const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const bragg_cache = b.createModule(.{
        .root_source_file = .{ .cwd_relative = "src/zig/core/bragg_cache.zig" },
    });

    const mathplz = b.createModule(.{
        .root_source_file = .{ .cwd_relative = "src/zig/core/mathplz.zig" },
    });

    const bench = b.addExecutable(.{
        .name = "bench_bragg",
        .root_source_file = .{ .cwd_relative = "benches/zig/bench_bragg.zig" },
        .target = target,
        .optimize = optimize,
    });

    bench.root_module.addImport("bragg_cache", bragg_cache);
    bench.root_module.addImport("mathplz", mathplz);

    const run_bench = b.addRunArtifact(bench);

    const bench_step = b.step("bench", "Run the Bragg cache benchmarks");
    bench_step.dependOn(&run_bench.step);

    b.default_step.dependOn(bench_step);
}
