const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const lib = b.addStaticLibrary(.{
        .name = "lazuline",
        .root_source_file = .{ .cwd_relative = "src/lib.zig" },
        .target = target,
        .optimize = optimize,
    });

    // Module definitions
    const crystal_mod = b.addModule("crystal", .{
        .root_source_file = .{ .cwd_relative = "src/crystal.zig" },
    });

    const harmony_mod = b.addModule("harmony", .{
        .root_source_file = .{ .cwd_relative = "src/harmony.zig" },
    });

    const whimsy_mod = b.addModule("whimsy", .{
        .root_source_file = .{ .cwd_relative = "src/whimsy.zig" },
    });

    // Add module dependencies
    lib.root_module.addImport("crystal", crystal_mod);
    lib.root_module.addImport("harmony", harmony_mod);
    lib.root_module.addImport("whimsy", whimsy_mod);

    // Install artifacts
    b.installArtifact(lib);

    // Tests
    const main_tests = b.addTest(.{
        .root_source_file = .{ .cwd_relative = "src/lib.zig" },
        .target = target,
        .optimize = optimize,
    });

    main_tests.root_module.addImport("crystal", crystal_mod);
    main_tests.root_module.addImport("harmony", harmony_mod);
    main_tests.root_module.addImport("whimsy", whimsy_mod);

    const run_main_tests = b.addRunArtifact(main_tests);
    const test_step = b.step("test", "Run library tests");
    test_step.dependOn(&run_main_tests.step);

    // Benchmarks with ReleaseFast
    const bench = b.addExecutable(.{
        .name = "benchmark",
        .root_source_file = .{ .cwd_relative = "bench/main.zig" },
        .target = target,
        .optimize = .ReleaseFast,
    });

    bench.root_module.addImport("crystal", crystal_mod);
    bench.root_module.addImport("harmony", harmony_mod);
    bench.root_module.addImport("whimsy", whimsy_mod);

    const run_bench = b.addRunArtifact(bench);
    const bench_step = b.step("bench", "Run benchmarks");
    bench_step.dependOn(&run_bench.step);
}
