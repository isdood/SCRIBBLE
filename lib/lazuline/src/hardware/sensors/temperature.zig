const std = @import("std");
const TMP102 = @import("tmp102.zig").TMP102;
const DS18B20 = @import("ds18b20.zig").DS18B20;
const CalibrationCurve = @import("../calibration/temperature.zig").CalibrationCurve;

pub const TemperatureSensor = struct {
    const Self = @This();

    // ... (previous code remains the same until the struct fields) ...

    sensor_type: SensorType,
    last_reading: f64,
    last_update: u64,
    update_interval: u64,
    samples: std.ArrayList(f64),
    allocator: std.mem.Allocator,
    timer: std.time.Timer,
    tmp102: ?TMP102 = null,
    ds18b20: ?DS18B20 = null,
    calibration: CalibrationCurve,

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
            .calibration = CalibrationCurve.init(allocator),
        };

        // ... (rest of init remains the same) ...

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
        self.calibration.deinit();
    }

    pub fn readTemperature(self: *Self) !f64 {
        const now = self.timer.read();
        if (now - self.last_update >= self.update_interval) {
            try self.updateReading();
        }
        return self.calibration.calibrate(self.last_reading);
    }

    pub fn addCalibrationPoint(self: *Self, reference_temp: f64) !void {
        const measured = try self.readUncalibratedTemperature();
        try self.calibration.addPoint(reference_temp, measured);
    }

    pub fn getCalibrationInfo(self: Self) struct {
        point_count: usize,
        max_error: f64,
        avg_error: f64,
        latest_timestamp: i64,
    } {
        return self.calibration.getCalibrationInfo();
    }

    fn readUncalibratedTemperature(self: *Self) !f64 {
        const now = self.timer.read();
        if (now - self.last_update >= self.update_interval) {
            try self.updateReading();
        }
        return self.last_reading;
    }

    // ... (rest of the implementation remains the same) ...
};
