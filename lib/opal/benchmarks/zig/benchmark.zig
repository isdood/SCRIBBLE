const std = @import("std");
const harmony = @import("../../src/zig/core/harmony.zig");

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();

    const start = std.time.timestamp();
    // Call the function you want to benchmark
    try benchmark_resonance_field();
    const end = std.time.timestamp();

    const elapsed = end - start;
    try stdout.print("Benchmark completed in {d} ns\n", .{ elapsed });
}

fn benchmark_resonance_field() !void {
    var field = try harmony.ResonanceField.init();
    try field.optimize();
}
