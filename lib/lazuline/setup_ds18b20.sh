#!/bin/bash

# Timestamp: 2025-01-22 02:16:19
# Author: guavabot1

echo "[INIT] Setting up DS18B20 1-Wire sensor support..."

# Create 1-Wire interface module
mkdir -p src/hardware/onewire

# Create 1-Wire bus interface
cat > src/hardware/onewire/bus.zig << 'EOW'
const std = @import("std");
const fs = std.fs;

pub const OneWireBus = struct {
    const Self = @This();

    pub const Error = error{
        BusError,
        DeviceNotFound,
        AccessDenied,
        CrcError,
        ConversionTimeout,
    };

    pub const Resolution = enum(u8) {
        Bits9 = 0x1F,
        Bits10 = 0x3F,
        Bits11 = 0x5F,
        Bits12 = 0x7F,
    };

    device_path: []const u8,
    device_id: []const u8,
    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator, device_id: []const u8) !Self {
        const path = try std.fmt.allocPrint(
            allocator,
            "/sys/bus/w1/devices/{s}/w1_slave",
            .{device_id}
        );

        return Self{
            .device_path = path,
            .device_id = try allocator.dupe(u8, device_id),
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *Self) void {
        self.allocator.free(self.device_path);
        self.allocator.free(self.device_id);
    }

    pub fn readRaw(self: *Self) ![]const u8 {
        const file = try fs.cwd().openFile(self.device_path, .{});
        defer file.close();

        var buf: [128]u8 = undefined;
        const bytes_read = try file.readAll(&buf);
        return try self.allocator.dupe(u8, buf[0..bytes_read]);
    }

    pub fn setResolution(self: *Self, resolution: Resolution) !void {
        // Write resolution to device's configuration register
        const config_file = try std.fmt.allocPrint(
            self.allocator,
            "/sys/bus/w1/devices/{s}/resolution",
            .{self.device_id}
        );
        defer self.allocator.free(config_file);

        const file = try fs.cwd().createFile(config_file, .{});
        defer file.close();

        const value = @intFromEnum(resolution);
        try file.writer().print("{d}", .{value});
    }

    pub fn startConversion(self: *Self) !void {
        const trigger_file = try std.fmt.allocPrint(
            self.allocator,
            "/sys/bus/w1/devices/{s}/temperature",
            .{self.device_id}
        );
        defer self.allocator.free(trigger_file);

        const file = try fs.cwd().createFile(trigger_file, .{});
        defer file.close();

        try file.writer().print("1", .{});
    }

    pub fn checkCrc(data: []const u8) bool {
        var crc: u8 = 0;
        for (data) |byte| {
            crc = crc ^ byte;
            for (0..8) |_| {
                if (crc & 0x01 != 0) {
                    crc = (crc >> 1) ^ 0x8C;
                } else {
                    crc >>= 1;
                }
            }
        }
        return crc == 0;
    }
};
EOW

