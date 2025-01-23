const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Main library
    const lib = b.addStaticLibrary(.{
        .name = "mathplz",
        .root_source_file = .{ .cwd_relative = "src/main.zig" },
        .target = target,
        .optimize = optimize,
    });

    // Add Rust commands as system run steps
    const rust_build = b.addSystemCommand(&[_][]const u8{
        "cargo",
        "build",
        "--release",
    });
    rust_build.setCwd(.{ .cwd_relative = "lib/rust" });

    // Benchmarking steps
    const bench_step = b.step("bench", "Run benchmark suite");
    const rust_bench = b.addSystemCommand(&[_][]const u8{
        "cargo",
        "bench",
    });
    rust_bench.setCwd(.{ .cwd_relative = "lib/rust" });
    bench_step.dependOn(&rust_bench.step);

    // Crystal lattice benchmarks
    const crystal_bench = b.step("bench-crystal", "Run crystal lattice benchmarks");
    const rust_crystal = b.addSystemCommand(&[_][]const u8{
        "cargo",
        "bench",
        "--bench",
        "crystal_bench",
    });
    rust_crystal.setCwd(.{ .cwd_relative = "lib/rust" });
    crystal_bench.dependOn(&rust_crystal.step);

    // Quantum state benchmarks
    const quantum_bench = b.step("bench-quantum", "Run quantum state benchmarks");
    const rust_quantum = b.addSystemCommand(&[_][]const u8{
        "cargo",
        "bench",
        "--bench",
        "quantum_bench",
    });
    rust_quantum.setCwd(.{ .cwd_relative = "lib/rust" });
    quantum_bench.dependOn(&rust_quantum.step);

    // DNA sequence benchmarks
    const dna_bench = b.step("bench-dna", "Run DNA sequence benchmarks");
    const rust_dna = b.addSystemCommand(&[_][]const u8{
        "cargo",
        "bench",
        "--bench",
        "dna_bench",
    });
    rust_dna.setCwd(.{ .cwd_relative = "lib/rust" });
    dna_bench.dependOn(&rust_dna.step);

    // Report generation
    const report_step = b.step("report", "Generate benchmark report");
    const report_cmd = b.addSystemCommand(&[_][]const u8{
        "bash",
        "run_benchmarks.sh",
    });
    report_cmd.setCwd(.{ .cwd_relative = "lib/rust" });
    report_step.dependOn(&report_cmd.step);

    // Unit tests
    const unit_tests = b.addTest(.{
        .root_source_file = .{ .cwd_relative = "src/main.zig" },
        .target = target,
        .optimize = optimize,
    });

    const run_unit_tests = b.addRunArtifact(unit_tests);
    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&run_unit_tests.step);

    // Make the Rust build a dependency of the library
    lib.step.dependOn(&rust_build.step);
}
