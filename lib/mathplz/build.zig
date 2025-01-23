const std = @import("std");
const Build = std.Build;

pub fn build(b: *Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Bio operations module
    const bio_module = b.createModule(.{
        .root_source_file = .{ .cwd_relative = "src/bio/main.zig" },
    });

    // Crystal operations module
    const crystal_module = b.createModule(.{
        .root_source_file = .{ .cwd_relative = "src/crystal/main.zig" },
    });

    // Quantum operations module
    const quantum_module = b.createModule(.{
        .root_source_file = .{ .cwd_relative = "src/quantum/main.zig" },
    });

    // Main library
    const lib = b.addStaticLibrary(.{
        .name = "mathplz",
        .root_source_file = .{ .cwd_relative = "src/main.zig" },
        .target = target,
        .optimize = optimize,
    });

    lib.root_module.addImport("bio", bio_module);
    lib.root_module.addImport("crystal", crystal_module);
    lib.root_module.addImport("quantum", quantum_module);

    // Install library
    b.installArtifact(lib);

    // Tests
    const main_tests = b.addTest(.{
        .root_source_file = .{ .cwd_relative = "src/main.zig" },
        .target = target,
        .optimize = optimize,
    });

    main_tests.root_module.addImport("bio", bio_module);
    main_tests.root_module.addImport("crystal", crystal_module);
    main_tests.root_module.addImport("quantum", quantum_module);

    const run_main_tests = b.addRunArtifact(main_tests);

    const test_step = b.step("test", "Run library tests");
    test_step.dependOn(&run_main_tests.step);
}
