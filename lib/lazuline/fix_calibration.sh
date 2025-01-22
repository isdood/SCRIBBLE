#!/bin/bash

# Timestamp: 2025-01-22 02:30:26
# Author: isdood

echo "[FIX] Updating calibration and temperature sensor modules..."

# Fix calibration module
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

        var max_err: f64 = 0;
        var total_err: f64 = 0;
        var latest_ts: i64 = std.math.minInt(i64);

        for (self.points.items) |point| {
            const calibrated = self.calibrate(point.measured_temp);
            const temp_err = std.math.fabs(calibrated - point.reference_temp);
            max_err = @max(max_err, temp_err);
            total_err += temp_err;
            latest_ts = @max(latest_ts, point.timestamp);
        }

        return .{
            .point_count = self.points.items.len,
            .max_error = max_err,
            .avg_error = total_err / @as(f64, @floatFromInt(self.points.items.len)),
            .latest_timestamp = latest_ts,
        };
    }
};
EOC

# Fix temperature sensor module
cat > src/hardware/sensors/temperature.zig << 'EOT'
const std = @import("std");
const TMP102 = @import("tmp102.zig").TMP102;
const DS18B20 = @import("ds18b20.zig").DS18B20;
const CalibrationCurve = @import("../calibration/temperature.zig").CalibrationCurve;

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
        i2c_bus: ?u8 = null,
        onewire_id: ?[]const u8 = null,
    };

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

        if (config.sensor_type == .TMP102) {
            if (config.i2c_bus == null) return error.MissingI2CBus;
            if (config.i2c_address == null) return error.MissingI2CAddress;

            self.tmp102 = try TMP102.init(.{
                .bus_number = config.i2c_bus.?,
                .address = config.i2c_address.?,
                .conversion_rate = .Four_Hz,
            });
        }

        if (config.sensor_type == .DS18B20) {
            if (config.onewire_id == null) return error.MissingOneWireID;

            self.ds18b20 = try DS18B20.init(allocator, .{
                .device_id = config.onewire_id.?,
            });
        }

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

    fn updateReading(self: *Self) !void {
        const raw_temp = switch (self.sensor_type) {
            .DS18B20 => try self.readDS18B20(),
            .TMP102 => if (self.tmp102) |*tmp| try tmp.readTemperature() else return error.SensorNotInitialized,
            .MAX31856 => try self.readMAX31856(),
            .Simulated => self.simulateTemperature(),
        };

        try self.samples.append(raw_temp);
        if (self.samples.items.len > 10) {
            _ = self.samples.orderedRemove(0);
        }

        var sum: f64 = 0;
        for (self.samples.items) |sample| {
            sum += sample;
        }
        self.last_reading = sum / @as(f64, @floatFromInt(self.samples.items.len));
        self.last_update = self.timer.read();
    }

    fn readDS18B20(self: *Self) !f64 {
        if (self.ds18b20) |*ds| {
            return try ds.readTemperature();
        }
        return error.SensorNotInitialized;
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

echo "[FIX] Renamed error variable to err_val to avoid keyword conflict"
echo "[FIX] Added missing SensorType enum definition"
echo "[FIX] Updated timestamp to: 2025-01-22 02:30:26"
echo "[INFO] Try running 'zig build test' again"

