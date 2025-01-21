///! Chomp Main Module
///! ==============
///! Author: isdood
///! Created: 2025-01-21 03:17:33 UTC
///! License: MIT

const std = @import("std");

pub const Bridge = @import("bridge.zig").Bridge;
pub const Safety = @import("safety.zig").Safety;
pub const Types = @import("types.zig").Types;

pub const ChompError = error{
    BridgeError,
    SafetyError,
    TypeError,
    RustError,
};

pub const Config = struct {
    safety_level: Safety.Level = .strict,
    enable_runtime_checks: bool = true,
};

pub fn init(allocator: std.mem.Allocator, config: Config) !void {
    _ = allocator;
    _ = config;
    // TODO: Implement initialization
}

test "basic add functionality" {
    try std.testing.expectEqual(@as(i32, 42), 42);
}
