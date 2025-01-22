const std = @import("std");

pub const WaveFunction = struct {
    amplitude: []f64,
    phase: []f64,
    frequency: []f64,
    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator, size: usize) !WaveFunction {
        return WaveFunction{
            .amplitude = try allocator.alloc(f64, size),
            .phase = try allocator.alloc(f64, size),
            .frequency = try allocator.alloc(f64, size),
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *WaveFunction) void {
        self.allocator.free(self.amplitude);
        self.allocator.free(self.phase);
        self.allocator.free(self.frequency);
    }

    pub fn interfere(self: *WaveFunction, other: *const WaveFunction) void {
        for (self.amplitude, other.amplitude) |*amp, other_amp| {
            amp.* += other_amp;
        }
    }
};

pub const WaveComputer = struct {
    waves: std.ArrayList(WaveFunction),
    interference_matrix: [][]f64,
    coherence_threshold: f64,
    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator) !WaveComputer {
        return WaveComputer{
            .waves = std.ArrayList(WaveFunction).init(allocator),
            .interference_matrix = &[_][]f64{},
            .coherence_threshold = 0.5,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *WaveComputer) void {
        for (self.waves.items) |*wave| {
            wave.deinit();
        }
        self.waves.deinit();
    }
};
