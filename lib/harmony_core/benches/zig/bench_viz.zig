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

    const len = @as(f64, @floatFromInt(times.len));
    const mean = sum / len;
    var variance_sum: f64 = 0;

    for (times) |t| {
        const diff = @as(f64, @floatFromInt(t)) - mean;
        variance_sum += diff * diff;
    }

    const stddev = @sqrt(variance_sum / len);

    return .{
        .min = min,
        .max = max,
        .mean = mean,
        .stddev = stddev,
    };
}

pub fn main() !void {
    // Create allocator
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    if (tracy_enabled) tracy.?.initFrame();
    defer if (tracy_enabled) tracy.?.endFrame();

    // Pre-generate vectors
    const vectors = try allocator.alloc(bragg_cache.Vector3D, VECTORS_PER_ITERATION);
    defer allocator.free(vectors);

    var prng = std.rand.DefaultPrng.init(0x12345678);
    const random = prng.random();

    for (vectors) |*vec| {
        vec.* = .{
            .x = random.float(f64),
            .y = random.float(f64),
            .z = random.float(f64),
        };
    }

    // Warmup phase with a fresh cache
    {
        var cache = try bragg_cache.BraggCache.init(allocator);
        defer cache.deinit();

        for (0..WARMUP_ITERATIONS) |_| {
            for (vectors) |vec| {
                try cache.processVector(vec);
            }
        }
    }

    // Store all iteration times
    const times = try allocator.alloc(u64, BENCH_ITERATIONS);
    defer allocator.free(times);

    var timer = try std.time.Timer.start();

    // Benchmark phase
    for (0..BENCH_ITERATIONS) |i| {
        var cache = try bragg_cache.BraggCache.init(allocator);
        defer cache.deinit();

        timer.reset();
        for (vectors) |vec| {
            try cache.processVector(vec);
        }
        times[i] = timer.lap();

        if (tracy_enabled) {
            tracy.?.plot("Time (ns)", @as(f64, @floatFromInt(times[i])));
        }
    }

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
