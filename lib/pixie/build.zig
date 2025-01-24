const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = std.builtin.OptimizeMode.ReleaseFast;

    // Create Lazuline module (sibling directory)
    const lazuline = b.addModule("lazuline", .{
        .root_source_file = .{ .cwd_relative = "../lazuline/src/lib.zig" },
    });

    // Create core modules
    const crystal_math = b.addModule("crystal_math", .{
        .root_source_file = .{ .cwd_relative = "src/zig/core/crystal_math.zig" },
    });

    // Create Pixie module that depends on crystal_math and lazuline
    const pixie = b.addModule("pixie", .{
        .root_source_file = .{ .cwd_relative = "src/zig/core/crystal_dance.zig" },
        .imports = &.{
            .{ .name = "crystal_math", .module = crystal_math },
            .{ .name = "lazuline", .module = lazuline },
        },
    });

    // Create the Pixie core library
    const lib = b.addStaticLibrary(.{
        .name = "pixie_core",
        .root_source_file = .{ .cwd_relative = "src/zig/core/crystal_dance.zig" },
        .target = target,
        .optimize = optimize,
    });

    // Add dependencies to library
    lib.root_module.addImport("crystal_math", crystal_math);
    lib.root_module.addImport("lazuline", lazuline);

    // Benchmarks
    const bench = b.addExecutable(.{
        .name = "pixie_bench",
        .root_source_file = .{ .cwd_relative = "benches/zig/bench_main.zig" },
        .target = target,
        .optimize = optimize,
    });

    // Add dependencies to benchmark
    bench.root_module.addImport("pixie", pixie);

    const run_bench = b.addRunArtifact(bench);
    const bench_step = b.step("bench", "Run pixie benchmarks");
    bench_step.dependOn(&run_bench.step);

    b.installArtifact(lib);
}
