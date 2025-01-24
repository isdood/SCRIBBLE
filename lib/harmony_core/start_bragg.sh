#!/bin/bash
# Created: 2025-01-24 03:36:25
# Author: isdood

cat > benches/zig/bench_verify.zig << 'EOL'
const std = @import("std");
const bragg_cache = @import("bragg_cache");

// Reduced workload for debugging
const BENCH_ITERATIONS = 10;
const VECTORS_PER_ITERATION = 1000;
const WARMUP_ITERATIONS = 2;
const VALIDATION_FREQUENCY = 2;

const BenchmarkError = error{
    VectorGenerationFailed,
    WarmupFailed,
    BenchmarkFailed,
    ValidationFailed,
    MemoryLeak,
};

fn logError(comptime fmt: []const u8, args: anytype) void {
    std.debug.print("\nERROR: " ++ fmt ++ "\n", args);
}

fn logDebug(comptime fmt: []const u8, args: anytype) void {
    std.debug.print("DEBUG: " ++ fmt ++ "\n", args);
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{
        .safety = true,
        .never_unmap = true,
    }){};
    defer {
        const check = gpa.deinit();
        if (check == .leak) {
            logError("Memory leak detected!", .{});
        }
    }
    const allocator = gpa.allocator();

    var error_count: usize = 0;

    std.debug.print("\nStarting benchmark setup...\n", .{});

    const vectors = allocator.alloc(bragg_cache.Vector3D, VECTORS_PER_ITERATION) catch |err| {
        logError("Failed to allocate vectors: {any}", .{err});
        return BenchmarkError.VectorGenerationFailed;
    };
    defer allocator.free(vectors);

    var validation_sum: f64 = 0.0;
    var magnitude_sum: f64 = 0.0;
    var warmup_magnitude: f64 = 0.0;

    std.debug.print("Generating test vectors...\n", .{});

    for (vectors, 0..) |*vec, i| {
        const idx = @as(f64, @floatFromInt(i));
        vec.* = .{
            .x = @sin(idx * 0.01),
            .y = @cos(idx * 0.01),
            .z = @tan(idx * 0.01),
        };

        if (!vec.isValid()) {
            logError("Invalid vector at {d}: ({d}, {d}, {d})",
                .{i, vec.x, vec.y, vec.z});
            error_count += 1;
            continue;
        }

        validation_sum += vec.x + vec.y + vec.z;
        magnitude_sum += vec.magnitude();
    }

    if (error_count > 0) {
        logError("Generated {d} invalid vectors", .{error_count});
    }

    std.debug.print("\nTesting single vector processing...\n", .{});
    {
        var cache = bragg_cache.BraggCache.init(allocator) catch |err| {
            logError("Cache initialization failed: {any}", .{err});
            return BenchmarkError.WarmupFailed;
        };
        defer cache.deinit();

        const test_vec = vectors[0];
        logDebug("Processing test vector: ({d}, {d}, {d})",
            .{test_vec.x, test_vec.y, test_vec.z});

        const result = cache.processVector(test_vec) catch |err| {
            logError("Vector processing failed: {any}", .{err});
            return BenchmarkError.WarmupFailed;
        };

        std.debug.print("\nSingle vector test successful\n", .{});
        std.debug.print("Result: {d}\n", .{result});
    }

    std.debug.print("\nStarting warmup phase...\n", .{});
    {
        var cache = bragg_cache.BraggCache.init(allocator) catch |err| {
            logError("Cache initialization failed: {any}", .{err});
            return BenchmarkError.WarmupFailed;
        };
        defer cache.deinit();

        std.debug.print("Running {d} warmup iterations...\n", .{WARMUP_ITERATIONS});

        for (0..WARMUP_ITERATIONS) |iter| {
            logDebug("Warmup iteration {d}", .{iter});
            error_count = 0;

            for (vectors) |vec| {
                if (!vec.isValid()) {
                    error_count += 1;
                    continue;
                }

                const result = cache.processVector(vec) catch |err| {
                    logError("Vector processing failed in warmup: {any}", .{err});
                    return BenchmarkError.WarmupFailed;
                };
                warmup_magnitude += result;
            }

            if (error_count > 0) {
                logError("Warmup iteration {d}: {d} invalid vectors",
                    .{iter, error_count});
            }

            const stats = cache.getCacheStats();
            logDebug("Warmup iter {d} stats: stored={d}, hits={d}, misses={d}",
                .{iter, stats.stored_vectors, stats.hits, stats.misses});
        }

        std.debug.print("\nWarmup completed successfully\n", .{});
        std.debug.print("Total warmup magnitude: {d}\n", .{warmup_magnitude});
    }

    var timer = try std.time.Timer.start();
    var total_magnitude: f64 = 0.0;
    var min_time: u64 = std.math.maxInt(u64);
    var max_time: u64 = 0;
    var total_time: u64 = 0;

    std.debug.print("\nStarting main benchmark...\n", .{});

    for (0..BENCH_ITERATIONS) |iter| {
        var cache = bragg_cache.BraggCache.init(allocator) catch |err| {
            logError("Cache initialization failed: {any}", .{err});
            return BenchmarkError.BenchmarkFailed;
        };
        defer cache.deinit();

        error_count = 0;
        timer.reset();

        for (vectors) |vec| {
            if (!vec.isValid()) {
                error_count += 1;
                continue;
            }

            const result = cache.processVector(vec) catch |err| {
                logError("Vector processing failed: {any}", .{err});
                return BenchmarkError.BenchmarkFailed;
            };

            total_magnitude += result;
        }

        const lap_time = timer.lap();

        if (error_count > 0) {
            logError("Iteration {d}: {d} invalid vectors",
                .{iter, error_count});
        }

        min_time = @min(min_time, lap_time);
        max_time = @max(max_time, lap_time);
        total_time += lap_time;

        if (iter % VALIDATION_FREQUENCY == 0) {
            const stats = cache.getCacheStats();
            logDebug("Iter {d} stats: stored={d}, hits={d}, misses={d}",
                .{iter, stats.stored_vectors, stats.hits, stats.misses});
        }
    }

    std.debug.print("\nBenchmark completed successfully\n", .{});

    const mean_time = @as(f64, @floatFromInt(total_time)) / @as(f64, BENCH_ITERATIONS);
    std.debug.print("\nPerformance Results:\n", .{});
    std.debug.print("  Min time: {d:.3} ms\n",
        .{@as(f64, @floatFromInt(min_time)) / 1_000_000.0});
    std.debug.print("  Max time: {d:.3} ms\n",
        .{@as(f64, @floatFromInt(max_time)) / 1_000_000.0});
    std.debug.print("  Mean time: {d:.3} ms\n", .{mean_time / 1_000_000.0});
    std.debug.print("  Total magnitude: {d}\n", .{total_magnitude});
}
EOL

echo "Fixed benchmark with:"
echo "1. Added proper return value handling"
echo "2. Added magnitude tracking in warmup"
echo "3. Enhanced progress reporting"
echo "4. Better error diagnostics"
echo ""
echo "Key changes:"
echo "- Fixed ignored value error"
echo "- Better warmup validation"
echo "- More detailed progress tracking"
echo "- Enhanced error reporting"
echo ""
echo "Run: zig build verify"
