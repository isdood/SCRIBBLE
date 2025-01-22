#!/bin/bash

# Timestamp: 2025-01-22 02:24:09
# Author: isdood

echo "[FIX] Updating DS18B20 sensor implementation..."

# Update DS18B20 sensor driver
cat > src/hardware/sensors/ds18b20.zig << 'EOD'
const std = @import("std");
const OneWireBus = @import("../onewire/bus.zig").OneWireBus;

pub const DS18B20 = struct {
    const Self = @This();

    pub const Config = struct {
        device_id: []const u8,
        resolution: OneWireBus.Resolution = .Bits12,
        conversion_delay: u64 = 750_000_000, // 750ms for 12-bit resolution
    };

    bus: OneWireBus,
    config: Config,
    allocator: std.mem.Allocator,
    timer: std.time.Timer,
    last_conversion: u64,

    pub fn init(allocator: std.mem.Allocator, config: Config) !Self {
        return Self{
            .bus = try OneWireBus.init(allocator, config.device_id),
            .config = .{
                .device_id = try allocator.dupe(u8, config.device_id),
                .resolution = config.resolution,
                .conversion_delay = config.conversion_delay,
            },
            .allocator = allocator,
            .timer = try std.time.Timer.start(),
            .last_conversion = 0,
        };
    }

    pub fn deinit(self: *Self) void {
        self.bus.deinit();
        self.allocator.free(self.config.device_id);
    }

    pub fn readTemperature(self: *Self) !f64 {
        const now = self.timer.read();

        // Start new conversion if needed
        if (now - self.last_conversion >= self.config.conversion_delay) {
            try self.bus.startConversion();
            self.last_conversion = now;
            std.time.sleep(self.config.conversion_delay);
        }

        const raw_data = try self.bus.readRaw();
        defer self.allocator.free(raw_data);

        if (!OneWireBus.checkCrc(raw_data)) {
            return error.CrcError;
        }

        return try Self.parseTemperature(raw_data);
    }

    fn parseTemperature(raw_data: []const u8) !f64 {
        // Parse the temperature from raw data format:
        // "xx xx xx xx xx xx xx xx xx : crc=xx YES\nxx xx xx xx xx xx xx xx t=12345"
        var iter = std.mem.split(u8, raw_data, "t=");
        _ = iter.next() orelse return error.InvalidData;
        const temp_str = iter.next() orelse return error.InvalidData;

        const temp_val = try std.fmt.parseInt(i32, temp_str, 10);
        return @as(f64, @floatFromInt(temp_val)) / 1000.0;
    }

    pub fn setResolution(self: *Self, resolution: OneWireBus.Resolution) !void {
        try self.bus.setResolution(resolution);
        self.config.resolution = resolution;

        // Update conversion delay based on resolution
        self.config.conversion_delay = switch (resolution) {
            .Bits9 => 94_000_000,   // 94ms
            .Bits10 => 188_000_000, // 188ms
            .Bits11 => 375_000_000, // 375ms
            .Bits12 => 750_000_000, // 750ms
        };
    }
};
EOD

# Update temperature sensor module directly
cat > src/hardware/sensors/temperature.zig << 'EOT'
const std = @import("std");
const TMP102 = @import("tmp102.zig").TMP102;
const DS18B20 = @import("ds18b20.zig").DS18B20;

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

echo "[FIX] Made parseTemperature static"
echo "[FIX] Removed unused self parameter"
echo "[FIX] Updated temperature sensor module directly"
echo "[FIX] Updated timestamp to: 2025-01-22 02:24:09"
echo "[INFO] Try running 'zig build test' again"

