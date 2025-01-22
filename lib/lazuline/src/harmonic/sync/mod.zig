const std = @import("std");

pub const HarmonicMutex = struct {
    const Self = @This();

    mutex: std.Thread.Mutex,
    resonance_pattern: []f64,
    current_frequency: f64,
    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator, pattern_size: usize) !Self {
        const resonance = try allocator.alloc(f64, pattern_size);
        for (resonance, 0..) |*v, i| {
            v.* = std.math.sin(@as(f64, @floatFromInt(i)) * std.math.pi * 2.0 / @as(f64, @floatFromInt(pattern_size)));
        }

        return Self{
            .mutex = std.Thread.Mutex{},
            .resonance_pattern = resonance,
            .current_frequency = 440.0,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *Self) void {
        self.allocator.free(self.resonance_pattern);
    }

    pub fn lock(self: *Self) void {
        self.mutex.lock();
        self.current_frequency *= 2.0; // Increase frequency when locked
    }

    pub fn unlock(self: *Self) void {
        self.current_frequency *= 0.5; // Decrease frequency when unlocked
        self.mutex.unlock();
    }
};
