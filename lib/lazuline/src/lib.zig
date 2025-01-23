const std = @import("std");
const builtin = @import("builtin");

pub const version = "0.1.0";

pub const WavePattern = struct {
    amplitude: f64,
    frequency: f64,
    phase: f64,
    omega: f64,

    pub fn new(amplitude: f64, frequency: f64, phase: f64) WavePattern {
        return WavePattern{
            .amplitude = amplitude,
            .frequency = frequency,
            .phase = phase,
            .omega = 2.0 * std.math.pi * frequency,
        };
    }

    pub fn compute(self: WavePattern, time: f64) f64 {
        return self.amplitude * @sin(self.omega * time + self.phase);
    }
};

pub const CrystalLattice = struct {
    dimensions: [3]usize,
    data: []align(32) f64,
    allocator: std.mem.Allocator,
    size: usize,

    // Fixed cache line size to 64 bytes for modern CPUs
    const CACHE_LINE_SIZE: usize = 64;
    const VECTOR_SIZE: usize = 4;

    pub fn init(allocator: std.mem.Allocator, dimensions: [3]usize) !*CrystalLattice {
        const self = try allocator.create(CrystalLattice);
        const size = dimensions[0] * dimensions[1] * dimensions[2];
        const data = try allocator.alignedAlloc(f64, CACHE_LINE_SIZE, size);

        self.* = .{
            .dimensions = dimensions,
            .data = data,
            .allocator = allocator,
            .size = size,
        };

        @memset(data, 0);
        return self;
    }

    pub fn deinit(self: *CrystalLattice) void {
        self.allocator.free(self.data);
        self.allocator.destroy(self);
    }

    pub fn batchSet(self: *CrystalLattice, value: f64) void {
        const aligned_size = self.size & ~@as(usize, VECTOR_SIZE - 1);

        var i: usize = 0;
        // Process 4 elements at a time
        while (i < aligned_size) : (i += VECTOR_SIZE) {
            self.data[i] = value;
            self.data[i + 1] = value;
            self.data[i + 2] = value;
            self.data[i + 3] = value;
        }
        // Handle remaining elements
        while (i < self.size) : (i += 1) {
            self.data[i] = value;
        }
    }

    pub fn clone(self: *const CrystalLattice) !*CrystalLattice {
        const new_lattice = try self.allocator.create(CrystalLattice);
        const new_data = try self.allocator.alignedAlloc(f64, CACHE_LINE_SIZE, self.size);
        @memcpy(new_data, self.data);

        new_lattice.* = .{
            .dimensions = self.dimensions,
            .data = new_data,
            .allocator = self.allocator,
            .size = self.size,
        };

        return new_lattice;
    }
};

pub const QuantumResonance = struct {
    frequency_inv: f64,
    coherence: f64,

    pub fn new(frequency: f64, coherence: f64) QuantumResonance {
        return QuantumResonance{
            .frequency_inv = 1.0 / frequency,
            .coherence = coherence,
        };
    }

    pub fn calculate(self: QuantumResonance, time: f64) f64 {
        return self.coherence * @exp(-time * self.frequency_inv);
    }
};
