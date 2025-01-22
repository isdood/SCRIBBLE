#!/bin/bash
# Crystal Runtime Error Handling Test Setup Script
# Created: 2025-01-22 00:45:52 UTC
# Author: isdood

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}Setting up Crystal Runtime error handling test suite...${NC}"

# Create error test directory
mkdir -p zig/crystal/src/tests/errors

# Create error test file
cat > zig/crystal/src/tests/errors/main.zig << 'END_ERROR_TEST'
const std = @import("std");
const crystal = @import("crystal");
const testing = std.testing;

const TestError = error{
    ResonanceOutOfBounds,
    NullPointerAccess,
    InvalidTaskLength,
};

pub fn main() !void {
    std.debug.print("\n=== Error Handling Tests ===\n", .{});

    // Test null pointer handling
    std.debug.print("\nTesting Null Pointer Handling:\n", .{});
    try testNullPointerHandling();
    std.debug.print("✓ Null pointer checks passed\n", .{});

    // Test boundary conditions
    std.debug.print("\nTesting Boundary Conditions:\n", .{});
    try testBoundaryConditions();
    std.debug.print("✓ Boundary condition checks passed\n", .{});

    // Test invalid inputs
    std.debug.print("\nTesting Invalid Inputs:\n", .{});
    try testInvalidInputs();
    std.debug.print("✓ Invalid input checks passed\n", .{});

    // Test memory handling
    std.debug.print("\nTesting Memory Handling:\n", .{});
    try testMemoryHandling();
    std.debug.print("✓ Memory handling checks passed\n", .{});

    std.debug.print("\n✨ All error handling tests passed successfully! ✨\n", .{});
}

fn testNullPointerHandling() !void {
    // Test FFI null pointer handling
    const null_core: ?*crystal.CrystalCore = null;
    crystal_core_process_task(null_core, "test".ptr, 4);

    const null_state: ?*crystal.harmony.HarmonyState = null;
    julia_harmony_process(null_state);
}

fn testBoundaryConditions() !void {
    const core = crystal_core_init() orelse return error.NullPointerAccess;
    defer std.heap.c_allocator.destroy(core);

    // Test empty task
    crystal_core_process_task(core, "".ptr, 0);
    try testing.expectEqual(@as(f64, 1.0), core.harmony_state.resonance);

    // Test large task
    const large_task = [_]u8{'x'} ** 1024;
    crystal_core_process_task(core, &large_task, large_task.len);
    try testing.expect(core.harmony_state.resonance > 0.0);
}

fn testInvalidInputs() !void {
    const core = crystal_core_init() orelse return error.NullPointerAccess;
    defer std.heap.c_allocator.destroy(core);

    // Test invalid task length
    crystal_core_process_task(core, "test".ptr, 0);
    try testing.expectEqual(@as(f64, 1.0), core.harmony_state.resonance);

    // Test mismatched length
    crystal_core_process_task(core, "test".ptr, 2);
    try testing.expect(core.harmony_state.resonance < 1.0);
}

fn testMemoryHandling() !void {
    // Test repeated allocations
    var states = std.ArrayList(?*crystal.harmony.HarmonyState).init(
        std.heap.c_allocator,
    );
    defer states.deinit();

    // Allocate multiple states
    for (0..10) |_| {
        const state = julia_harmony_init();
        if (state) |s| {
            try states.append(s);
        }
    }

    // Clean up
    for (states.items) |state| {
        if (state) |s| {
            std.heap.c_allocator.destroy(s);
        }
    }
}

// Import FFI functions
extern fn crystal_core_init() ?*crystal.CrystalCore;
extern fn crystal_core_process_task(?*crystal.CrystalCore, [*]const u8, usize) void;
extern fn julia_harmony_init() ?*crystal.harmony.HarmonyState;
extern fn julia_harmony_process(?*crystal.harmony.HarmonyState) void;

test {
    try main();
}
END_ERROR_TEST

# Create complete build.zig
cat > build.zig << 'END_BUILD'
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

    // Create test step
    const test_step = b.step("test", "Run all tests");

    // Add test runners
    const run_unit_tests = b.addRunArtifact(unit_tests);
    const run_ffi_tests = b.addRunArtifact(ffi_tests);
    const run_error_tests = b.addRunArtifact(error_tests);

    // Add dependencies to test step
    test_step.dependOn(&run_unit_tests.step);
    test_step.dependOn(&run_ffi_tests.step);
    test_step.dependOn(&run_error_tests.step);

    // Install artifacts
    b.installArtifact(lib);
    b.installArtifact(ffi);
    b.installArtifact(julia_bridge);
}
END_BUILD

echo -e "${GREEN}Crystal Runtime error handling test suite has been set up successfully!${NC}"
echo -e "${BLUE}Test instructions:${NC}"
echo "1. Run 'zig build test' to run all tests including error handling tests"
