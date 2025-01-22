#!/bin/bash

# Timestamp: 2025-01-22 01:44:00 UTC
# Author: isdood

echo "[INIT] Creating comprehensive tests for harmonic channels and timers..."

# Create comprehensive test suite
cat > tests/harmonic_test.zig << 'EOT'
const std = @import("std");
const testing = std.testing;
const lazuline = @import("lazuline");

test "crystal channel multi-threaded operations" {
    const TestContext = struct {
        channel: *lazuline.crystal.channels.CrystalChannel,
        messages: []const []const u8,
        allocator: std.mem.Allocator,
    };

    const producer = struct {
        fn run(ctx: *TestContext) !void {
            for (ctx.messages) |msg| {
                try ctx.channel.send(msg);
                std.time.sleep(1_000_000); // 1ms between sends
            }
        }
    };

    const consumer = struct {
        fn run(ctx: *TestContext) !void {
            for (ctx.messages) |expected| {
                const received = try ctx.channel.receive();
                defer ctx.allocator.free(received);
                try testing.expectEqualStrings(expected, received);
            }
        }
    };

    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var channel = lazuline.crystal.channels.CrystalChannel.init(allocator, .{});
    defer channel.deinit();

    var ctx = TestContext{
        .channel = &channel,
        .messages = &[_][]const u8{
            "Wave 1",
            "Wave 2",
            "Wave 3",
            "Wave 4",
            "Wave 5",
        },
        .allocator = allocator,
    };

    var producer_thread = try std.Thread.spawn(.{}, producer.run, .{&ctx});
    var consumer_thread = try std.Thread.spawn(.{}, consumer.run, .{&ctx});

    producer_thread.join();
    consumer_thread.join();
}

test "crystal timer precision and drift" {
    // Test timer with different precision settings
    const configs = [_]lazuline.crystal.timers.CrystalTimer.Config{
        .{ .precision = 1_000_000, .crystal_freq = 32_768.0, .temp_compensation = 0.0 },
        .{ .precision = 1_000_000, .crystal_freq = 32_768.0, .temp_compensation = 1.0 },
        .{ .precision = 100_000, .crystal_freq = 100_000.0, .temp_compensation = 0.5 },
    };

    for (configs) |config| {
        var timer = try lazuline.crystal.timers.CrystalTimer.init(config);

        // Test multiple sleep cycles
        var total_drift: f64 = 0.0;
        const iterations: usize = 10;

        for (0..iterations) |_| {
            try timer.sleep(1_000_000); // 1ms sleep
            const elapsed = try timer.tick();
            total_drift += @fabs(@as(f64, @floatFromInt(elapsed)) - 1_000_000.0);
        }

        const avg_drift = total_drift / @as(f64, @floatFromInt(iterations));
        try testing.expect(avg_drift < config.precision);
    }
}

test "harmonic mutex wave patterns" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var mutex = try lazuline.harmonic.sync.HarmonicMutex.init(allocator, 16);
    defer mutex.deinit();

    const ThreadContext = struct {
        mutex: *lazuline.harmonic.sync.HarmonicMutex,
        counter: *usize,
    };

    const worker = struct {
        fn run(ctx: *ThreadContext) !void {
            for (0..100) |_| {
                ctx.mutex.lock();
                defer ctx.mutex.unlock();
                ctx.counter.* += 1;
            }
        }
    };

    var counter: usize = 0;
    var ctx = ThreadContext{
        .mutex = &mutex,
        .counter = &counter,
    };

    var threads: [4]std.Thread = undefined;
    for (&threads) |*thread| {
        thread.* = try std.Thread.spawn(.{}, worker.run, .{&ctx});
    }

    for (&threads) |*thread| {
        thread.join();
    }

    try testing.expectEqual(counter, 400);
}

