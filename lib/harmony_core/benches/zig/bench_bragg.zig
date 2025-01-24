const std = @import("std");
const bragg_cache = @import("bragg_cache");
const mathplz = @import("mathplz");

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    try stdout.print("Bragg Cache Benchmark Started\n", .{});

    const allocator = std.heap.page_allocator;
    var cache = try bragg_cache.BraggCache.init(allocator);
    defer cache.deinit();

    const vec = bragg_cache.Vector3D{ .x = 1.0, .y = 0.0, .z = 0.0 };
    try cache.processVector(vec);

    try stdout.print("Basic test completed.\n", .{});
}
