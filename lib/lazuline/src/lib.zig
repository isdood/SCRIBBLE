const std = @import("std");
const math = std.math;
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
            .omega = 2.0 * math.pi * frequency,
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
    buffer_index: ?usize,

    pub const CACHE_LINE_SIZE: usize = 64;
    pub const VECTOR_SIZE: usize = 4;
    pub const DEFAULT_BLOCK_SIZE: usize = 16;
    pub const MAX_SIZE: usize = DEFAULT_BLOCK_SIZE * DEFAULT_BLOCK_SIZE * DEFAULT_BLOCK_SIZE;

    // Static buffer for small lattices
    const StaticBuffer = struct {
        data: [MAX_SIZE]f64 align(32),
        used: bool,
    };

    var static_buffers: [4]StaticBuffer = .{
        .{ .data = undefined, .used = false },
        .{ .data = undefined, .used = false },
        .{ .data = undefined, .used = false },
        .{ .data = undefined, .used = false },
    };

    fn acquireStaticBuffer() struct { buffer: ?[]align(32) f64, index: ?usize } {
        for (&static_buffers, 0..) |*buffer, i| {
            if (!buffer.used) {
                buffer.used = true;
                return .{
                    .buffer = buffer.data[0..MAX_SIZE],
                    .index = i,
                };
            }
        }
        return .{ .buffer = null, .index = null };
    }

    fn releaseStaticBuffer(index: usize) void {
        if (index < static_buffers.len) {
            static_buffers[index].used = false;
        }
    }

    pub fn init(allocator: std.mem.Allocator, dimensions: [3]usize) !*CrystalLattice {
        const size = dimensions[0] * dimensions[1] * dimensions[2];
        if (size > MAX_SIZE) return error.SizeTooLarge;

        const self = try allocator.create(CrystalLattice);
        errdefer allocator.destroy(self);

        // Try to use static buffer first
        const static_result = acquireStaticBuffer();
        if (static_result.buffer) |buffer| {
            self.* = .{
                .dimensions = dimensions,
                .data = buffer[0..size],
                .allocator = allocator,
                .size = size,
                .buffer_index = static_result.index,
            };
            @memset(self.data, 0);
            return self;
        }

        // Fall back to dynamic allocation
        const data = try allocator.alignedAlloc(f64, CACHE_LINE_SIZE, size);
        errdefer allocator.free(data);

        self.* = .{
            .dimensions = dimensions,
            .data = data,
            .allocator = allocator,
            .size = size,
            .buffer_index = null,
        };

        @memset(data, 0);
        return self;
    }

    pub fn deinit(self: *CrystalLattice) void {
        if (self.buffer_index) |index| {
            releaseStaticBuffer(index);
        } else {
            self.allocator.free(self.data);
        }
        self.allocator.destroy(self);
    }

    pub fn batchSet(self: *CrystalLattice, value: f64) void {
        const AlignVector = @Vector(4, f64);
        const value_vector: AlignVector = @splat(value);

        var i: usize = 0;
        const aligned_size = self.size & ~@as(usize, 3);

        // Vector operations for 16-element chunks
        while (i + 15 < aligned_size) : (i += 16) {
            const ptr = @as(*align(32) [4]AlignVector, @ptrCast(@alignCast(self.data[i..].ptr)));
            ptr[0] = value_vector;
            ptr[1] = value_vector;
            ptr[2] = value_vector;
            ptr[3] = value_vector;
        }

        // Handle remaining aligned elements
        while (i < aligned_size) : (i += 4) {
            const ptr = @as(*align(32) AlignVector, @ptrCast(@alignCast(self.data[i..].ptr)));
            ptr.* = value_vector;
        }

        // Handle remaining elements
        while (i < self.size) : (i += 1) {
            self.data[i] = value;
        }
    }

    pub fn clone(self: *const CrystalLattice) !*CrystalLattice {
        const new_lattice = try init(self.allocator, self.dimensions);
        @memcpy(new_lattice.data, self.data);
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
