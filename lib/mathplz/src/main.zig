const std = @import("std");

pub fn main() !void {
    std.debug.print("MathPLZ - Scientific Computing Library\n", .{});
}

test "basic test" {
    try std.testing.expectEqual(1, 1);
}
