const std = @import("std");

pub const Executor = struct {
    pub fn init() Executor {
        return .{};
    }

    pub fn execute(self: *Executor, task: []const u8) void {
        _ = self;
        _ = task;
    }
};
