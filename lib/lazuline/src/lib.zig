const std = @import("std");
const builtin = @import("builtin");
const arch = builtin.cpu.arch;

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

pub const CrystalLattice = struct {
    // Rest of the implementation remains the same...
    dimensions: [3]usize,
    data: []align(64) f64,
    allocator: std.mem.Allocator,
    size: usize,
    buffer_index: ?usize,

    pub const CACHE_LINE_SIZE: usize = 64;
    pub const VECTOR_SIZE: usize = 8;
    pub const MAX_SIZE: usize = 65536;
    pub const CRYSTAL_ALIGNMENT: usize = 64;

    const VectorType = @Vector(8, f64);

    const BufferPool = struct {
        data: [MAX_SIZE * 16]f64 align(64),
        used: [16]bool,
    };

    var buffer_pool: BufferPool = .{
        .data = [_]f64{0} ** (MAX_SIZE * 16),
        .used = [_]bool{false} ** 16,
    };

    fn acquireBuffer(size: usize) ?struct { data: []align(64) f64, index: usize } {
        for (&buffer_pool.used, 0..) |*used, i| {
            if (!used.*) {
                used.* = true;
                const start = i * MAX_SIZE;
                const slice = buffer_pool.data[start..start + size];
                const aligned_slice: []align(64) f64 = @alignCast(slice);
                return .{
                    .data = aligned_slice,
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

        if (self.size <= VECTOR_SIZE) {
            @memset(self.data, value);
            return;
        }

        const vec_count = self.size / VECTOR_SIZE;
        const vec_end = vec_count * VECTOR_SIZE;

        var i: usize = 0;
        while (i < vec_end) : (i += VECTOR_SIZE * 4) {
            inline for (0..4) |offset| {
                if (i + (VECTOR_SIZE * offset) < vec_end) {
                    const ptr_aligned: *align(64) f64 = @alignCast(&self.data[i + (VECTOR_SIZE * offset)]);
                    const vec_ptr: *VectorType = @ptrCast(ptr_aligned);
                    vec_ptr.* = value_vector;
                }
            }
        }

        while (i < self.size) : (i += 1) {
            self.data[i] = value;
        }
    }

    pub fn clone(self: *const CrystalLattice) !*CrystalLattice {
        const new_lattice = try CrystalLattice.init(self.allocator, self.dimensions);

        if (self.size <= VECTOR_SIZE) {
            @memcpy(new_lattice.data, self.data[0..self.size]);
            return new_lattice;
        }

        const vec_count = self.size / VECTOR_SIZE;
        const vec_end = vec_count * VECTOR_SIZE;

        var i: usize = 0;
        while (i < vec_end) : (i += VECTOR_SIZE * 4) {
            inline for (0..4) |offset| {
                if (i + (VECTOR_SIZE * offset) < vec_end) {
                    const src_aligned: *align(64) const f64 = @alignCast(&self.data[i + (VECTOR_SIZE * offset)]);
                    const dst_aligned: *align(64) f64 = @alignCast(&new_lattice.data[i + (VECTOR_SIZE * offset)]);
                    const src: *const VectorType = @ptrCast(src_aligned);
                    const dst: *VectorType = @ptrCast(dst_aligned);
                    dst.* = src.*;
                }
            }
        }

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
