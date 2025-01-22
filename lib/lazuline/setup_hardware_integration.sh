#!/bin/bash

# Timestamp: 2025-01-22 02:01:59 UTC
# Author: isdood

echo "[INIT] Setting up hardware sensor integration..."

# Create hardware abstraction layer
mkdir -p src/hardware/{sensors,crystal}

# Create hardware sensor interface
cat > src/hardware/sensors/temperature.zig << 'EOT'
const std = @import("std");

pub const TemperatureSensor = struct {
    const Self = @This();

    pub const SensorType = enum {
        DS18B20,    // 1-Wire digital temperature sensor
        TMP102,     // I2C temperature sensor
        MAX31856,   // Thermocouple interface
        Simulated,  // Simulated sensor for testing
    };

    pub const Config = struct {
        sensor_type: SensorType = .Simulated,
        i2c_address: ?u8 = null,
        update_interval: u64 = 1000 * 1000 * 1000, // 1 second in nanoseconds
        samples_to_average: u32 = 10,
    };

    sensor_type: SensorType,
    last_reading: f64,
    last_update: u64,
    update_interval: u64,
    samples: std.ArrayList(f64),
    allocator: std.mem.Allocator,
    timer: std.time.Timer,

    pub fn init(allocator: std.mem.Allocator, config: Config) !Self {
        return Self{
            .sensor_type = config.sensor_type,
            .last_reading = 25.0,
            .last_update = 0,
            .update_interval = config.update_interval,
            .samples = try std.ArrayList(f64).initCapacity(allocator, config.samples_to_average),
            .allocator = allocator,
            .timer = try std.time.Timer.start(),
        };
    }

    pub fn deinit(self: *Self) void {
        self.samples.deinit();
    }

    pub fn readTemperature(self: *Self) !f64 {
        const now = self.timer.read();
        if (now - self.last_update >= self.update_interval) {
            try self.updateReading();
        }
        return self.last_reading;
    }

    fn updateReading(self: *Self) !void {
        const raw_temp = switch (self.sensor_type) {
            .DS18B20 => try self.readDS18B20(),
            .TMP102 => try self.readTMP102(),
            .MAX31856 => try self.readMAX31856(),
            .Simulated => self.simulateTemperature(),
        };

        try self.samples.append(raw_temp);
        if (self.samples.items.len > 10) {
            _ = self.samples.orderedRemove(0);
        }

        // Calculate moving average
        var sum: f64 = 0;
        for (self.samples.items) |sample| {
            sum += sample;
        }
        self.last_reading = sum / @as(f64, @floatFromInt(self.samples.items.len));
        self.last_update = self.timer.read();
    }

    fn readDS18B20(self: *Self) !f64 {
        // TODO: Implement actual DS18B20 reading
        return self.simulateTemperature();
    }

    fn readTMP102(self: *Self) !f64 {
        // TODO: Implement actual TMP102 reading
        return self.simulateTemperature();
    }

    fn readMAX31856(self: *Self) !f64 {
        // TODO: Implement actual MAX31856 reading
        return self.simulateTemperature();
    }

    fn simulateTemperature(self: *Self) f64 {
        const time = @as(f64, @floatFromInt(self.timer.read())) / 1_000_000_000.0;
        return 25.0 + 2.0 * std.math.sin(time * 0.001);
    }
};
EOT

# Create crystal frequency monitor
cat > src/hardware/crystal/frequency.zig << 'EOC'
const std = @import("std");

pub const CrystalMonitor = struct {
    const Self = @This();

    pub const Config = struct {
        nominal_frequency: f64 = 32_768.0,
        measurement_period: u64 = 1000 * 1000 * 1000, // 1 second
        samples_to_average: u32 = 10,
    };

    nominal_freq: f64,
    current_freq: f64,
    measurement_period: u64,
    samples: std.ArrayList(f64),
    allocator: std.mem.Allocator,
    timer: std.time.Timer,
    last_update: u64,

    pub fn init(allocator: std.mem.Allocator, config: Config) !Self {
        return Self{
            .nominal_freq = config.nominal_frequency,
            .current_freq = config.nominal_frequency,
            .measurement_period = config.measurement_period,
            .samples = try std.ArrayList(f64).initCapacity(allocator, config.samples_to_average),
            .allocator = allocator,
            .timer = try std.time.Timer.start(),
            .last_update = 0,
        };
    }

    pub fn deinit(self: *Self) void {
        self.samples.deinit();
    }

    pub fn measureFrequency(self: *Self) !f64 {
        const now = self.timer.read();
        if (now - self.last_update >= self.measurement_period) {
            try self.updateFrequency();
        }
        return self.current_freq;
    }

    fn updateFrequency(self: *Self) !void {
        const raw_freq = self.simulateFrequency();
        try self.samples.append(raw_freq);
        if (self.samples.items.len > 10) {
            _ = self.samples.orderedRemove(0);
        }

        // Calculate moving average
        var sum: f64 = 0;
        for (self.samples.items) |sample| {
            sum += sample;
        }
        self.current_freq = sum / @as(f64, @floatFromInt(self.samples.items.len));
        self.last_update = self.timer.read();
    }

    fn simulateFrequency(self: *Self) f64 {
        const time = @as(f64, @floatFromInt(self.timer.read())) / 1_000_000_000.0;
        // Simulate temperature-dependent frequency drift
        return self.nominal_freq * (1.0 + 0.0001 * std.math.sin(time * 0.0001));
    }
};
EOC

# Create test file for hardware components
cat > tests/hardware_test.zig << 'EOH'
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
        try testing.expect(std.math.fabs(temp - prev_temp) < 1.0);
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
        try testing.expect(std.math.fabs(freq - 32_768.0) < 32.768);
        // Check frequency doesn't change too rapidly
        try testing.expect(std.math.fabs(freq - prev_freq) < 1.0);
        prev_freq = freq;
    }
}
EOH

# Update build.zig to include hardware modules
cat >> build.zig << 'EOB'

    // Add hardware modules
    const hardware_module = b.addModule("hardware", .{
        .root_source_file = .{ .cwd_relative = "src/hardware/mod.zig" },
    });

    // Add hardware tests
    const hardware_tests = b.addTest(.{
        .root_source_file = .{ .cwd_relative = "tests/hardware_test.zig" },
        .target = target,
        .optimize = optimize,
    });
    hardware_tests.root_module.addImport("hardware", hardware_module);
    const run_hardware_tests = b.addRunArtifact(hardware_tests);
    test_step.dependOn(&run_hardware_tests.step);
EOB

# Create hardware module entry point
cat > src/hardware/mod.zig << 'EOM'
pub const sensors = @import("sensors/temperature.zig");
pub const crystal = @import("crystal/frequency.zig");
EOM

echo "[SETUP] Created temperature sensor module"
echo "[SETUP] Created crystal frequency monitor"
echo "[SETUP] Added hardware tests"
echo "[SETUP] Updated build configuration"
echo "[INFO] Next steps:"
echo "1. Implement actual sensor communication"
echo "2. Add calibration routines"
echo "3. Add error handling for hardware failures"
echo "[INFO] Run tests with: zig build test"

