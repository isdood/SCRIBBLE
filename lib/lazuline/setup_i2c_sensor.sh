#!/bin/bash

# Timestamp: 2025-01-22 02:10:33
# Author: isdood

echo "[INIT] Setting up I2C sensor communication..."

# Create I2C hardware interface
mkdir -p src/hardware/i2c

# Create I2C bus interface
cat > src/hardware/i2c/bus.zig << 'EOI'
const std = @import("std");
const os = std.os;
const fs = std.fs;

pub const I2CBus = struct {
    const Self = @This();

    pub const Error = error{
        BusError,
        DeviceNotFound,
        AccessDenied,
        Timeout,
    };

    fd: fs.File,
    bus_number: u8,

    pub fn init(bus_number: u8) !Self {
        const path = try std.fmt.allocPrint(
            std.heap.page_allocator,
            "/dev/i2c-{d}",
            .{bus_number}
        );
        defer std.heap.page_allocator.free(path);

        const file = try fs.cwd().openFile(path, .{ .mode = .read_write });

        return Self{
            .fd = file,
            .bus_number = bus_number,
        };
    }

    pub fn deinit(self: *Self) void {
        self.fd.close();
    }

    pub fn writeReg(self: *Self, device_addr: u8, reg: u8, value: u8) !void {
        _ = try self.fd.write(&[_]u8{ reg, value });
    }

    pub fn readReg(self: *Self, device_addr: u8, reg: u8) !u8 {
        _ = try self.fd.write(&[_]u8{reg});
        var buf: [1]u8 = undefined;
        _ = try self.fd.read(&buf);
        return buf[0];
    }

    pub fn read16(self: *Self, device_addr: u8, reg: u8) !u16 {
        _ = try self.fd.write(&[_]u8{reg});
        var buf: [2]u8 = undefined;
        _ = try self.fd.read(&buf);
        return @as(u16, buf[0]) << 8 | buf[1];
    }
};
EOI

# Create TMP102 sensor driver
cat > src/hardware/sensors/tmp102.zig << 'EOT'
const std = @import("std");
const I2CBus = @import("../i2c/bus.zig").I2CBus;

pub const TMP102 = struct {
    const Self = @This();

    pub const Config = struct {
        bus_number: u8 = 1,
        address: u8 = 0x48,
        conversion_rate: ConversionRate = .Quarter_Hz,
    };

    pub const ConversionRate = enum(u8) {
        Quarter_Hz = 0,
        One_Hz = 1,
        Four_Hz = 2,
        Eight_Hz = 3,
    };

    bus: I2CBus,
    address: u8,
    last_temp: f64,
    last_read: u64,
    timer: std.time.Timer,

    pub fn init(config: Config) !Self {
        return Self{
            .bus = try I2CBus.init(config.bus_number),
            .address = config.address,
            .last_temp = 0,
            .last_read = 0,
            .timer = try std.time.Timer.start(),
        };
    }

    pub fn deinit(self: *Self) void {
        self.bus.deinit();
    }

    pub fn readTemperature(self: *Self) !f64 {
        const raw = try self.bus.read16(self.address, 0x00);
        const temp = @as(f64, @floatFromInt(raw)) * 0.0625;
        self.last_temp = temp;
        self.last_read = self.timer.read();
        return temp;
    }

    pub fn configure(self: *Self, rate: ConversionRate) !void {
        var config: u16 = 0;
        config |= @as(u16, @intFromEnum(rate)) << 6;
        config |= 0x60; // 12-bit resolution
        try self.bus.writeReg(self.address, 0x01, @intCast((config >> 8) & 0xFF));
        try self.bus.writeReg(self.address, 0x02, @intCast(config & 0xFF));
    }
};
EOT

# Update temperature sensor module to use TMP102
cat > src/hardware/sensors/temperature.zig << 'EOS'
const std = @import("std");
const TMP102 = @import("tmp102.zig").TMP102;

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
    };

    sensor_type: SensorType,
    last_reading: f64,
    last_update: u64,
    update_interval: u64,
    samples: std.ArrayList(f64),
    allocator: std.mem.Allocator,
    timer: std.time.Timer,
    tmp102: ?TMP102 = null,

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

        return self;
    }

    pub fn deinit(self: *Self) void {
        if (self.tmp102) |*tmp| {
            tmp.deinit();
        }
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
        // TODO: Implement actual DS18B20 reading
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
EOS

# Create test for TMP102 sensor
cat > tests/tmp102_test.zig << 'EOT'
const std = @import("std");
const testing = std.testing;
const hardware = @import("hardware");

test "TMP102 sensor configuration" {
    if (!@import("builtin").is_test) return error.SkipZigTest;

    const config = hardware.sensors.TMP102.Config{
        .bus_number = 1,
        .address = 0x48,
        .conversion_rate = .Four_Hz,
    };

    var sensor = try hardware.sensors.TMP102.init(config);
    defer sensor.deinit();

    try sensor.configure(.Four_Hz);
}

test "TMP102 temperature reading simulation" {
    if (!@import("builtin").is_test) return error.SkipZigTest;

    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var sensor = try hardware.sensors.TemperatureSensor.init(allocator, .{
        .sensor_type = .TMP102,
        .i2c_bus = 1,
        .i2c_address = 0x48,
        .update_interval = 100_000_000, // 100ms
    });
    defer sensor.deinit();

    const temp = try sensor.readTemperature();
    try testing.expect(temp >= -55.0 and temp <= 150.0); // TMP102 range
}
EOT

# Update hardware module to export TMP102
cat > src/hardware/mod.zig << 'EOM'
pub const sensors = struct {
    pub const TemperatureSensor = @import("sensors/temperature.zig").TemperatureSensor;
    pub const TMP102 = @import("sensors/tmp102.zig").TMP102;
};
pub const crystal = @import("crystal/frequency.zig");
pub const i2c = @import("i2c/bus.zig");
EOM

echo "[SETUP] Created I2C bus interface"
echo "[SETUP] Created TMP102 sensor driver"
echo "[SETUP] Updated temperature sensor module"
echo "[SETUP] Added TMP102 tests"
echo "[INFO] Next steps:"
echo "1. Add error handling for I2C communication"
echo "2. Implement other sensor types"
echo "3. Add calibration support"
echo "[INFO] Run tests with: zig build test"

