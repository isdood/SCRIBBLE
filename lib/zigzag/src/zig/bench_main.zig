const std = @import("std");
const Vector3D = @import("vector/vector3d.zig").Vector3D;
const print = std.debug.print;

pub fn main() !void {
    // Warm up the CPU
    var warmup: u64 = 0;
    for (0..1000000) |_| {
        warmup += 1;
    }
    // Use warmup value in a way that won't be optimized out
    if (warmup < 1000000) {
        print("Warmup failed\n", .{});
        return;
    }

    const iterations: u64 = 10_000_000;
    var timer = try std.time.Timer.start();

    // Setup test vectors
    const v1 = Vector3D.init(1.0, 2.0, 3.0);
    const v2 = Vector3D.init(4.0, 5.0, 6.0);
    var result: f64 = 0.0;

    // Benchmark dot product
    timer.reset();
    for (0..iterations) |_| {
        result += v1.dot(v2);
    }
    const dot_time = @as(f64, @floatFromInt(timer.lap())) / @as(f64, @floatFromInt(std.time.ns_per_s));

    // Benchmark magnitude
    timer.reset();
    for (0..iterations) |_| {
        result += v1.magnitude();
    }
    const mag_time = @as(f64, @floatFromInt(timer.lap())) / @as(f64, @floatFromInt(std.time.ns_per_s));

    // Print results (use result to prevent optimization)
    if (result == 0.0) {
        print("Unexpected zero result\n", .{});
    }

    print(
        \\
        \\Zig Benchmark Results ({d} iterations):
        \\----------------------------------------
        \\Dot Product: {d:.9}s ({d:.3} ns/iter)
        \\Magnitude:   {d:.9}s ({d:.3} ns/iter)
        \\
        \\
    , .{
        iterations,
        dot_time,
        dot_time * 1e9 / @as(f64, @floatFromInt(iterations)),
        mag_time,
        mag_time * 1e9 / @as(f64, @floatFromInt(iterations)),
    });
}
