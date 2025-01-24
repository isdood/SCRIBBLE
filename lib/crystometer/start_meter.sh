cat > build.zig << 'EOF'
const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Core library
    const lib = b.addStaticLibrary(.{
        .name = "crystometer",
        .root_source_file = .{ .cwd_relative = "zig/core/crystal_core.zig" },
        .target = target,
        .optimize = optimize,
    });

    // Benchmark header
    const bench_header = b.addSystemCommand(&[_][]const u8{
        "sh", "-c",
        \\printf "\n\033[35m+====================================+\n" && \
        \\printf "\033[35m|     CRYSTOMETER BENCH SUITE        |\n" && \
        \\printf "\033[35m+------------------------------------+\n" && \
        \\printf "\033[35m| Time: 2025-01-24 00:42:49          |\n" && \
        \\printf "\033[35m| User: isdood                       |\n" && \
        \\printf "\033[35m| CPU:  4 cores                      |\n" && \
        \\printf "\033[35m| Mem:  15Gi                         |\n" && \
        \\printf "\033[35m+====================================+\n\n"
    });

    // Benchmark runners in correct order
    const zig_bench = b.addSystemCommand(&[_][]const u8{
        "sh", "-c",
        \\printf "\033[33m+====================================+\n" && \
        \\printf "\033[33m|          ZIG BENCH SUITE           |\n" && \
        \\printf "\033[33m+------------------------------------+\n" && \
        \\./zig/bench/run_bench.sh && \
        \\printf "\033[33m+====================================+\n\n"
    });

    const rust_bench = b.addSystemCommand(&[_][]const u8{
        "sh", "-c",
        \\printf "\033[36m+====================================+\n" && \
        \\printf "\033[36m|         RUST BENCH SUITE           |\n" && \
        \\printf "\033[36m+------------------------------------+\n" && \
        \\./rust/bench/run_bench.sh && \
        \\printf "\033[36m+====================================+\n\n"
    });

    const julia_bench = b.addSystemCommand(&[_][]const u8{
        "sh", "-c",
        \\printf "\033[34m+====================================+\n" && \
        \\printf "\033[34m|        JULIA BENCH SUITE           |\n" && \
        \\printf "\033[34m+------------------------------------+\n" && \
        \\./julia/bench/run_bench.sh && \
        \\printf "\033[34m+====================================+\n\n"
    });

    const bench_footer = b.addSystemCommand(&[_][]const u8{
        "sh", "-c",
        \\printf "\033[35m+====================================+\n" && \
        \\printf "\033[35m|      ALL BENCHMARKS COMPLETED      |\n" && \
        \\printf "\033[35m+====================================+\n\n"
    });

    // Individual benchmark steps
    const bench_step = b.step("bench", "Run Zig benchmarks only");
    bench_step.dependOn(&bench_header.step);
    bench_step.dependOn(&zig_bench.step);
    bench_step.dependOn(&bench_footer.step);

    // Combined benchmark step with correct order
    const bench_all = b.step("bench-all", "Run all benchmarks");
    bench_all.dependOn(&bench_header.step);
    zig_bench.step.dependOn(&bench_header.step);
    rust_bench.step.dependOn(&zig_bench.step);
    julia_bench.step.dependOn(&rust_bench.step);
    bench_footer.step.dependOn(&julia_bench.step);
    bench_all.dependOn(&bench_footer.step);

    b.installArtifact(lib);
}
EOF

echo "Updated timestamp to 2025-01-24 00:42:49 and fixed CPU line spacing (now 7 spaces from right margin)"
