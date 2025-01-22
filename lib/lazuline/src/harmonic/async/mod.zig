const std = @import("std");

pub const WaveFunction = struct {
    amplitude: []f64,
    phase: []f64,
    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator, size: usize) !WaveFunction {
        return WaveFunction{
            .amplitude = try allocator.alloc(f64, size),
            .phase = try allocator.alloc(f64, size),
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *WaveFunction) void {
        self.allocator.free(self.amplitude);
        self.allocator.free(self.phase);
    }
};

pub const HarmonicAsync = struct {
    const Self = @This();

    pub const Future = struct {
        wave: WaveFunction,
        completed: bool,
        result: ?[]u8,
        allocator: std.mem.Allocator,

        pub fn init(allocator: std.mem.Allocator, wave_size: usize) !Future {
            return Future{
                .wave = try WaveFunction.init(allocator, wave_size),
                .completed = false,
                .result = null,
                .allocator = allocator,
            };
        }

        pub fn deinit(self: *Future) void {
            if (self.result) |result| {
                self.allocator.free(result);
            }
            self.wave.deinit();
        }
    };

    futures: std.ArrayList(Future),
    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator) Self {
        return Self{
            .futures = std.ArrayList(Future).init(allocator),
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *Self) void {
        for (self.futures.items) |*future| {
            future.deinit();
        }
        self.futures.deinit();
    }
};