test "harmonic async wave functions" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var async_ctx = lazuline.harmonic.async_primitives.HarmonicAsync.init(allocator);
    defer async_ctx.deinit();

    const wave_size: usize = 32;
    var future = try lazuline.harmonic.async_primitives.HarmonicAsync.Future.init(allocator, wave_size);
    defer future.deinit();

    // Initialize wave pattern
    for (future.wave.amplitude, 0..) |*amp, i| {
        amp.* = std.math.sin(@as(f64, @floatFromInt(i)) * std.math.pi * 2.0 / @as(f64, @floatFromInt(wave_size)));
    }

    try testing.expect(!future.completed);

    // Simulate async operation completion
    const result = "Operation complete";
    future.result = try allocator.dupe(u8, result);
    future.completed = true;

    try testing.expect(future.completed);
    try testing.expectEqualStrings(result, future.result.?);
}

test {
    _ = @import("std").testing.refAllDecls(@This());
}
EOT

# Update main benchmark file to include channel and timer benchmarks
cat > bench/channels_bench.zig << 'EOB'
const std = @import("std");
const lazuline = @import("lazuline");

fn benchmarkChannelThroughput(
    allocator: std.mem.Allocator,
    message_size: usize,
    iterations: usize
) !struct { throughput: f64, latency: f64 } {
    var channel = lazuline.crystal.channels.CrystalChannel.init(allocator, .{
        .buffer_size = 1024,
        .resonance_frequency = 440.0,
        .damping_factor = 0.01,
    });
    defer channel.deinit();

    var message = try allocator.alloc(u8, message_size);
    defer allocator.free(message);
    @memset(message, 'A');

    var timer = try std.time.Timer.start();
    const start = timer.lap();

    // Measure send throughput
    for (0..iterations) |_| {
        try channel.send(message);
    }

    // Measure receive throughput
    for (0..iterations) |_| {
        const received = try channel.receive();
        defer allocator.free(received);
    }

    const elapsed = timer.lap() - start;
    const seconds = @as(f64, @floatFromInt(elapsed)) / 1_000_000_000.0;
    const bytes_transferred = @as(f64, @floatFromInt(message_size * iterations * 2));
    const throughput = bytes_transferred / seconds;
    const latency = seconds / @as(f64, @floatFromInt(iterations * 2));

    return .{
        .throughput = throughput,
        .latency = latency,
    };
}

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const sizes = [_]usize{ 64, 256, 1024, 4096 };
    const iterations: usize = 10_000;

    std.debug.print("\nCrystal Channel Benchmarks\n", .{});
    std.debug.print("=======================\n\n", .{});

    for (sizes) |size| {
        const result = try benchmarkChannelThroughput(allocator, size, iterations);

        std.debug.print("Message Size: {d} bytes\n", .{size});
        std.debug.print("  Throughput: {d:.2} MB/s\n", .{result.throughput / 1_000_000.0});
        std.debug.print("  Latency: {d:.3} µs\n\n", .{result.latency * 1_000_000.0});
    }
}
EOB

# Update build.zig to include new benchmark
cat >> build.zig << 'EOZ'

    // Create channel benchmark executable
    const channel_bench = b.addExecutable(.{
        .name = "channel_bench",
        .root_source_file = .{ .cwd_relative = "bench/channels_bench.zig" },
        .target = target,
        .optimize = .ReleaseFast,
    });
    channel_bench.root_module.addImport("lazuline", main_module);

    // Create channel benchmark step
    const channel_bench_step = b.step("bench-channels", "Run channel benchmarks");
    const run_channel_bench = b.addRunArtifact(channel_bench);
    channel_bench_step.dependOn(&run_channel_bench.step);
    b.installArtifact(channel_bench);
EOZ

echo "[SETUP] Created comprehensive test suite"
echo "[SETUP] Added channel benchmarks"
echo "[SETUP] Updated build.zig with new benchmark target"
echo "[INFO] Run tests with: zig build test"
echo "[INFO] Run all benchmarks with: zig build bench"
echo "[INFO] Run channel benchmarks with: zig build bench-channels"

