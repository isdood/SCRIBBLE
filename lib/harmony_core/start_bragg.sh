#!/bin/bash
# Bragg-Enhanced Quantum-Crystal Cache System v1.0.6
# Author: isdood
# Current UTC: 2025-01-23 22:58:26

set -euo pipefail

# ... (previous mathplz.zig and bragg_cache.zig remain the same) ...

# Update bench_bragg.zig with fixed timer handling
cat > benches/zig/bench_bragg.zig << 'EOL'
const std = @import("std");
const bragg_cache = @import("bragg_cache");

const BenchmarkResult = struct {
    avg_ns: f64,
    memory_usage: usize,
    vectors_per_sec: f64,
    hit_rate: f64,
};

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    const stdout_file = std.io.getStdOut();
    var bw = std.io.bufferedWriter(stdout_file.writer());
    const stdout = bw.writer();
    
    const vectors = [_]bragg_cache.Vector3D{
        bragg_cache.Vector3D.init(1, 0, 0),
        bragg_cache.Vector3D.init(0, 1, 0),
        bragg_cache.Vector3D.init(0, 0, 1),
        bragg_cache.Vector3D.init(1, 1, 1),
        bragg_cache.Vector3D.init(-1, 0, 0),
        bragg_cache.Vector3D.init(0, -1, 0),
        bragg_cache.Vector3D.init(0, 0, -1),
        bragg_cache.Vector3D.init(-1, -1, -1),
    };

    const iterations: usize = 10_000;
    const num_runs: usize = 10;
    const progress_update_interval: usize = 100;
    var results: [num_runs]BenchmarkResult = undefined;
    
    try stdout.print("\nBragg-Enhanced Cache Benchmark\n", .{});
    try stdout.print("Running {d} iterations × {d} runs\n", .{iterations, num_runs});
    try stdout.print("Using wavelength: {d} Å (Cu Kα)\n\n", .{bragg_cache.WAVELENGTH});
    try bw.flush();
    
    var run: usize = 0;
    while (run < num_runs) : (run += 1) {
        var cache = bragg_cache.BraggCache.init(allocator);
        defer cache.deinit();

        const start_time = std.time.milliTimestamp();
        var last_progress_time = start_time;
        
        var i: usize = 0;
        while (i < iterations) : (i += 1) {
            const current_time = std.time.milliTimestamp();
            
            // Check for timeout (5 seconds)
            if (current_time - start_time > 5000) {
                try stdout.print("\nTimeout detected in run {d}, aborting...\n", .{run + 1});
                try bw.flush();
                return error.BenchmarkTimeout;
            }

            // Update progress every 100ms
            if (current_time - last_progress_time >= 100) {
                const percent = @as(f64, @floatFromInt(i)) / @as(f64, @floatFromInt(iterations)) * 100.0;
                try stdout.print("\rRun {d:2}/{d} [{d:3.0}%] ", .{ run + 1, num_runs, percent });
                
                // Progress bar
                const progress_width = 20;
                const filled = @as(usize, @intFromFloat(percent / 100.0 * @as(f64, @floatFromInt(progress_width))));
                try stdout.print("[", .{});
                var j: usize = 0;
                while (j < progress_width) : (j += 1) {
                    if (j < filled) {
                        try stdout.print("=", .{});
                    } else if (j == filled) {
                        try stdout.print(">", .{});
                    } else {
                        try stdout.print(" ", .{});
                    }
                }
                try stdout.print("]", .{});
                try bw.flush();
                
                last_progress_time = current_time;
            }

            for (vectors) |vec| {
                _ = try cache.processVector(vec);
            }
        }
        
        const end_time = std.time.milliTimestamp();
        const elapsed_ms = @as(f64, @floatFromInt(end_time - start_time));
        const avg_ns = (elapsed_ms * 1_000_000.0) / @as(f64, @floatFromInt(iterations));
        const vectors_per_sec = @as(f64, @floatFromInt(vectors.len * iterations)) / (elapsed_ms / 1000.0);
        const hit_rate = @as(f64, @floatFromInt(cache.hit_count)) / @as(f64, @floatFromInt(cache.hit_count + cache.miss_count)) * 100;
        
        results[run] = .{
            .avg_ns = avg_ns,
            .memory_usage = cache.getMemoryUsage(),
            .vectors_per_sec = vectors_per_sec,
            .hit_rate = hit_rate,
        };
        try stdout.print(" Done!\n", .{});
        try bw.flush();
    }

    // Calculate statistics
    var total_ns: f64 = 0;
    var total_mem: usize = 0;
    var total_vps: f64 = 0;
    var total_hr: f64 = 0;
    var min_ns: f64 = std.math.inf(f64);
    var max_ns: f64 = 0;
    
    for (results) |result| {
        total_ns += result.avg_ns;
        total_mem += result.memory_usage;
        total_vps += result.vectors_per_sec;
        total_hr += result.hit_rate;
        min_ns = @min(min_ns, result.avg_ns);
        max_ns = @max(max_ns, result.avg_ns);
    }
    
    const avg_ns = total_ns / @as(f64, @floatFromInt(num_runs));
    const avg_mem = @divFloor(total_mem, num_runs);
    const avg_vps = total_vps / @as(f64, @floatFromInt(num_runs));
    const avg_hr = total_hr / @as(f64, @floatFromInt(num_runs));
    const stddev_ns = calculateStdDev(results[0..], avg_ns);
    
    try stdout.print("\nResults (averaged over {d} runs):\n", .{num_runs});
    try stdout.print("----------------------------------------\n", .{});
    try stdout.print("Average time:     {d:.2} ns/op (σ = {d:.2} ns)\n", .{avg_ns, stddev_ns});
    try stdout.print("Range:           {d:.2} - {d:.2} ns/op\n", .{min_ns, max_ns});
    try stdout.print("Memory usage:    {d} bytes\n", .{avg_mem});
    try stdout.print("Throughput:      {d:.2} vectors/sec\n", .{avg_vps});
    try stdout.print("Cache hit rate:  {d:.2}%\n", .{avg_hr});
    try stdout.print("----------------------------------------\n", .{});
    try stdout.print("\nBenchmark completed at: 2025-01-23 22:58:26\n", .{});
    try bw.flush();
}

fn calculateStdDev(results: []const BenchmarkResult, mean: f64) f64 {
    var sum_squared_diff: f64 = 0;
    for (results) |result| {
        const diff = result.avg_ns - mean;
        sum_squared_diff += diff * diff;
    }
    return @sqrt(sum_squared_diff / @as(f64, @floatFromInt(results.len)));
}
EOL

# ... (build.zig remains the same) ...

chmod +x "$0"

echo "Bragg-Enhanced Quantum-Crystal Cache system initialized!"
echo "Run benchmarks with: zig build bench"
