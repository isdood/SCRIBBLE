#!/bin/bash

echo "[INFO] Starting Lazuline build configuration fix..."
echo "[INFO] Current time: 2025-01-23 00:40:40 UTC"
echo "[INFO] User: isdood"
echo "[INFO] Zig version: 0.13.0"

# Update lib.zig with optimized SIMD and precaching
cat > src/lib.zig << 'EOF'
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
    pub const VECTOR_SIZE: usize = 8; // Increased vector size
    pub const DEFAULT_BLOCK_SIZE: usize = 16;
    pub const MAX_SIZE: usize = DEFAULT_BLOCK_SIZE * DEFAULT_BLOCK_SIZE * DEFAULT_BLOCK_SIZE;
    pub const PREFETCH_DISTANCE: usize = 8;

    const StaticBuffer = struct {
        data: [MAX_SIZE]f64 align(32),
        used: bool,
    };

    var static_buffers: [8]StaticBuffer = .{
        .{ .data = undefined, .used = false },
        .{ .data = undefined, .used = false },
        .{ .data = undefined, .used = false },
        .{ .data = undefined, .used = false },
        .{ .data = undefined, .used = false },
        .{ .data = undefined, .used = false },
        .{ .data = undefined, .used = false },
        .{ .data = undefined, .used = false },
    };

    // Pre-initialized zero buffer for fast clearing
    var zero_buffer: [MAX_SIZE]f64 align(32) = [_]f64{0} ** MAX_SIZE;

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

        const static_result = acquireStaticBuffer();
        if (static_result.buffer) |buffer| {
            self.* = .{
                .dimensions = dimensions,
                .data = buffer[0..size],
                .allocator = allocator,
                .size = size,
                .buffer_index = static_result.index,
            };
            // Fast clear using pre-initialized zero buffer
            @memcpy(self.data, zero_buffer[0..size]);
            return self;
        }

        const data = try allocator.alignedAlloc(f64, CACHE_LINE_SIZE, size);
        errdefer allocator.free(data);

        self.* = .{
            .dimensions = dimensions,
            .data = data,
            .allocator = allocator,
            .size = size,
            .buffer_index = null,
        };

        @memcpy(data, zero_buffer[0..size]);
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
        const AlignVector = @Vector(8, f64);
        const value_vector: AlignVector = @splat(value);

        var i: usize = 0;
        const aligned_size = self.size & ~@as(usize, 7);

        // Prefetch next cache lines
        if (aligned_size >= PREFETCH_DISTANCE) {
            var prefetch_idx: usize = PREFETCH_DISTANCE;
            while (prefetch_idx < aligned_size) : (prefetch_idx += CACHE_LINE_SIZE) {
                @prefetch(&self.data[prefetch_idx], .{.locality = 3});
            }
        }

        // Vector operations for 32-element chunks
        while (i + 31 < aligned_size) : (i += 32) {
            const ptr = @as(*align(32) [4]AlignVector, @ptrCast(@alignCast(self.data[i..].ptr)));
            ptr[0] = value_vector;
            ptr[1] = value_vector;
            ptr[2] = value_vector;
            ptr[3] = value_vector;
        }

        // Handle remaining aligned elements
        while (i < aligned_size) : (i += 8) {
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

        var i: usize = 0;
        const aligned_size = self.size & ~@as(usize, 7);

        // Prefetch for clone operation
        if (aligned_size >= PREFETCH_DISTANCE) {
            var prefetch_idx: usize = PREFETCH_DISTANCE;
            while (prefetch_idx < aligned_size) : (prefetch_idx += CACHE_LINE_SIZE) {
                @prefetch(&self.data[prefetch_idx], .{.locality = 3});
                @prefetch(&new_lattice.data[prefetch_idx], .{.locality = 3});
            }
        }

        // Use SIMD for copying
        while (i < aligned_size) : (i += 8) {
            const src = @as(*align(32) const @Vector(8, f64), @ptrCast(@alignCast(self.data[i..].ptr)));
            const dst = @as(*align(32) @Vector(8, f64), @ptrCast(@alignCast(new_lattice.data[i..].ptr)));
            dst.* = src.*;
        }

        // Copy remaining elements
        while (i < self.size) : (i += 1) {
            new_lattice.data[i] = self.data[i];
        }

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
EOF

# Clean any existing build artifacts
echo "[INFO] Cleaning existing build artifacts..."
rm -rf zig-cache
rm -rf zig-out

echo "[SUCCESS] Build configuration has been updated"
echo "[INFO] Try running 'zig build bench' now"
echo ""
echo "Performance Optimizations:"
echo "1. Increased vector size to 8"
echo "2. Added prefetching"
echo "3. Pre-initialized zero buffer"
echo "4. Optimized SIMD copy"
echo "5. Doubled static buffers"
