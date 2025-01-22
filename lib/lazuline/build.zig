const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Create crystal module
    const crystal_module = b.addModule("crystal", .{
        .root_source_file = .{ .cwd_relative = "zig/crystal/src/main.zig" },
    });

    // Main Crystal Runtime Library
    const lib = b.addStaticLibrary(.{
        .name = "crystal_runtime",
        .root_source_file = .{ .cwd_relative = "zig/crystal/src/main.zig" },
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    lib.root_module.addImport("crystal", crystal_module);

    // FFI Layer
    const ffi = b.addSharedLibrary(.{
        .name = "crystal_ffi",
        .root_source_file = .{ .cwd_relative = "zig/ffi/bridge.zig" },
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    ffi.root_module.addImport("crystal", crystal_module);

    // Julia Integration
    const julia_bridge = b.addSharedLibrary(.{
        .name = "crystal_julia",
        .root_source_file = .{ .cwd_relative = "zig/ffi/julia_bridge.zig" },
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    julia_bridge.root_module.addImport("crystal", crystal_module);

    // Unit Tests
    const unit_tests = b.addTest(.{
        .root_source_file = .{ .cwd_relative = "zig/crystal/src/tests/main.zig" },
        .target = target,
        .optimize = optimize,
    });
    unit_tests.root_module.addImport("crystal", crystal_module);

    // FFI Tests
    const ffi_tests = b.addTest(.{
        .root_source_file = .{ .cwd_relative = "zig/crystal/src/tests/ffi/main.zig" },
        .target = target,
        .optimize = optimize,
    });
    ffi_tests.root_module.addImport("crystal", crystal_module);
    ffi_tests.linkLibrary(ffi);
    ffi_tests.linkLibrary(julia_bridge);

    // Error Handling Tests
    const error_tests = b.addTest(.{
        .root_source_file = .{ .cwd_relative = "zig/crystal/src/tests/errors/main.zig" },
        .target = target,
        .optimize = optimize,
    });
    error_tests.root_module.addImport("crystal", crystal_module);
    error_tests.linkLibrary(ffi);
    error_tests.linkLibrary(julia_bridge);

    // Performance Tests
    const perf_tests = b.addTest(.{
        .root_source_file = .{ .cwd_relative = "zig/crystal/src/tests/perf/main.zig" },
        .target = target,
        .optimize = optimize,
    });
    perf_tests.root_module.addImport("crystal", crystal_module);
    perf_tests.linkLibrary(ffi);
    perf_tests.linkLibrary(julia_bridge);

    // Create test step
    const test_step = b.step("test", "Run all tests");

    // Add test runners
    const run_unit_tests = b.addRunArtifact(unit_tests);
    const run_ffi_tests = b.addRunArtifact(ffi_tests);
    const run_error_tests = b.addRunArtifact(error_tests);
    const run_perf_tests = b.addRunArtifact(perf_tests);

    // Add dependencies to test step
    test_step.dependOn(&run_unit_tests.step);
    test_step.dependOn(&run_ffi_tests.step);
    test_step.dependOn(&run_error_tests.step);
    test_step.dependOn(&run_perf_tests.step);

    // Install artifacts
    b.installArtifact(lib);
    b.installArtifact(ffi);
    b.installArtifact(julia_bridge);
}
