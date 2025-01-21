///! Bridge System Tests
///! ================
///! Author: isdood
///! Created: 2025-01-21 03:05:35 UTC
///! License: MIT

const std = @import("std");
const testing = std.testing;
const chomp = @import("chomp");

test "function bridge" {
    const allocator = testing.allocator;

    var bridge = try chomp.Bridge.init(allocator);
    defer bridge.deinit();

    // Define test function
    const TestFn = struct {
        fn add(a: i32, b: i32) i32 {
            return a + b;
        }
    };

    // Register function
    try bridge.exportFunction("add", TestFn.add);

    // Call function through bridge
    const result = try bridge.call("add", .{40, 2});
    try testing.expectEqual(@as(i32, 42), result);
}

test "type bridge" {
    const allocator = testing.allocator;

    var bridge = try chomp.Bridge.init(allocator);
    defer bridge.deinit();

    const Point = struct {
        x: f32,
        y: f32,
    };

    // Register type
    try bridge.registerType(Point, "RustPoint");

    // Create instance
    const point = Point{ .x = 1.0, .y = 2.0 };

    // Convert to Rust and back
    const rust_point = try bridge.toRust(point);
    const zig_point = try bridge.toZig(Point, rust_point);

    try testing.expectEqual(point.x, zig_point.x);
    try testing.expectEqual(point.y, zig_point.y);
}
