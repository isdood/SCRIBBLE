const std = @import("std");
const bragg_cache = @import("bragg_cache");
const mathplz = @import("mathplz");

const BENCH_ITERATIONS = 100;  // Increased iterations
const VECTORS_PER_ITERATION = 100_000;  // Increased vector count
const WARMUP_ITERATIONS = 5;

pub fn main() !void {
    // Initialize random number generator
    var prng = std.rand.DefaultPrng.init(0x12345678);
    const random = prng.random();

    // Create allocator
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Pre-generate vectors to remove random generation from timing
    const vectors = try allocator.alloc(bragg_cache.Vector3D, VECTORS_PER_ITERATION);
    defer allocator.free(vectors);

    // Initialize vectors with random values
    for (vectors) |*vec| {
        vec.* = .{
            .x = random.float(f64),
            .y = random.float(f64),
            .z = random.float(f64),
        };
    }

    // Initialize results array for statistical analysis
    const times = try allocator.alloc(u64, BENCH_ITERATIONS);
    defer allocator.free(times);

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

    var timer = try std.time.Timer.start();

    // Benchmark phase
    for (times) |*time| {
        var cache = try bragg_cache.BraggCache.init(allocator);
        defer cache.deinit();

        timer.reset();
        for (vectors) |vec| {
            try cache.processVector(vec);
        }
        time.* = timer.lap();
    }

    // Calculate statistics
    var min_time: u64 = times[0];
    var max_time: u64 = times[0];
    var sum_time: u64 = 0;

    for (times) |t| {
        min_time = @min(min_time, t);
        max_time = @max(max_time, t);
        sum_time += t;
    }

    const avg_time_ns = @as(f64, @floatFromInt(sum_time)) / @as(f64, BENCH_ITERATIONS);
    const min_time_ns = @as(f64, @floatFromInt(min_time));
    const max_time_ns = @as(f64, @floatFromInt(max_time));

    const time_per_vector_ns = min_time_ns / @as(f64, VECTORS_PER_ITERATION);
    const vectors_per_second = 1_000_000_000.0 / time_per_vector_ns;

    // Print results
    const stdout = std.io.getStdOut().writer();
    try stdout.print("\nBragg Cache Performance Metrics\n", .{});
    try stdout.print("────────────────────────────────────────────────\n", .{});
    try stdout.print("Configuration:\n", .{});
    try stdout.print("  Vectors per iteration: {:>12}\n", .{VECTORS_PER_ITERATION});
    try stdout.print("  Number of iterations: {:>12}\n", .{BENCH_ITERATIONS});
    try stdout.print("  Warmup iterations:    {:>12}\n", .{WARMUP_ITERATIONS});
    try stdout.print("\nTiming Results:\n", .{});
    try stdout.print("  Best time:    {:>12.2} ms\n", .{min_time_ns / 1_000_000.0});
    try stdout.print("  Average time: {:>12.2} ms\n", .{avg_time_ns / 1_000_000.0});
    try stdout.print("  Worst time:   {:>12.2} ms\n", .{max_time_ns / 1_000_000.0});
    try stdout.print("\nPerformance:\n", .{});
    try stdout.print("  Time per vector: {:>12.2} ns\n", .{time_per_vector_ns});
    try stdout.print("  Throughput:      {:>12.2} M vectors/sec\n\n", .{vectors_per_second / 1_000_000.0});
}
