///! Bridge Runtime Support
///! ===================
///! Author: Caleb J.D. Terkovics <isdood>
///! Created: 2025-01-21 02:52:27 UTC
///! License: MIT

const std = @import("std");
const safety = @import("../safety/mod.zig");
const types = @import("types.zig");

pub const Runtime = struct {
    bridge: *Bridge,
    allocator: std.mem.Allocator,
    exports: std.StringHashMap(FFIExport),

    pub fn init(allocator: std.mem.Allocator, safety_level: safety.Level) !*Runtime {
        var self = try allocator.create(Runtime);
        self.* = .{
            .bridge = try Bridge.init(allocator, safety_level),
            .allocator = allocator,
            .exports = std.StringHashMap(FFIExport).init(allocator),
        };
        return self;
    }

    pub fn export(self: *Runtime, name: []const u8, func: anytype) !void {
        const export = FFIExport.init(name, func);
        try self.exports.put(name, export);
    }

    pub fn wrap(self: *Runtime, func: anytype) WrapperFn(@TypeOf(func)) {
        return self.bridge.wrap(@TypeOf(func), func);
    }

    pub fn trackLifetime(self: *Runtime, ptr: *anyopaque, context: []const u8) !void {
        const lifetime = Lifetime{
            .start = std.time.milliTimestamp(),
            .end = null,
            .context = context,
        };
        try self.bridge.trackLifetime(ptr, lifetime);
    }
};

pub const RuntimeConfig = struct {
    safety_level: safety.Level = .strict,
    stack_trace: bool = true,
    lifetime_tracking: bool = true,
};

pub fn initRuntime(allocator: std.mem.Allocator, config: RuntimeConfig) !*Runtime {
    return Runtime.init(allocator, config.safety_level);
}
