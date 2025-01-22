const std = @import("std");

pub const TemperatureCalibration = struct {
    const Self = @This();

    pub const CalibrationPoint = struct {
        temperature: f64,
        frequency: f64,
        drift: f64,
    };

    calibration_points: std.ArrayList(CalibrationPoint),
    compensation_curve: []f64,
    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator) !Self {
        return Self{
            .calibration_points = std.ArrayList(CalibrationPoint).init(allocator),
            .compensation_curve = try allocator.alloc(f64, 100), // 100 point curve
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *Self) void {
        self.calibration_points.deinit();
        self.allocator.free(self.compensation_curve);
    }

    pub fn addCalibrationPoint(self: *Self, temp: f64, freq: f64, drift: f64) !void {
        try self.calibration_points.append(.{
            .temperature = temp,
            .frequency = freq,
            .drift = drift,
        });
        try self.updateCompensationCurve();
    }

    fn updateCompensationCurve(self: *Self) !void {
        // TODO: Implement curve fitting algorithm
        // For now, use linear interpolation between points
        if (self.calibration_points.items.len < 2) return;

        const points = self.calibration_points.items;
        for (self.compensation_curve, 0..) |*comp, i| {
            const temp = @as(f64, @floatFromInt(i)) * 100.0 / @as(f64, @floatFromInt(self.compensation_curve.len));
            comp.* = self.interpolateCompensation(temp);
        }
    }

    fn interpolateCompensation(self: *Self, temp: f64) f64 {
        // Simple linear interpolation
        const points = self.calibration_points.items;
        var i: usize = 0;
        while (i < points.len - 1) : (i += 1) {
            if (temp >= points[i].temperature and temp <= points[i + 1].temperature) {
                const t = (temp - points[i].temperature) / (points[i + 1].temperature - points[i].temperature);
                return points[i].drift * (1 - t) + points[i + 1].drift * t;
            }
        }
        return 0;
    }
};
