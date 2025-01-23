const std = @import("std");

var global_value: u64 = 0;

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();

    var timer = try std.time.Timer.start();
    const iterations: u32 = 1000000;
    var total_time: u64 = 0;

    // Run multiple iterations for more accurate timing
    var i: u32 = 0;
    while (i < iterations) : (i += 1) {
        const start = timer.read();
        try benchmark_resonance_field();
        const end = timer.read();
        total_time += end - start;
    }

    const avg_ns = @divFloor(total_time, iterations);
    try stdout.print("Benchmark completed in {d} ns (avg over {d} iterations)\n", .{ avg_ns, iterations });

    // Prevent dead code elimination
    if (global_value != 0) {
        try stdout.print("Control sum: {d}\n", .{global_value});
    }
}

fn benchmark_resonance_field() !void {
    var prng = std.rand.DefaultPrng.init(@as(u64, @intCast(std.time.timestamp())));
    const random = prng.random();
    const value = random.float(f64) * std.math.pi;
    // Use memory barrier to prevent optimization
    @atomicStore(u64, &global_value, @as(u64, @intFromFloat(std.math.sin(value) * 1000000.0)), .seq_cst);
}
