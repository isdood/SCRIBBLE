const std = @import("std");

pub const CrystalMonitor = struct {
    const Self = @This();

    pub const Config = struct {
        nominal_frequency: f64 = 32_768.0,
        measurement_period: u64 = 1000 * 1000 * 1000, // 1 second
        samples_to_average: u32 = 10,
    };

    nominal_freq: f64,
    current_freq: f64,
    measurement_period: u64,
    samples: std.ArrayList(f64),
    allocator: std.mem.Allocator,
    timer: std.time.Timer,
    last_update: u64,

    pub fn init(allocator: std.mem.Allocator, config: Config) !Self {
        return Self{
            .nominal_freq = config.nominal_frequency,
            .current_freq = config.nominal_frequency,
            .measurement_period = config.measurement_period,
            .samples = try std.ArrayList(f64).initCapacity(allocator, config.samples_to_average),
            .allocator = allocator,
            .timer = try std.time.Timer.start(),
            .last_update = 0,
        };
    }

    pub fn deinit(self: *Self) void {
        self.samples.deinit();
    }

    pub fn measureFrequency(self: *Self) !f64 {
        const now = self.timer.read();
        if (now - self.last_update >= self.measurement_period) {
            try self.updateFrequency();
        }
        return self.current_freq;
    }

    fn updateFrequency(self: *Self) !void {
        const raw_freq = self.simulateFrequency();
        try self.samples.append(raw_freq);
        if (self.samples.items.len > 10) {
            _ = self.samples.orderedRemove(0);
        }

        // Calculate moving average
        var sum: f64 = 0;
        for (self.samples.items) |sample| {
            sum += sample;
        }
        self.current_freq = sum / @as(f64, @floatFromInt(self.samples.items.len));
        self.last_update = self.timer.read();
    }

    fn simulateFrequency(self: *Self) f64 {
        const time = @as(f64, @floatFromInt(self.timer.read())) / 1_000_000_000.0;
        // Simulate temperature-dependent frequency drift
        return self.nominal_freq * (1.0 + 0.0001 * std.math.sin(time * 0.0001));
    }
};
