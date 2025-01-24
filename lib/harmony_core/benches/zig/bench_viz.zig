const std = @import("std");
const bragg_cache = @import("bragg_cache");
const mathplz = @import("mathplz");

const BENCH_ITERATIONS = 100;
const VECTORS_PER_ITERATION = 100_000;
const WARMUP_ITERATIONS = 5;

const tracy_enabled = @hasDecl(@import("root"), "TRACY_ENABLE");
const tracy = if (tracy_enabled) @import("tracy") else null;

const Stats = struct {
    min: f64,
    max: f64,
    mean: f64,
    stddev: f64,
};

fn calculateStats(times: []const u64) Stats {
    var min: f64 = @as(f64, @floatFromInt(times[0]));
    var max: f64 = min;
    var sum: f64 = 0;

    for (times) |t| {
        const time = @as(f64, @floatFromInt(t));
        min = @min(min, time);
        max = @max(max, time);
        sum += time;
    }

    const mean = sum / @as(f64, times.len);
    var variance_sum: f64 = 0;

    for (times) |t| {
        const diff = @as(f64, @floatFromInt(t)) - mean;
        variance_sum += diff * diff;
    }

    const stddev = @sqrt(variance_sum / @as(f64, times.len));

    return .{
        .min = min,
        .max = max,
        .mean = mean,
        .stddev = stddev,
    };
}

pub fn main() !void {
    // ... [Previous allocator and vector setup code remains the same] ...

    // Store all iteration times
    var times = try allocator.alloc(u64, BENCH_ITERATIONS);
    defer allocator.free(times);

    // Benchmark phase
    beginZone("Benchmark");
    for (times) |*time| {
        beginZone("Iteration");

        var cache = try bragg_cache.BraggCache.init(allocator);
        defer cache.deinit();

        timer.reset();
        for (vectors) |vec| {
            try cache.processVector(vec);
        }
        time.* = timer.lap();

        if (tracy_enabled) {
            tracy.?.plot("Time (ns)", @as(f64, @floatFromInt(time.*)));
        }
        endZone();
    }
    endZone();

    // Calculate statistics
    const stats = calculateStats(times);
    const time_stats = Stats{
        .min = stats.min / 1_000_000.0, // Convert to ms
        .max = stats.max / 1_000_000.0,
        .mean = stats.mean / 1_000_000.0,
        .stddev = stats.stddev / 1_000_000.0,
    };

    const best_time_ns = stats.min;
    const time_per_vector_ns = best_time_ns / @as(f64, VECTORS_PER_ITERATION);
    const vectors_per_second = 1_000_000_000.0 / time_per_vector_ns;

    // Print results with statistical analysis
    const stdout = std.io.getStdOut().writer();
    try stdout.print("\nBragg Cache Performance Metrics\n", .{});
    try stdout.print("────────────────────────────────────────────────\n", .{});
    try stdout.print("Configuration:\n", .{});
    try stdout.print("  Tracy Profiling:      {s}\n", .{if (tracy_enabled) "Enabled" else "Disabled"});
    try stdout.print("  Vectors per iteration: {:>12}\n", .{VECTORS_PER_ITERATION});
    try stdout.print("  Number of iterations: {:>12}\n", .{BENCH_ITERATIONS});
    try stdout.print("  Warmup iterations:    {:>12}\n", .{WARMUP_ITERATIONS});
    try stdout.print("\nTiming Statistics (ms):\n", .{});
    try stdout.print("  Minimum:     {:>12.3}\n", .{time_stats.min});
    try stdout.print("  Maximum:     {:>12.3}\n", .{time_stats.max});
    try stdout.print("  Mean:        {:>12.3}\n", .{time_stats.mean});
    try stdout.print("  Std Dev:     {:>12.3}\n", .{time_stats.stddev});
    try stdout.print("  Coefficient of Variation: {:>6.2}%\n", .{(time_stats.stddev / time_stats.mean) * 100.0});
    try stdout.print("\nPerformance (based on best run):\n", .{});
    try stdout.print("  Time per vector:  {:>12.3} ns\n", .{time_per_vector_ns});
    try stdout.print("  Throughput:       {:>12.2} M vectors/sec\n\n", .{vectors_per_second / 1_000_000.0});
}
