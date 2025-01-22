const std = @import("std");
const CalibrationPoint = @import("../temperature.zig").CalibrationPoint;

pub const CalibrationStorage = struct {
    const Self = @This();

    pub const StorageFormat = struct {
        magic: u32 = 0x4C_54_43_50, // "LTCP" (Lazuline Temperature Calibration Points)
        version: u32 = 1,
        points: []const CalibrationPoint,
    };

    pub const Error = error{
        InvalidMagic,
        InvalidVersion,
        InvalidData,
        StorageError,
    };

    allocator: std.mem.Allocator,
    path: []const u8,

    pub fn init(allocator: std.mem.Allocator, path: []const u8) !Self {
        const duped_path = try allocator.dupe(u8, path);
        return Self{
            .allocator = allocator,
            .path = duped_path,
        };
    }

    pub fn deinit(self: *Self) void {
        self.allocator.free(self.path);
    }

    pub fn save(self: *Self, points: []const CalibrationPoint) !void {
        const data = StorageFormat{
            .points = points,
        };

        const file = try std.fs.cwd().createFile(self.path, .{});
        defer file.close();

        try file.writeAll(&std.mem.toBytes(data.magic));
        try file.writeAll(&std.mem.toBytes(data.version));

        // Write point count
        const point_count = @as(u32, @intCast(points.len));
        try file.writeAll(&std.mem.toBytes(point_count));

        // Write points
        for (points) |point| {
            try file.writeAll(&std.mem.toBytes(point.reference_temp));
            try file.writeAll(&std.mem.toBytes(point.measured_temp));
            try file.writeAll(&std.mem.toBytes(point.timestamp));
        }
    }

    pub fn load(self: *Self) ![]CalibrationPoint {
        const file = try std.fs.cwd().openFile(self.path, .{});
        defer file.close();

        var magic: u32 = undefined;
        var version: u32 = undefined;
        var point_count: u32 = undefined;

        _ = try file.readAll(std.mem.asBytes(&magic));
        if (magic != 0x4C_54_43_50) return Error.InvalidMagic;

        _ = try file.readAll(std.mem.asBytes(&version));
        if (version != 1) return Error.InvalidVersion;

        _ = try file.readAll(std.mem.asBytes(&point_count));

        var points = try std.ArrayList(CalibrationPoint).initCapacity(self.allocator, point_count);
        errdefer points.deinit();

        var i: u32 = 0;
        while (i < point_count) : (i += 1) {
            var point: CalibrationPoint = undefined;
            _ = try file.readAll(std.mem.asBytes(&point.reference_temp));
            _ = try file.readAll(std.mem.asBytes(&point.measured_temp));
            _ = try file.readAll(std.mem.asBytes(&point.timestamp));
            try points.append(point);
        }

        return points.toOwnedSlice();
    }
};
