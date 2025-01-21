///! Chomp Type System
///! =============
///! Author: isdood
///! Created: 2025-01-21 03:17:33 UTC
///! License: MIT

const std = @import("std");

pub const Types = struct {
    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator) !Types {
        return Types{
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *Types) void {
        _ = self;
    }
};

test "types initialization" {
    var types = try Types.init(std.testing.allocator);
    defer types.deinit();
}
