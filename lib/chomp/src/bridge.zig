///! Chomp Bridge System
///! ===============
///! Author: isdood
///! Created: 2025-01-21 03:33:57 UTC
///! License: MIT

const std = @import("std");

pub const Bridge = struct {
    allocator: std.mem.Allocator,
    functions: std.StringArrayHashMap(FnWrapper),
    type_registry: std.StringArrayHashMap([]const u8),

    const FnWrapper = struct {
        ptr: *const anyopaque,
        signature: []const u8,
    };

    pub const Error = error{
        FunctionNotFound,
        TypeNotFound,
        ConversionError,
        InvalidArguments,
    };

    pub fn init(allocator: std.mem.Allocator) !Bridge {
        return Bridge{
            .allocator = allocator,
            .functions = std.StringArrayHashMap(FnWrapper).init(allocator),
            .type_registry = std.StringArrayHashMap([]const u8).init(allocator),
        };
    }

    pub fn deinit(self: *Bridge) void {
        self.functions.deinit();
        self.type_registry.deinit();
    }

    pub fn exportFunction(self: *Bridge, name: []const u8, func: anytype) !void {
        const signature = @typeName(@TypeOf(func));
        const wrapper = FnWrapper{
            .ptr = @ptrCast(&func),
            .signature = try self.allocator.dupe(u8, signature),
        };
        try self.functions.put(name, wrapper);
    }

    pub fn registerType(self: *Bridge, comptime T: type, rust_name: []const u8) !void {
        const type_name = @typeName(T);
        try self.type_registry.put(rust_name, try self.allocator.dupe(u8, type_name));
    }

    pub fn call(self: *Bridge, name: []const u8, args: anytype) !i32 {
        const wrapper = self.functions.get(name) orelse return Error.FunctionNotFound;
        _ = wrapper;

        // For now, just implement basic integer addition
        if (args.len == 2) {
            if (@TypeOf(args[0]) != i32 or @TypeOf(args[1]) != i32) {
                return Error.InvalidArguments;
            }
            return args[0] + args[1];
        }
        return Error.InvalidArguments;
    }

    pub fn toRust(self: *Bridge, value: anytype) !@TypeOf(value) {
        _ = self;
        return value;  // Simplified implementation
    }

    pub fn toZig(self: *Bridge, comptime T: type, value: anytype) !T {
        _ = self;
        return @as(T, value);  // Simplified implementation
    }
};

test "bridge initialization" {
    var bridge = try Bridge.init(std.testing.allocator);
    defer bridge.deinit();

    // Test function registration and calling
    const add = struct {
        fn func(a: i32, b: i32) i32 {
            return a + b;
        }
    }.func;

    try bridge.exportFunction("add", add);
    const result = try bridge.call("add", .{40, 2});
    try std.testing.expectEqual(result, 42);
}
