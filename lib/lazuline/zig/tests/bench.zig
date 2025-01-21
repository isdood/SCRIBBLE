const std = @import("std");
const testing = std.testing;
const Timer = std.time.Timer;

test "benchmark resonator throughput" {
    const allocator = testing.allocator;
    var timer = try Timer.start();

    const iterations = 1_000_000;
    var resonator = try Resonator.init(allocator);
    defer resonator.deinit();

    const start = timer.lap();
    try resonator.process(iterations);
    const end = timer.read();

    const ns_per_op = @intToFloat(f64, end - start) / @intToFloat(f64, iterations);
    std.debug.print("\nResonator throughput: {d:.2} ns/op\n", .{ns_per_op});
}
