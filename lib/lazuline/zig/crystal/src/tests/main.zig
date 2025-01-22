const std = @import("std");
const crystal = @import("crystal");

pub fn main() !void {
    try runTests();
}

fn runTests() !void {
    // HarmonyState Tests
    {
        std.debug.print("\n=== HarmonyState Tests ===\n", .{});

        // Test initialization
        const state = crystal.harmony.HarmonyState.init();
        try std.testing.expectEqual(@as(f64, 1.0), state.resonance);
        std.debug.print("✓ Initial resonance is 1.0\n", .{});

        // Test single process
        var process_state = crystal.harmony.HarmonyState.init();
        process_state.process();
        try std.testing.expectEqual(@as(f64, 0.99), process_state.resonance);
        std.debug.print("✓ Single process updates resonance correctly\n", .{});

        // Test multiple processes
        var multi_state = crystal.harmony.HarmonyState.init();
        multi_state.process();
        multi_state.process();
        try std.testing.expectEqual(@as(f64, 0.9801), multi_state.resonance);
        std.debug.print("✓ Multiple processes compound correctly\n", .{});
    }

    // CrystalCore Tests
    {
        std.debug.print("\n=== CrystalCore Tests ===\n", .{});

        // Test initialization
        const core = crystal.CrystalCore.init();
        try std.testing.expectEqual(@as(f64, 1.0), core.harmony_state.resonance);
        std.debug.print("✓ Core initializes with correct resonance\n", .{});

        // Test empty task processing
        var empty_core = crystal.CrystalCore.init();
        empty_core.processTask("");
        try std.testing.expectEqual(@as(f64, 1.0), empty_core.harmony_state.resonance);
        std.debug.print("✓ Empty task doesn't affect resonance\n", .{});

        // Test task processing
        var task_core = crystal.CrystalCore.init();
        task_core.processTask("test task");
        try std.testing.expectEqual(@as(f64, 0.99), task_core.harmony_state.resonance);
        std.debug.print("✓ Task processing updates resonance\n", .{});

        // Test multiple task processing
        var multi_core = crystal.CrystalCore.init();
        multi_core.processTask("task1");
        multi_core.processTask("task2");
        try std.testing.expectEqual(@as(f64, 0.9801), multi_core.harmony_state.resonance);
        std.debug.print("✓ Multiple tasks compound resonance correctly\n", .{});
    }

    std.debug.print("\n✨ All tests passed successfully! ✨\n", .{});
}

test {
    try runTests();
}
