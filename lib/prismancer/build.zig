const std = @import("std");
const builtin = @import("builtin");

// Build timestamp and metadata
const BUILD_TIMESTAMP = "2025-01-21 18:37:57";
const BUILD_USER = "isdood";

pub fn build(b: *std.Build) void {
    // Standard target options
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Build options
    const options = .{
        .enable_simd = b.option(bool, "simd", "Enable SIMD optimizations")
        orelse target.cpu.arch.isX86(),
        .enable_vulkan = b.option(bool, "vulkan", "Enable Vulkan support")
        orelse true,
        .enable_cache = b.option(bool, "cache", "Enable geometry cache")
        orelse true,
        .crystal_coherence = b.option(f64, "coherence", "Crystal coherence threshold")
        orelse 0.95,
    };

    // Main library
    const lib = b.addStaticLibrary(.{
        .name = "prismancer-zig",
        .root_source_file = .{ .path = "src/low_level/main.zig" },
        .target = target,
        .optimize = optimize,
    });

    // Add build options
    const opt = b.addOptions();
    opt.addOption(bool, "enable_simd", options.enable_simd);
    opt.addOption(bool, "enable_vulkan", options.enable_vulkan);
    opt.addOption(bool, "enable_cache", options.enable_cache);
    opt.addOption(f64, "crystal_coherence", options.crystal_coherence);
    opt.addOption([]const u8, "build_timestamp", BUILD_TIMESTAMP);
    opt.addOption([]const u8, "build_user", BUILD_USER);

    lib.addOptions("build_options", opt);

    // Core modules
    const modules = .{
        .memory = b.addModule("memory", .{
            .source_file = .{ .path = "src/low_level/memory.zig" },
        }),
        .geometry = b.addModule("geometry", .{
            .source_file = .{ .path = "src/low_level/geometry.zig" },
        }),
        .cache = b.addModule("cache", .{
            .source_file = .{ .path = "src/low_level/cache.zig" },
        }),
        .vulkan = b.addModule("vulkan", .{
            .source_file = .{ .path = "src/low_level/vulkan.zig" },
        }),
    };

    // Add dependencies between modules
    modules.geometry.addImport("memory", modules.memory);
    modules.cache.addImport("memory", modules.memory);
    modules.vulkan.addImport("geometry", modules.geometry);
    modules.vulkan.addImport("cache", modules.cache);

    // Add all modules to main library
    inline for (std.meta.fields(@TypeOf(modules))) |field| {
        lib.addModule(field.name, @field(modules, field.name));
    }

    // System libraries
    if (options.enable_vulkan) {
        if (target.isWindows()) {
            lib.linkSystemLibrary("vulkan-1");
        } else {
            lib.linkSystemLibrary("vulkan");
        }
    }

    // SIMD-specific configuration
    if (options.enable_simd) {
        lib.target.cpu_features_add.addFeature("sse4.1");
        lib.target.cpu_features_add.addFeature("avx2");
        if (target.cpu.arch.isX86()) {
            lib.target.cpu_features_add.addFeature("fma");
        }
    }

    // Install library and headers
    b.installArtifact(lib);
    b.installDirectory(.{
        .source_dir = "include",
        .install_dir = .header,
        .install_subdir = "prismancer",
    });

    // Tests
    const main_tests = b.addTest(.{
        .root_source_file = .{ .path = "src/low_level/main.zig" },
        .target = target,
        .optimize = optimize,
    });

    // Add test dependencies
    inline for (std.meta.fields(@TypeOf(modules))) |field| {
        main_tests.addModule(field.name, @field(modules, field.name));
    }

    const run_main_tests = b.addRunArtifact(main_tests);
    const test_step = b.step("test", "Run library tests");
    test_step.dependOn(&run_main_tests.step);

    // Component-specific tests
    const components = .{
        "memory",
        "geometry",
        "cache",
        "vulkan",
    };

    inline for (components) |component| {
        const comp_tests = b.addTest(.{
            .root_source_file = .{
                .path = b.fmt("src/low_level/{s}.zig", .{component})
            },
            .target = target,
            .optimize = optimize,
        });

        // Add dependencies for component tests
        inline for (std.meta.fields(@TypeOf(modules))) |field| {
            comp_tests.addModule(field.name, @field(modules, field.name));
        }

        const run_comp_tests = b.addRunArtifact(comp_tests);
        const comp_test_step = b.step(
            b.fmt("test-{s}", .{component}),
                                      b.fmt("Run {s} tests", .{component})
        );
        comp_test_step.dependOn(&run_comp_tests.step);
        test_step.dependOn(comp_test_step);
    }

    // Benchmarks
    const bench = b.addExecutable(.{
        .name = "benchmark",
        .root_source_file = .{ .path = "benches/low_level/main.zig" },
        .target = target,
        .optimize = .ReleaseFast,
    });

    // Add benchmark dependencies
    inline for (std.meta.fields(@TypeOf(modules))) |field| {
        bench.addModule(field.name, @field(modules, field.name));
    }

    const run_bench = b.addRunArtifact(bench);
    const bench_step = b.step("bench", "Run benchmarks");
    bench_step.dependOn(&run_bench.step);

    // Documentation
    const docs = b.addTest(.{
        .root_source_file = .{ .path = "src/low_level/main.zig" },
        .target = target,
        .optimize = optimize,
    });
    docs.emit_docs = .emit;

    const docs_step = b.step("docs", "Generate documentation");
    docs_step.dependOn(&docs.step);
}

// Build metadata
pub const BuildInfo = struct {
    timestamp: []const u8,
    user: []const u8,
    target: std.Target,
    optimize: std.builtin.OptimizeMode,
    simd_enabled: bool,
    vulkan_enabled: bool,
    cache_enabled: bool,
    crystal_coherence: f64,
};
