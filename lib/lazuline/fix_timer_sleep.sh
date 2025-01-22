#!/bin/bash

# Current timestamp: 2025-01-22 01:51:46
# Author: isdood

echo "[FIX] Updating crystal timers module with correct sleep handling..."

# Fix the timer module with proper error handling
cat > src/crystal/timers/mod.zig << 'EOT'
const std = @import("std");

pub const CrystalTimer = struct {
    const Self = @This();

    pub const Config = struct {
        precision: u64 = 1_000_000, // nanoseconds
        crystal_freq: f64 = 32_768.0, // typical crystal oscillator frequency
        temp_compensation: f64 = 0.0, // temperature compensation in ppm/°C
    };

    timer: std.time.Timer,
    crystal_period: f64,
    temp_coefficient: f64,
    precision: u64,
    last_tick: u64,
    drift: f64,

    pub fn init(config: Config) !Self {
        return Self{
            .timer = try std.time.Timer.start(),
            .crystal_period = 1.0 / config.crystal_freq,
            .temp_coefficient = config.temp_compensation,
            .precision = config.precision,
            .last_tick = 0,
            .drift = 0.0,
        };
    }

    pub fn tick(self: *Self) !u64 {
        const now = self.timer.read();
        const elapsed = now - self.last_tick;
        self.last_tick = now;

        // Simulate crystal oscillator temperature effects
        self.drift += self.temp_coefficient * @as(f64, @floatFromInt(elapsed)) * 1e-12;

        return @as(u64, @intFromFloat(@as(f64, @floatFromInt(elapsed)) * (1.0 + self.drift)));
    }

    pub fn sleep(self: *Self, duration: u64) void {
        const adjusted_duration = @as(u64, @intFromFloat(
            @as(f64, @floatFromInt(duration)) / (1.0 + self.drift)
        ));
        std.time.sleep(adjusted_duration);
    }

    pub fn reset(self: *Self) void {
        self.drift = 0.0;
        self.last_tick = self.timer.read();
    }
};
EOT

# Update test file to match new sleep signature
cat > tests/harmonic_test.zig << 'EOH'
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
            timer.sleep(1_000_000); // 1ms sleep
            const elapsed = try timer.tick();
            const drift = @as(f64, @floatFromInt(elapsed)) - 1_000_000.0;
            total_drift += if (drift < 0) -drift else drift; // absolute value
        }

        const avg_drift = total_drift / @as(f64, @floatFromInt(iterations));
        try testing.expect(avg_drift < @as(f64, @floatFromInt(config.precision)));
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
EOH

echo "[FIX] Changed sleep function to return void instead of error"
echo "[FIX] Updated tests to match new sleep signature"
echo "[FIX] Updated timestamp to: 2025-01-22 01:51:46"
echo "[INFO] Try running 'zig build test' again"

