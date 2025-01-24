const std = @import("std");
const bragg_cache = @import("bragg_cache");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var cache = try bragg_cache.BraggCache.init(allocator);
    defer cache.deinit();

    const vec = bragg_cache.Vector3D{
        .x = 1.0,
        .y = 0.0,
        .z = 0.0,
    };

    const result = try cache.processVector(vec);
    std.debug.print("\nVector3D Type: {any}\n", .{@TypeOf(vec)});
    std.debug.print("Result Type: {any}\n", .{@TypeOf(result)});
    std.debug.print("Result Value: {any}\n\n", .{result});
}
