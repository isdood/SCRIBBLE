const std = @import("std");
const Vector3D = @import("vector3d.zig").Vector3D;
const time = std.time;
const print = std.debug.print;

pub fn main() !void {
    // Setup
    var timer = try time.Timer.start();
    const iterations: u32 = 10_000_000;  // Increased iterations

    const v1 = Vector3D.init(1.0, 2.0, 3.0);
    const v2 = Vector3D.init(4.0, 5.0, 6.0);

    // Benchmark dot product
    timer.reset();
    var result: f64 = 0;
    var i: u32 = 0;
    while (i < iterations) : (i += 1) {
        result += v1.dot(v2);
    }
    const dot_time = @as(f64, @floatFromInt(timer.lap())) / @as(f64, @floatFromInt(time.ns_per_s));

    // Benchmark magnitude
    timer.reset();
    i = 0;
    while (i < iterations) : (i += 1) {
        result += v1.magnitude();
    }
    const mag_time = @as(f64, @floatFromInt(timer.lap())) / @as(f64, @floatFromInt(time.ns_per_s));

    // Prevent result from being optimized away
    if (result == 0) {
        print("Unexpected result\n", .{});
    }

    print(
        \\Benchmark Results ({d} iterations):
        \\Dot Product: {d:.9}s ({d:.3} ns/iter)
        \\Magnitude:   {d:.9}s ({d:.3} ns/iter)
        \\
    , .{
        iterations,
        dot_time,
        dot_time * 1e9 / @as(f64, @floatFromInt(iterations)),
        mag_time,
        mag_time * 1e9 / @as(f64, @floatFromInt(iterations)),
    });
}
