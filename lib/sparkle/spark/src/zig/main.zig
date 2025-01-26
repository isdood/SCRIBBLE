const std = @import("std");

const c = struct {
    const safety_bridge = @cImport({
        @cInclude("safety_bridge.h");
    });
};

pub fn main() !void {
    const result = c.safety_bridge.init_safety_bridge();
    if (result != 0) {
        std.debug.print("Failed to initialize safety bridge\n", .{});
        return error.InitFailed;
    }
    std.debug.print("ZigRust Safety Bridge initialized successfully!\n", .{});

    // Test different safety levels
    const test_data = [_]u8{ 1, 2, 3, 4, 5 };

    inline for ([_]struct { name: []const u8, level: i32 }{
        .{ .name = "Calm", .level = c.safety_bridge.SAFETY_LEVEL_CALM },
        .{ .name = "Balanced", .level = c.safety_bridge.SAFETY_LEVEL_BALANCED },
        .{ .name = "Wild", .level = c.safety_bridge.SAFETY_LEVEL_WILD },
    }) |config| {
        const check_result = c.safety_bridge.check_safety(
            &test_data,
            test_data.len,
            config.level,
            true,
            true
        );
        std.debug.print("Safety check ({s}): {d}\n", .{ config.name, check_result });
    }

    const stats = c.safety_bridge.get_safety_stats();
    std.debug.print("Total checks performed: {d}\n", .{stats});
}
