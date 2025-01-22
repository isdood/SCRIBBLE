//! Crystal Wave Runtime Benchmarks
//! Created: 2025-01-22 01:34:33
//! Author: isdood

const std = @import("std");
const lazuline = @import("lazuline");

fn benchmarkWaveInterference(allocator: std.mem.Allocator, size: usize, iterations: usize) !u64 {
    var wave1 = try lazuline.wave.WaveFunction.init(allocator, size);
    defer wave1.deinit();
    var wave2 = try lazuline.wave.WaveFunction.init(allocator, size);
    defer wave2.deinit();

    // Initialize waves
    for (0..size) |i| {
        wave1.amplitude[i] = @as(f64, @floatFromInt(i)) / @as(f64, @floatFromInt(size));
        wave2.amplitude[i] = 1.0 - wave1.amplitude[i];
    }

    var timer = try std.time.Timer.start();
    const start = timer.lap();

    // Benchmark interference operations
    for (0..iterations) |_| {
        wave1.interfere(&wave2);
    }

    return timer.lap() - start;
}

pub fn main() !void {
    // Setup
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    // Benchmark parameters
    const sizes = [_]usize{ 64, 256, 1024, 4096 };
    const iterations: usize = 100_000;

    // Run benchmarks
    std.debug.print("\nCrystal Wave Runtime Benchmarks\n", .{});
    std.debug.print("==========================\n\n", .{});

    for (sizes) |size| {
        const elapsed = try benchmarkWaveInterference(allocator, size, iterations);
        const ns_per_op = @as(f64, @floatFromInt(elapsed)) / @as(f64, @floatFromInt(iterations));

        std.debug.print("Wave Interference Benchmark (size={}):\n", .{size});
        std.debug.print("  Operations: {d}\n", .{iterations});
        std.debug.print("  Total time: {d}ns\n", .{elapsed});
        std.debug.print("  {d:.2} ns/op\n\n", .{ns_per_op});
    }
}
