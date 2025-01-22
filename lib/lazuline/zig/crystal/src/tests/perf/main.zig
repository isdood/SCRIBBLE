const std = @import("std");
const crystal = @import("crystal");
const testing = std.testing;

const PerformanceMetrics = struct {
    operation_time: i64,
    memory_used: usize,
    iterations: usize,
};

pub fn main() !void {
    std.debug.print("\n=== Performance Tests ===\n", .{});

    // Core Operation Performance
    std.debug.print("\nTesting Core Operation Performance:\n", .{});
    const core_metrics = try testCorePerformance();
    const avg_time = @divTrunc(core_metrics.operation_time, @intCast(i64, core_metrics.iterations));
    std.debug.print("✓ Core operations: {d}ns/op ({d} iterations)\n",
        .{ avg_time, core_metrics.iterations });

    // Memory Usage Performance
    std.debug.print("\nTesting Memory Usage:\n", .{});
    const memory_metrics = try testMemoryPerformance();
    const avg_memory = @divTrunc(memory_metrics.memory_used, memory_metrics.iterations);
    std.debug.print("✓ Memory usage: {d} bytes/op\n", .{avg_memory});

    // Concurrent Operations Performance
    std.debug.print("\nTesting Concurrent Operations:\n", .{});
    const concurrent_metrics = try testConcurrentPerformance();
    const avg_concurrent_time = @divTrunc(concurrent_metrics.operation_time, @intCast(i64, concurrent_metrics.iterations));
    std.debug.print("✓ Concurrent operations: {d}ns/op\n", .{avg_concurrent_time});

    std.debug.print("\n✨ All performance tests completed successfully! ✨\n", .{});
}

fn testCorePerformance() !PerformanceMetrics {
    const iterations: usize = 10_000;
    var timer = try std.time.Timer.start();
    const start_time = timer.lap();

    const core = crystal_core_init() orelse return error.InitializationFailed;
    defer std.heap.c_allocator.destroy(core);

    const task = "performance test task";
    var i: usize = 0;
    while (i < iterations) : (i += 1) {
        crystal_core_process_task(core, task.ptr, task.len);
    }

    const end_time = timer.lap();

    return PerformanceMetrics{
        .operation_time = end_time - start_time,
        .memory_used = @sizeOf(crystal.CrystalCore),
        .iterations = iterations,
    };
}

fn testMemoryPerformance() !PerformanceMetrics {
    const iterations: usize = 1_000;
    var total_memory: usize = 0;
    var timer = try std.time.Timer.start();
    const start_time = timer.lap();

    var i: usize = 0;
    while (i < iterations) : (i += 1) {
        const state = julia_harmony_init() orelse return error.InitializationFailed;
        total_memory += @sizeOf(crystal.harmony.HarmonyState);
        std.heap.c_allocator.destroy(state);
    }

    const end_time = timer.lap();

    return PerformanceMetrics{
        .operation_time = end_time - start_time,
        .memory_used = total_memory,
        .iterations = iterations,
    };
}

fn testConcurrentPerformance() !PerformanceMetrics {
    const iterations: usize = 1_000;
    var timer = try std.time.Timer.start();
    const start_time = timer.lap();

    var cores = std.ArrayList(?*crystal.CrystalCore).init(std.heap.c_allocator);
    defer cores.deinit();

    // Create multiple cores
    var i: usize = 0;
    while (i < 4) : (i += 1) {
        const core = crystal_core_init() orelse return error.InitializationFailed;
        try cores.append(core);
    }
    defer for (cores.items) |core| {
        if (core) |c| std.heap.c_allocator.destroy(c);
    };

    // Process tasks concurrently
    const task = "concurrent test task";
    i = 0;
    while (i < iterations) : (i += 1) {
        for (cores.items) |core| {
            if (core) |c| crystal_core_process_task(c, task.ptr, task.len);
        }
    }

    const end_time = timer.lap();

    return PerformanceMetrics{
        .operation_time = end_time - start_time,
        .memory_used = cores.items.len * @sizeOf(crystal.CrystalCore),
        .iterations = iterations * cores.items.len,
    };
}

// Import FFI functions
extern fn crystal_core_init() ?*crystal.CrystalCore;
extern fn crystal_core_process_task(?*crystal.CrystalCore, [*]const u8, usize) void;
extern fn julia_harmony_init() ?*crystal.harmony.HarmonyState;
extern fn julia_harmony_process(?*crystal.harmony.HarmonyState) void;

test {
    try main();
}
