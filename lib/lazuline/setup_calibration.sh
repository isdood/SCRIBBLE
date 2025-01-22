#!/bin/bash

# Timestamp: 2025-01-22 02:28:46
# Author: isdood

echo "[INIT] Setting up temperature sensor calibration..."

# Create calibration module
mkdir -p src/hardware/calibration

# Create calibration interface
cat > src/hardware/calibration/temperature.zig << 'EOC'
const std = @import("std");

pub const CalibrationPoint = struct {
    reference_temp: f64,
    measured_temp: f64,
    timestamp: i64,
};

pub const CalibrationCurve = struct {
    const Self = @This();

    points: std.ArrayList(CalibrationPoint),
    coefficients: struct {
        offset: f64 = 0.0,
        scale: f64 = 1.0,
        quadratic: f64 = 0.0,
    },

    pub fn init(allocator: std.mem.Allocator) Self {
        return Self{
            .points = std.ArrayList(CalibrationPoint).init(allocator),
            .coefficients = .{},
        };
    }

    pub fn deinit(self: *Self) void {
        self.points.deinit();
    }

    pub fn addPoint(self: *Self, reference: f64, measured: f64) !void {
        const now = std.time.timestamp();
        try self.points.append(.{
            .reference_temp = reference,
            .measured_temp = measured,
            .timestamp = now,
        });
        try self.recalculateCoefficients();
    }

    pub fn calibrate(self: Self, measured: f64) f64 {
        const x = measured;
        return (x * self.coefficients.scale + self.coefficients.offset) +
               (x * x * self.coefficients.quadratic);
    }

    fn recalculateCoefficients(self: *Self) !void {
        if (self.points.items.len < 2) {
            // Not enough points for calibration
            self.coefficients = .{};
            return;
        }

        var sum_x: f64 = 0.0;
        var sum_y: f64 = 0.0;
        var sum_xx: f64 = 0.0;
        var sum_xy: f64 = 0.0;
        var sum_xxx: f64 = 0.0;
        var sum_xxy: f64 = 0.0;

        for (self.points.items) |point| {
            const x = point.measured_temp;
            const y = point.reference_temp;
            sum_x += x;
            sum_y += y;
            sum_xx += x * x;
            sum_xy += x * y;
            sum_xxx += x * x * x;
            sum_xxy += x * x * y;
        }

        const n = @as(f64, @floatFromInt(self.points.items.len));
        const det = (n * sum_xx * sum_xxx) - (sum_x * sum_xx * sum_xx);

        if (std.math.fabs(det) < 1e-10) {
            // Fall back to linear calibration
            const slope = (n * sum_xy - sum_x * sum_y) / (n * sum_xx - sum_x * sum_x);
            const intercept = (sum_y - slope * sum_x) / n;
            self.coefficients = .{
                .scale = slope,
                .offset = intercept,
                .quadratic = 0.0,
            };
        } else {
            // Quadratic calibration
            const a = ((n * sum_xxy) - (sum_x * sum_xy)) / det;
            const b = ((sum_xxx * sum_xy) - (sum_xx * sum_xxy)) / det;
            const c = (sum_y - b * sum_x - a * sum_xx) / n;
            self.coefficients = .{
                .scale = b,
                .offset = c,
                .quadratic = a,
            };
        }
    }

    pub fn getCalibrationInfo(self: Self) struct {
        point_count: usize,
        max_error: f64,
        avg_error: f64,
        latest_timestamp: i64,
    } {
        if (self.points.items.len == 0) {
            return .{
                .point_count = 0,
                .max_error = 0,
                .avg_error = 0,
                .latest_timestamp = 0,
            };
        }

        var max_error: f64 = 0;
        var total_error: f64 = 0;
        var latest_ts: i64 = std.math.minInt(i64);

        for (self.points.items) |point| {
            const calibrated = self.calibrate(point.measured_temp);
            const error = std.math.fabs(calibrated - point.reference_temp);
            max_error = @max(max_error, error);
            total_error += error;
            latest_ts = @max(latest_ts, point.timestamp);
        }

        return .{
            .point_count = self.points.items.len,
            .max_error = max_error,
            .avg_error = total_error / @as(f64, @floatFromInt(self.points.items.len)),
            .latest_timestamp = latest_ts,
        };
    }
};
EOC

# Update temperature sensor to include calibration
cat > src/hardware/sensors/temperature.zig << 'EOT'
const std = @import("std");
const TMP102 = @import("tmp102.zig").TMP102;
const DS18B20 = @import("ds18b20.zig").DS18B20;
const CalibrationCurve = @import("../calibration/temperature.zig").CalibrationCurve;

