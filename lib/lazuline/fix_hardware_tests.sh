#!/bin/bash

# Timestamp: 2025-01-22 02:07:14
# Author: isdood

echo "[FIX] Updating hardware tests with correct math functions..."

cat > tests/hardware_test.zig << 'EOT'
const std = @import("std");
const testing = std.testing;
const hardware = @import("hardware");

test "temperature sensor simulation" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var sensor = try hardware.sensors.TemperatureSensor.init(allocator, .{
        .sensor_type = .Simulated,
        .update_interval = 100_000_000, // 100ms
    });
    defer sensor.deinit();

    // Test multiple readings
    var prev_temp: f64 = try sensor.readTemperature();
    var i: usize = 0;
    while (i < 10) : (i += 1) {
        std.time.sleep(200_000_000); // 200ms
        const temp = try sensor.readTemperature();
        try testing.expect(temp >= 20.0 and temp <= 30.0);
        const temp_diff = temp - prev_temp;
        try testing.expect(if (temp_diff < 0) -temp_diff else temp_diff < 1.0);
        prev_temp = temp;
    }
}

test "crystal frequency monitor" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var monitor = try hardware.crystal.CrystalMonitor.init(allocator, .{
        .nominal_frequency = 32_768.0,
        .measurement_period = 100_000_000, // 100ms
    });
    defer monitor.deinit();

    // Test multiple measurements
    var prev_freq: f64 = try monitor.measureFrequency();
    var i: usize = 0;
    while (i < 10) : (i += 1) {
        std.time.sleep(200_000_000); // 200ms
        const freq = try monitor.measureFrequency();
        // Check frequency is within 0.1% of nominal
        const freq_diff = freq - 32_768.0;
        try testing.expect((if (freq_diff < 0) -freq_diff else freq_diff) < 32.768);
        // Check frequency doesn't change too rapidly
        const delta = freq - prev_freq;
        try testing.expect((if (delta < 0) -delta else delta) < 1.0);
        prev_freq = freq;
    }
}
EOT

echo "[FIX] Replaced std.math.fabs with manual absolute value calculation"
echo "[FIX] Updated timestamp to: 2025-01-22 02:07:14"
echo "[INFO] Try running 'zig build test' again"

