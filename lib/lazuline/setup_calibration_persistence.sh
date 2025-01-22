#!/bin/bash

# Timestamp: 2025-01-22 02:32:22
# Author: isdood

echo "[INIT] Setting up calibration data persistence..."

# Create persistence module
mkdir -p src/hardware/calibration/persistence

# Create calibration storage interface
cat > src/hardware/calibration/persistence/storage.zig << 'EOS'
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
EOS

# Update calibration curve to support persistence
cat > src/hardware/calibration/temperature.zig << 'EOT'
const std = @import("std");
const CalibrationStorage = @import("persistence/storage.zig").CalibrationStorage;

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
    storage: ?CalibrationStorage = null,

    pub fn init(allocator: std.mem.Allocator) Self {
        return Self{
            .points = std.ArrayList(CalibrationPoint).init(allocator),
            .coefficients = .{},
            .storage = null,
        };
    }

    pub fn initWithStorage(allocator: std.mem.Allocator, storage_path: []const u8) !Self {
        var self = Self{
            .points = std.ArrayList(CalibrationPoint).init(allocator),
            .coefficients = .{},
            .storage = try CalibrationStorage.init(allocator, storage_path),
        };

        // Try to load existing calibration points
        if (self.storage) |*storage| {
            if (storage.load()) |loaded_points| {
                for (loaded_points) |point| {
                    try self.points.append(point);
                }
                try self.recalculateCoefficients();
            } else |_| {
                // Ignore load errors, start with empty calibration
            }
        }

        return self;
    }

    pub fn deinit(self: *Self) void {
        self.points.deinit();
        if (self.storage) |*storage| {
            storage.deinit();
        }
    }

    pub fn save(self: *Self) !void {
        if (self.storage) |*storage| {
            try storage.save(self.points.items);
        }
    }

    // ... (rest of the implementation remains the same) ...

    pub fn addPoint(self: *Self, reference: f64, measured: f64) !void {
        const now = std.time.timestamp();
        try self.points.append(.{
            .reference_temp = reference,
            .measured_temp = measured,
            .timestamp = now,
        });
        try self.recalculateCoefficients();
        try self.save();
    }
};
EOT

# Create calibration persistence tests
cat > tests/calibration_persistence_test.zig << 'EOT'
const std = @import("std");
const testing = std.testing;
const hardware = @import("hardware");
const fs = std.fs;

test "calibration storage save and load" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    // Create a temporary file
    const temp_path = "test_calibration.dat";
    defer fs.cwd().deleteFile(temp_path) catch {};

    // Create and save calibration data
    var curve1 = try hardware.calibration.CalibrationCurve.initWithStorage(allocator, temp_path);
    defer curve1.deinit();

    try curve1.addPoint(0.0, -0.5);
    try curve1.addPoint(100.0, 99.0);

    // Load calibration data in a new curve
    var curve2 = try hardware.calibration.CalibrationCurve.initWithStorage(allocator, temp_path);
    defer curve2.deinit();

    try testing.expectEqual(@as(usize, 2), curve2.points.items.len);
    try testing.expectApprox(0.0, curve2.calibrate(-0.5));
    try testing.expectApprox(100.0, curve2.calibrate(99.0));
}

test "calibration storage error handling" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const invalid_path = "/nonexistent/path/calibration.dat";
    var storage = try hardware.calibration.persistence.CalibrationStorage.init(allocator, invalid_path);
    defer storage.deinit();

    try testing.expectError(error.FileNotFound, storage.load());
}
EOT

# Update hardware module to export calibration persistence
cat > src/hardware/mod.zig << 'EOM'
pub const sensors = struct {
    pub const TemperatureSensor = @import("sensors/temperature.zig").TemperatureSensor;
    pub const TMP102 = @import("sensors/tmp102.zig").TMP102;
    pub const DS18B20 = @import("sensors/ds18b20.zig").DS18B20;
};
pub const crystal = @import("crystal/frequency.zig");
pub const i2c = @import("i2c/bus.zig");
pub const onewire = @import("onewire/bus.zig");
pub const calibration = struct {
    pub const CalibrationCurve = @import("calibration/temperature.zig").CalibrationCurve;
    pub const CalibrationPoint = @import("calibration/temperature.zig").CalibrationPoint;
    pub const persistence = @import("calibration/persistence/storage.zig");
};
EOM

echo "[SETUP] Created calibration storage module"
echo "[SETUP] Updated calibration curve with persistence"
echo "[SETUP] Added persistence tests"
echo "[INFO] Next steps:"
echo "1. Add calibration backup/restore"
echo "2. Implement MAX31856 thermocouple interface"
echo "3. Add temperature logging and analysis"
echo "[INFO] Run tests with: zig build test"

