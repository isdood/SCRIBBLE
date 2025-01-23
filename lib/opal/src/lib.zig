const std = @import("std");
const builtin = @import("builtin");
const arch = builtin.cpu.arch;

pub const version = "0.1.0";

// Previous WavePattern and QuantumResonance implementations remain unchanged...

pub const CrystalLattice = struct {
    dimensions: [3]usize,
    data: []align(32) f64,
    allocator: std.mem.Allocator,
    size: usize,
    buffer_index: ?usize,

    pub const CACHE_LINE_SIZE: usize = 64;
    pub const VECTOR_SIZE: usize = 8;
    pub const MAX_SIZE: usize = 65536;
    pub const CRYSTAL_ALIGNMENT: usize = 32;

    const VectorType = @Vector(8, f64);

    const BufferPool = struct {
        data: [MAX_SIZE * 16]f64 align(32),
        used: [16]bool,
    };

    var buffer_pool: BufferPool = .{
        .data = [_]f64{0} ** (MAX_SIZE * 16),
        .used = [_]bool{false} ** 16,
    };

    fn acquireBuffer(size: usize) ?struct { data: []align(32) f64, index: usize } {
        for (&buffer_pool.used, 0..) |*used, i| {
            if (!used.*) {
                used.* = true;
                const start = i * MAX_SIZE;
                return .{
                    .data = buffer_pool.data[start..start + size],
                    .index = i,
                };
            }
        }
        return null;
    }

    fn releaseBuffer(index: usize) void {
        if (index < buffer_pool.used.len) {
            buffer_pool.used[index] = false;
        }
    }

    pub fn init(allocator: std.mem.Allocator, dimensions: [3]usize) !*CrystalLattice {
        const size = dimensions[0] * dimensions[1] * dimensions[2];
        if (size > MAX_SIZE) return error.SizeTooLarge;

        const self = try allocator.create(CrystalLattice);
        errdefer allocator.destroy(self);

        if (acquireBuffer(size)) |buffer| {
            self.* = .{
                .dimensions = dimensions,
                .data = buffer.data,
                .allocator = allocator,
                .size = size,
                .buffer_index = buffer.index,
            };
            return self;
        }

        const data = try allocator.alignedAlloc(f64, CRYSTAL_ALIGNMENT, size);
        errdefer allocator.free(data);

        self.* = .{
            .dimensions = dimensions,
            .data = data,
            .allocator = allocator,
            .size = size,
            .buffer_index = null,
        };

        return self;
    }

    pub fn batchSet(self: *CrystalLattice, value: f64) void {
        const value_vector: VectorType = @splat(value);

        // Fast path for small sizes
        if (self.size <= VECTOR_SIZE) {
            var i: usize = 0;
            while (i < self.size) : (i += 1) {
                self.data[i] = value;
            }
            return;
        }

        // Main vectorized loop with manual unrolling
        var i: usize = 0;
        const vec_count = self.size / VECTOR_SIZE;
        const vec_end = vec_count * VECTOR_SIZE;

        // Ensure proper SIMD alignment
        const data_aligned = @alignCast(32, self.data);

        while (i < vec_end) : (i += VECTOR_SIZE * 4) {
            // Process 4 vectors at once
            inline for (0..4) |offset| {
                if (i + (VECTOR_SIZE * offset) < vec_end) {
                    const ptr = @ptrCast(*VectorType, &data_aligned[i + (VECTOR_SIZE * offset)]);
                    ptr.* = value_vector;
                }
            }
        }

        // Handle remaining elements
        while (i < self.size) : (i += 1) {
            self.data[i] = value;
        }
    }

    pub fn clone(self: *const CrystalLattice) !*CrystalLattice {
        const new_lattice = try CrystalLattice.init(self.allocator, self.dimensions);

        // Fast path for small sizes
        if (self.size <= VECTOR_SIZE) {
            @memcpy(new_lattice.data, self.data[0..self.size]);
            return new_lattice;
        }

        // Main vectorized copy loop with manual unrolling
        var i: usize = 0;
        const vec_count = self.size / VECTOR_SIZE;
        const vec_end = vec_count * VECTOR_SIZE;

        // Ensure proper SIMD alignment for source and destination
        const src_aligned = @alignCast(32, self.data);
        const dst_aligned = @alignCast(32, new_lattice.data);

        while (i < vec_end) : (i += VECTOR_SIZE * 4) {
            // Copy 4 vectors at once
            inline for (0..4) |offset| {
                if (i + (VECTOR_SIZE * offset) < vec_end) {
                    const src = @ptrCast(*const VectorType, &src_aligned[i + (VECTOR_SIZE * offset)]);
                    const dst = @ptrCast(*VectorType, &dst_aligned[i + (VECTOR_SIZE * offset)]);
                    dst.* = src.*;
                }
            }
        }

        // Copy remaining elements
        while (i < self.size) : (i += 1) {
            new_lattice.data[i] = self.data[i];
        }

        return new_lattice;
    }

    pub fn deinit(self: *CrystalLattice) void {
        if (self.buffer_index) |index| {
            releaseBuffer(index);
        } else {
            self.allocator.free(self.data);
        }
        self.allocator.destroy(self);
    }
};

// WavePattern implementation
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

// QuantumResonance implementation
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
