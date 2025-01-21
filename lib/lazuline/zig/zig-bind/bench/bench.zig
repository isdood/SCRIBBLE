const std = @import("std");
const lib = @import("../src/main.zig");

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();

    var timer = try std.time.Timer.start();
    const iterations: u32 = 10_000;

    var i: u32 = 0;
    while (i < iterations) : (i += 1) {
        _ = lib.lazuline_init();
    }

    const elapsed = timer.lap();
    const avg_ns = @divFloor(elapsed, iterations);

    try stdout.print("Zig Benchmark Results:\n", .{});
    try stdout.print("Total iterations: {}\n", .{iterations});
    try stdout.print("Average time: {}ns per call\n", .{avg_ns});
}
