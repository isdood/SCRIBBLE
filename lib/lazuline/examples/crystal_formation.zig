const std = @import("std");
const crystal = @import("crystal");

pub fn main() !void {
    const c = crystal.Crystal.init();
    const stdout = std.io.getStdOut().writer();
    try stdout.print("Crystal Formation Example\n", .{});
    try stdout.print("Initial Value: {d}\n", .{c.value});
}
