#!/bin/bash
# Crystal Runtime FFI Test Setup Script
# Created: 2025-01-22 00:39:50 UTC
# Author: isdood

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}Setting up Crystal Runtime FFI test suite...${NC}"

# Create FFI test directory
mkdir -p zig/crystal/src/tests/ffi

# Create FFI test file
cat > zig/crystal/src/tests/ffi/main.zig << 'END_FFI_TEST'
const std = @import("std");
const crystal = @import("crystal");
const testing = std.testing;

const FFIError = error{
    NullPointer,
    InvalidState,
};

fn testFFICore() !void {
    // Test core initialization
    const core = crystal_core_init();
    if (core == null) return FFIError.NullPointer;
    defer std.heap.c_allocator.destroy(core.?);

    // Test task processing
    const task = "FFI test task";
    crystal_core_process_task(core, task.ptr, task.len);

    // Verify state through direct access
    try testing.expectEqual(@as(f64, 0.99), core.?.harmony_state.resonance);
}

fn testFFIJulia() !void {
    // Test Julia bridge initialization
    const state = julia_harmony_init();
    if (state == null) return FFIError.NullPointer;
    defer std.heap.c_allocator.destroy(state.?);

    // Test harmony processing
    julia_harmony_process(state);

    // Verify state
    try testing.expectEqual(@as(f64, 0.99), state.?.resonance);
}

pub fn main() !void {
    std.debug.print("\n=== FFI Layer Tests ===\n", .{});

    // Test Core FFI
    std.debug.print("\nTesting Core FFI:\n", .{});
    try testFFICore();
    std.debug.print("✓ Core initialization and task processing\n", .{});

    // Test Julia FFI
    std.debug.print("\nTesting Julia Bridge:\n", .{});
    try testFFIJulia();
    std.debug.print("✓ Julia bridge initialization and processing\n", .{});

    std.debug.print("\n✨ All FFI tests passed successfully! ✨\n", .{});
}

// Import FFI functions
extern fn crystal_core_init() ?*crystal.CrystalCore;
extern fn crystal_core_process_task(?*crystal.CrystalCore, [*]const u8, usize) void;
extern fn julia_harmony_init() ?*crystal.harmony.HarmonyState;
extern fn julia_harmony_process(?*crystal.harmony.HarmonyState) void;

test {
    try main();
}
END_FFI_TEST

# Update build.zig to include FFI tests
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

    const run_unit_tests = b.addRunArtifact(unit_tests);
    const run_ffi_tests = b.addRunArtifact(ffi_tests);

    const test_step = b.step("test", "Run all tests");
    test_step.dependOn(&run_unit_tests.step);
    test_step.dependOn(&run_ffi_tests.step);

    // Install artifacts
    b.installArtifact(lib);
    b.installArtifact(ffi);
    b.installArtifact(julia_bridge);
}
END_BUILD

echo -e "${GREEN}Crystal Runtime FFI test suite has been set up successfully!${NC}"
echo -e "${BLUE}Test instructions:${NC}"
echo "1. Run 'zig build test' to run all tests including FFI tests"
