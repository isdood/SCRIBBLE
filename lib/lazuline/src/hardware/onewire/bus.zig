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
