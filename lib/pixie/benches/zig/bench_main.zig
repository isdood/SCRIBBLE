const std = @import("std");
const CrystalDance = @import("pixie").CrystalDance;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const size: usize = 1000;
    var dance = try CrystalDance.init(allocator, size);
    defer dance.deinit();

    // Warm up
    try dance.dance();

    const iterations: usize = 10;
    var i: usize = 0;
    const start = std.time.nanoTimestamp();
    while (i < iterations) : (i += 1) {
        try dance.dance();
    }
    const end = std.time.nanoTimestamp();

    const elapsed_ns = @as(f64, @floatFromInt(end - start));
    const avg_ms = (elapsed_ns / @as(f64, @floatFromInt(iterations))) / 1_000_000.0;

    const stdout = std.io.getStdOut().writer();
    try stdout.print("Average time per dance: {d:.2} ms\n", .{avg_ms});
}
