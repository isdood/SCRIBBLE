const std = @import("std");
const lazuline = @import("lazuline");
const print = std.debug.print;

fn verifyResults(scalar_sum: f64, vector_sum: f64) bool {
    const diff = @abs(scalar_sum - vector_sum);
    return diff < 1e-10;
}

pub fn main() !void {
    const iterations: usize = 1000000;
    const warmup_iterations: usize = 100000;

    // Wave Pattern benchmark
    {
        const amp = 1.0;
        const freq = 440.0;
        const phase = 0.0;
        var wave = lazuline.WavePattern.new(amp, freq, phase);

        // Warmup with realistic data
        var warmup_acc: f64 = 0;
        var i: usize = 0;
        while (i < warmup_iterations) : (i += 1) {
            const t = @as(f64, @floatFromInt(i)) * 0.001;
            warmup_acc += wave.compute(t);
        }

        // Scalar benchmark
        const start_scalar = std.time.nanoTimestamp();
        var acc_scalar: f64 = 0;
        i = 0;
        while (i < iterations) : (i += 1) {
            const t = @as(f64, @floatFromInt(i)) * 0.001;
            acc_scalar += wave.compute(t);
        }
        const end_scalar = std.time.nanoTimestamp();
        const duration_scalar = @as(f64, @floatFromInt(end_scalar - start_scalar)) / @as(f64, @floatFromInt(iterations));

        // Vector benchmark with aligned data
        var times: [8]f64 align(64) = undefined;
        const start_vector = std.time.nanoTimestamp();
        var acc_vector: f64 = 0;
        i = 0;
        while (i < iterations) : (i += 8) {
            for (0..8) |j| {
                times[j] = @as(f64, @floatFromInt(i + j)) * 0.001;
            }
            const results = wave.computeVectorized(&times);
            for (results) |result| {
                acc_vector += result;
            }
        }
        const end_vector = std.time.nanoTimestamp();
        const duration_vector = @as(f64, @floatFromInt(end_vector - start_vector)) / @as(f64, @floatFromInt(iterations));

        print("Wave Pattern (Scalar): {d:.2} ns/op\n", .{duration_scalar});
        print("Wave Pattern (Vector): {d:.2} ns/op\n", .{duration_vector});
        print("Control sums: scalar={d}, vector={d}\n", .{acc_scalar, acc_vector});
        print("Speedup: {d:.2}x\n", .{duration_scalar / duration_vector});
        print("Results match: {}\n", .{verifyResults(acc_scalar, acc_vector)});
    }
}
