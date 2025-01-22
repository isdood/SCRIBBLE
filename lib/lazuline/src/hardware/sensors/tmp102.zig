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
