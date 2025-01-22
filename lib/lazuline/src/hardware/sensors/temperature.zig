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
