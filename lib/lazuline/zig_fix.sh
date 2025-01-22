cat > build.zig << 'EOF'
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

    // Add module dependency to benchmark
    bench.root_module.addImport("lazuline", main_module);

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

    // Create channel benchmark step
    const channel_bench_step = b.step("bench-channels", "Run channel benchmarks");
    const run_channel_bench = b.addRunArtifact(channel_bench);
    channel_bench_step.dependOn(&run_channel_bench.step);

    // Install channel benchmark binary
    b.installArtifact(channel_bench);
}
EOF

echo "[BUILD] Fixed build.zig configuration"
echo "[BUILD] Added proper test and benchmark targets"
echo "[BUILD] Current timestamp: 2025-01-22 01:46:28"
echo "[INFO] Try running the tests again with: zig build test"
