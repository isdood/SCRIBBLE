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

    pub fn calibrate(self: Self, measured: f64) f64 {
        const x = measured;
        return (x * self.coefficients.scale + self.coefficients.offset) +
               (x * x * self.coefficients.quadratic);
    }

    fn recalculateCoefficients(self: *Self) !void {
        if (self.points.items.len < 2) {
            // Not enough points for calibration, use identity transform
            self.coefficients = .{
                .offset = 0.0,
                .scale = 1.0,
                .quadratic = 0.0,
            };
            return;
        }

        var sum_x: f64 = 0.0;
        var sum_y: f64 = 0.0;
        var sum_xx: f64 = 0.0;
        var sum_xy: f64 = 0.0;
        var sum_xxx: f64 = 0.0;
        var sum_xxy: f64 = 0.0;

        for (self.points.items) |point| {
            const x = point.measured_temp;
            const y = point.reference_temp;
            sum_x += x;
            sum_y += y;
            sum_xx += x * x;
            sum_xy += x * y;
            sum_xxx += x * x * x;
            sum_xxy += x * x * y;
        }

        const n = @as(f64, @floatFromInt(self.points.items.len));
        const det = (n * sum_xx * sum_xxx) - (sum_x * sum_xx * sum_xx);

        if (std.math.fabs(det) < 1e-10) {
            // Fall back to linear calibration
            const slope = (n * sum_xy - sum_x * sum_y) / (n * sum_xx - sum_x * sum_x);
            const intercept = (sum_y - slope * sum_x) / n;
            self.coefficients = .{
                .scale = slope,
                .offset = intercept,
                .quadratic = 0.0,
            };
        } else {
            // Quadratic calibration
            const a = ((n * sum_xxy) - (sum_x * sum_xy)) / det;
            const b = ((sum_xxx * sum_xy) - (sum_xx * sum_xxy)) / det;
            const c = (sum_y - b * sum_x - a * sum_xx) / n;
            self.coefficients = .{
                .scale = b,
                .offset = c,
                .quadratic = a,
            };
        }
    }

    pub fn getCalibrationInfo(self: Self) struct {
        point_count: usize,
        max_error: f64,
        avg_error: f64,
        latest_timestamp: i64,
    } {
        if (self.points.items.len == 0) {
            return .{
                .point_count = 0,
                .max_error = 0,
                .avg_error = 0,
                .latest_timestamp = 0,
            };
        }

        var max_err: f64 = 0;
        var total_err: f64 = 0;
        var latest_ts: i64 = std.math.minInt(i64);

        for (self.points.items) |point| {
            const calibrated = self.calibrate(point.measured_temp);
            const temp_err = std.math.fabs(calibrated - point.reference_temp);
            max_err = @max(max_err, temp_err);
            total_err += temp_err;
            latest_ts = @max(latest_ts, point.timestamp);
        }

        return .{
            .point_count = self.points.items.len,
            .max_error = max_err,
            .avg_error = total_err / @as(f64, @floatFromInt(self.points.items.len)),
            .latest_timestamp = latest_ts,
        };
    }
};