pub const TemperatureSensor = struct {
    const Self = @This();

    // ... (previous code remains the same until the struct fields) ...

    sensor_type: SensorType,
    last_reading: f64,
    last_update: u64,
    update_interval: u64,
    samples: std.ArrayList(f64),
    allocator: std.mem.Allocator,
    timer: std.time.Timer,
    tmp102: ?TMP102 = null,
    ds18b20: ?DS18B20 = null,
    calibration: CalibrationCurve,

    pub fn init(allocator: std.mem.Allocator, config: Config) !Self {
        var self = Self{
            .sensor_type = config.sensor_type,
            .last_reading = 25.0,
            .last_update = 0,
            .update_interval = config.update_interval,
            .samples = try std.ArrayList(f64).initCapacity(allocator, config.samples_to_average),
            .allocator = allocator,
            .timer = try std.time.Timer.start(),
            .tmp102 = null,
            .ds18b20 = null,
            .calibration = CalibrationCurve.init(allocator),
        };

        // ... (rest of init remains the same) ...

        return self;
    }

    pub fn deinit(self: *Self) void {
        if (self.tmp102) |*tmp| {
            tmp.deinit();
        }
        if (self.ds18b20) |*ds| {
            ds.deinit();
        }
        self.samples.deinit();
        self.calibration.deinit();
    }

    pub fn readTemperature(self: *Self) !f64 {
        const now = self.timer.read();
        if (now - self.last_update >= self.update_interval) {
            try self.updateReading();
        }
        return self.calibration.calibrate(self.last_reading);
    }

    pub fn addCalibrationPoint(self: *Self, reference_temp: f64) !void {
        const measured = try self.readUncalibratedTemperature();
        try self.calibration.addPoint(reference_temp, measured);
    }

    pub fn getCalibrationInfo(self: Self) struct {
        point_count: usize,
        max_error: f64,
        avg_error: f64,
        latest_timestamp: i64,
    } {
        return self.calibration.getCalibrationInfo();
    }

    fn readUncalibratedTemperature(self: *Self) !f64 {
        const now = self.timer.read();
        if (now - self.last_update >= self.update_interval) {
            try self.updateReading();
        }
        return self.last_reading;
    }

    // ... (rest of the implementation remains the same) ...
};
EOT

# Create calibration tests
cat > tests/calibration_test.zig << 'EOT'
const std = @import("std");
const testing = std.testing;
const hardware = @import("hardware");

test "temperature calibration curve" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var curve = hardware.calibration.CalibrationCurve.init(allocator);
    defer curve.deinit();

    // Add calibration points
    try curve.addPoint(0.0, -0.5);    // Sensor reads -0.5°C at 0°C
    try curve.addPoint(100.0, 99.0);  // Sensor reads 99.0°C at 100°C

    // Test calibration
    try testing.expectApprox(0.0, curve.calibrate(-0.5));
    try testing.expectApprox(100.0, curve.calibrate(99.0));
    try testing.expectApprox(50.0, curve.calibrate(49.25));
}

test "temperature sensor with calibration" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var sensor = try hardware.sensors.TemperatureSensor.init(allocator, .{
        .sensor_type = .Simulated,
        .update_interval = 100_000_000, // 100ms
    });
    defer sensor.deinit();

    // Add calibration points
    try sensor.addCalibrationPoint(25.0);  // Current simulated temp is ~25°C
    try sensor.addCalibrationPoint(30.0);  // Wait and add another point

    const info = sensor.getCalibrationInfo();
    try testing.expectEqual(@as(usize, 2), info.point_count);
    try testing.expect(info.max_error < 1.0);
}
EOT

# Update hardware module to export calibration
cat > src/hardware/mod.zig << 'EOM'
pub const sensors = struct {
    pub const TemperatureSensor = @import("sensors/temperature.zig").TemperatureSensor;
    pub const TMP102 = @import("sensors/tmp102.zig").TMP102;
    pub const DS18B20 = @import("sensors/ds18b20.zig").DS18B20;
};
pub const crystal = @import("crystal/frequency.zig");
pub const i2c = @import("i2c/bus.zig");
pub const onewire = @import("onewire/bus.zig");
pub const calibration = @import("calibration/temperature.zig");
EOM

echo "[SETUP] Created temperature calibration module"
echo "[SETUP] Updated temperature sensor with calibration"
echo "[SETUP] Added calibration tests"
echo "[INFO] Next steps:"
echo "1. Add calibration persistence"
echo "2. Implement MAX31856 thermocouple interface"
echo "3. Add temperature logging and analysis"
echo "[INFO] Run tests with: zig build test"

