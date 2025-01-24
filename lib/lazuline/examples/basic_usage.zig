const std = @import("std");
const crystal = @import("crystal");
const harmony = @import("harmony");
const whimsy = @import("whimsy");

pub fn main() !void {
    // Initialize components
    const c = crystal.Crystal.init();
    const h = harmony.Harmony.init();
    const w = whimsy.Whimsy.init();

    // Use the standard output writer
    const stdout = std.io.getStdOut().writer();
    try stdout.print("Lazuline Basic Usage Example\n", .{});
    try stdout.print("Crystal Value: {d}\n", .{c.value});
    try stdout.print("Harmony Resonance: {d}\n", .{h.resonance});
    try stdout.print("Whimsy Level: {d}\n", .{w.level});
}
