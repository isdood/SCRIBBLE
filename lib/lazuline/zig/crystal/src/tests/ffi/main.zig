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