# Create DS18B20 sensor driver
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

        return self.parseTemperature(raw_data);
    }

    fn parseTemperature(self: *Self, raw_data: []const u8) !f64 {
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

# Update temperature sensor module to use DS18B20
cat > patches/temperature.patch << 'EOP'
--- src/hardware/sensors/temperature.zig
+++ src/hardware/sensors/temperature.zig
@@ -1,5 +1,6 @@
 const std = @import("std");
 const TMP102 = @import("tmp102.zig").TMP102;
+const DS18B20 = @import("ds18b20.zig").DS18B20;

 pub const TemperatureSensor = struct {
     const Self = @This();
@@ -16,6 +17,7 @@ pub const TemperatureSensor = struct {
         update_interval: u64 = 1000 * 1000 * 1000, // 1 second in nanoseconds
         samples_to_average: u32 = 10,
         i2c_bus: ?u8 = null,
+        onewire_id: ?[]const u8 = null,
     };

     sensor_type: SensorType,
@@ -25,6 +27,7 @@ pub const TemperatureSensor = struct {
     allocator: std.mem.Allocator,
     timer: std.time.Timer,
     tmp102: ?TMP102 = null,
+    ds18b20: ?DS18B20 = null,

     pub fn init(allocator: std.mem.Allocator, config: Config) !Self {
         var self = Self{
@@ -35,6 +38,7 @@ pub const TemperatureSensor = struct {
             .allocator = allocator,
             .timer = try std.time.Timer.start(),
             .tmp102 = null,
+            .ds18b20 = null,
         };

         if (config.sensor_type == .TMP102) {
@@ -47,12 +51,24 @@ pub const TemperatureSensor = struct {
             });
         }

+        if (config.sensor_type == .DS18B20) {
+            if (config.onewire_id == null) return error.MissingOneWireID;
+
+            self.ds18b20 = try DS18B20.init(allocator, .{
+                .device_id = config.onewire_id.?,
+            });
+        }
+
         return self;
     }

     pub fn deinit(self: *Self) void {
         if (self.tmp102) |*tmp| {
             tmp.deinit();
+        }
+        if (self.ds18b20) |*ds| {
+            ds.deinit();
+        }
         self.samples.deinit();
     }

@@ -66,7 +82,7 @@ pub const TemperatureSensor = struct {
     fn updateReading(self: *Self) !void {
         const raw_temp = switch (self.sensor_type) {
             .DS18B20 => try self.readDS18B20(),
-            .TMP102 => if (self.tmp102) |*tmp| try tmp.readTemperature() else error.SensorNotInitialized,
+            .TMP102 => if (self.tmp102) |*tmp| try tmp.readTemperature() else return error.SensorNotInitialized,
             .MAX31856 => try self.readMAX31856(),
             .Simulated => self.simulateTemperature(),
         };
@@ -84,8 +100,11 @@ pub const TemperatureSensor = struct {
     }

     fn readDS18B20(self: *Self) !f64 {
-        // TODO: Implement actual DS18B20 reading
-        return self.simulateTemperature();
+        if (self.ds18b20) |*ds| {
+            return try ds.readTemperature();
+        }
+
+        return error.SensorNotInitialized;
     }

     fn readMAX31856(self: *Self) !f64 {
EOP

# Create test for DS18B20 sensor
cat > tests/ds18b20_test.zig << 'EOT'
const std = @import("std");
const testing = std.testing;
const hardware = @import("hardware");

test "DS18B20 sensor resolution configuration" {
    if (!@import("builtin").is_test) return error.SkipZigTest;

    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var sensor = try hardware.sensors.DS18B20.init(allocator, .{
        .device_id = "28-xxxxxxxxxxxx",
    });
    defer sensor.deinit();

    try sensor.setResolution(.Bits12);
    try testing.expectEqual(sensor.config.conversion_delay, 750_000_000);

    try sensor.setResolution(.Bits9);
    try testing.expectEqual(sensor.config.conversion_delay, 94_000_000);
}

test "DS18B20 CRC calculation" {
    const test_data = "28 01 4B 46 7F FF 0C 10 5C : crc=5C YES\n28 01 4B 46 7F FF 0C 10 5C t=23125";
    try testing.expect(hardware.onewire.OneWireBus.checkCrc(test_data));
}

test "DS18B20 temperature reading simulation" {
    if (!@import("builtin").is_test) return error.SkipZigTest;

    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var sensor = try hardware.sensors.TemperatureSensor.init(allocator, .{
        .sensor_type = .DS18B20,
        .onewire_id = "28-xxxxxxxxxxxx",
        .update_interval = 100_000_000, // 100ms
    });
    defer sensor.deinit();

    const temp = try sensor.readTemperature();
    try testing.expect(temp >= -55.0 and temp <= 125.0); // DS18B20 range
}
EOT

# Update hardware module to export DS18B20
cat > src/hardware/mod.zig << 'EOM'
pub const sensors = struct {
    pub const TemperatureSensor = @import("sensors/temperature.zig").TemperatureSensor;
    pub const TMP102 = @import("sensors/tmp102.zig").TMP102;
    pub const DS18B20 = @import("sensors/ds18b20.zig").DS18B20;
};
pub const crystal = @import("crystal/frequency.zig");
pub const i2c = @import("i2c/bus.zig");
pub const onewire = @import("onewire/bus.zig");
EOM

echo "[SETUP] Created 1-Wire bus interface"
echo "[SETUP] Created DS18B20 sensor driver"
echo "[SETUP] Updated temperature sensor module"
echo "[SETUP] Added DS18B20 tests"
echo "[INFO] Next steps:"
echo "1. Add temperature sensor calibration"
echo "2. Implement MAX31856 thermocouple interface"
echo "3. Add temperature logging and analysis"
echo "[INFO] Run tests with: zig build test"

