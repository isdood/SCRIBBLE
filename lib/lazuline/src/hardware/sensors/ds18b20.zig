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
