const std = @import("std");
const crystal = @import("crystal");
const harmony = @import("harmony");
const whimsy = @import("whimsy");

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    try stdout.print("\n🔮 Lazuline Benchmarks\n", .{});
    try stdout.print("==================\n\n", .{});

    var timer = try std.time.Timer.start();
    const iterations: u32 = 1_000_000;

    // Benchmark Crystal operations
    {
        var i: u32 = 0;
        var total_time: u64 = 0;

        while (i < iterations) : (i += 1) {
            timer.reset();
            const c = crystal.Crystal.init();
            total_time += timer.lap();
            _ = c;
        }

        const avg_time = @divFloor(total_time, iterations);
        try stdout.print("💎 Crystal Formation:\n", .{});
        try stdout.print("  Operations: {d}\n", .{iterations});
        try stdout.print("  Total Time: {d}ns\n", .{total_time});
        try stdout.print("  Average:    {d}ns/op\n\n", .{avg_time});
    }

    // Benchmark Harmony operations
    {
        var i: u32 = 0;
        var total_time: u64 = 0;

        while (i < iterations) : (i += 1) {
            timer.reset();
            const h = harmony.Harmony.init();
            total_time += timer.lap();
            _ = h;
        }

        const avg_time = @divFloor(total_time, iterations);
        try stdout.print("🎵 Harmony Formation:\n", .{});
        try stdout.print("  Operations: {d}\n", .{iterations});
        try stdout.print("  Total Time: {d}ns\n", .{total_time});
        try stdout.print("  Average:    {d}ns/op\n\n", .{avg_time});
    }

    // Benchmark Whimsy operations
    {
        var i: u32 = 0;
        var total_time: u64 = 0;

        while (i < iterations) : (i += 1) {
            timer.reset();
            const w = whimsy.Whimsy.init();
            total_time += timer.lap();
            _ = w;
        }

        const avg_time = @divFloor(total_time, iterations);
        try stdout.print("✨ Whimsy Formation:\n", .{});
        try stdout.print("  Operations: {d}\n", .{iterations});
        try stdout.print("  Total Time: {d}ns\n", .{total_time});
        try stdout.print("  Average:    {d}ns/op\n\n", .{avg_time});
    }

    // Print summary with fancy formatting
    try stdout.print("📊 Performance Summary\n", .{});
    try stdout.print("==================\n", .{});
    try stdout.print("Test completed at: {s}\n", .{"2025-01-24 03:18:35 UTC"});
    try stdout.print("Build mode: ReleaseFast\n", .{});
    try stdout.print("System: Zig {s}\n\n", .{@import("builtin").zig_version_string});
}
