const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});

    const shatter_module = b.addModule("shatter_cache", .{
        .root_source_file = .{ .cwd_relative = "src/zig/core/shatter_cache.zig" },
    });

    const bench = b.addExecutable(.{
        .name = "bench",
        .root_source_file = .{ .cwd_relative = "benches/zig/bench_main.zig" },
        .target = target,
        .optimize = .ReleaseFast,
    });
    bench.root_module.addImport("shatter_cache", shatter_module);
    
    const run_bench = b.addRunArtifact(bench);
    const bench_step = b.step("bench", "Run benchmarks");
    bench_step.dependOn(&run_bench.step);
}
