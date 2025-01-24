const std = @import("std");
const harmony = @import("harmony");

pub fn main() !void {
    const h = harmony.Harmony.init();
    const stdout = std.io.getStdOut().writer();
    try stdout.print("Harmony Field Example\n", .{});
    try stdout.print("Initial Resonance: {d}\n", .{h.resonance});
}
