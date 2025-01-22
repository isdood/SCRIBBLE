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
