///! Basic Integration Tests
///! ====================
///! Author: isdood
///! Created: 2025-01-21 03:05:35 UTC
///! License: MIT

const std = @import("std");
const testing = std.testing;
const chomp = @import("chomp");

test "basic type conversion" {
    const allocator = testing.allocator;

    var safety = try chomp.Safety.init(allocator, .strict);
    defer safety.deinit();

    const ZigType = struct {
        value: i32,
        text: []const u8,
    };

    // Register Zig type
    try safety.registerType(ZigType, "RustStruct");

    // Create test value
    const zig_value = ZigType{
        .value = 42,
        .text = "test",
    };

    // Convert to Rust type
    const rust_value = try safety.convertToRust(zig_value);

    // Convert back to Zig
    const converted_value = try safety.convertToZig(ZigType, rust_value);

    try testing.expectEqual(zig_value.value, converted_value.value);
    try testing.expectEqualStrings(zig_value.text, converted_value.text);
}

test "safety violations" {
    const allocator = testing.allocator;

    var safety = try chomp.Safety.init(allocator, .strict);
    defer safety.deinit();

    const UnsafeType = struct {
        ptr: *anyopaque,
    };

    // This should fail due to unsafe pointer
    try testing.expectError(
        error.UnsafeType,
        safety.registerType(UnsafeType, "UnsafeRustStruct")
    );
}
