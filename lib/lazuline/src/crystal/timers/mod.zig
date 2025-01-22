const std = @import("std");

pub const CrystalTimer = struct {
    const Self = @This();

    pub const Config = struct {
        precision: u64 = 1_000_000, // nanoseconds
        crystal_freq: f64 = 32_768.0, // typical crystal oscillator frequency
        temp_compensation: f64 = 0.0, // temperature compensation in ppm/°C
    };

    timer: std.time.Timer,
    crystal_period: f64,
    temp_coefficient: f64,
    precision: u64,
    last_tick: u64,
    drift: f64,

    pub fn init(config: Config) !Self {
        return Self{
            .timer = try std.time.Timer.start(),
            .crystal_period = 1.0 / config.crystal_freq,
            .temp_coefficient = config.temp_compensation,
            .precision = config.precision,
            .last_tick = 0,
            .drift = 0.0,
        };
    }

    pub fn tick(self: *Self) !u64 {
        const now = self.timer.read();
        const elapsed = now - self.last_tick;
        self.last_tick = now;

        // Simulate crystal oscillator temperature effects
        self.drift += self.temp_coefficient * @as(f64, @floatFromInt(elapsed)) * 1e-12;

        return @as(u64, @intFromFloat(@as(f64, @floatFromInt(elapsed)) * (1.0 + self.drift)));
    }

    pub fn sleep(self: *Self, duration: u64) void {
        const adjusted_duration = @as(u64, @intFromFloat(
            @as(f64, @floatFromInt(duration)) / (1.0 + self.drift)
        ));
        std.time.sleep(adjusted_duration);
    }

    pub fn reset(self: *Self) void {
        self.drift = 0.0;
        self.last_tick = self.timer.read();
    }
};
