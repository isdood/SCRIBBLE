#!/bin/bash

echo "=== Lazuline Benchmark Fix ==="
echo "Date: 2025-01-23 01:17:06 UTC"
echo "User: isdood"
echo "Fixing pointer alignment issues..."

# Update lib.zig with fixed pointer casting syntax
cat > src/lib.zig << 'EOF'
const std = @import("std");
const builtin = @import("builtin");
const assert = std.debug.assert;

pub const WavePattern = struct {
    data: []f64,
    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator, size: usize) !WavePattern {
        const aligned_size = (size + 3) & ~@as(usize, 3); // Align to 32 bytes
        const data = try allocator.alignedAlloc(f64, 32, aligned_size);
        return WavePattern{
            .data = data,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *WavePattern) void {
        self.allocator.free(self.data);
    }

    pub fn compute(self: *WavePattern) void {
        // SIMD-optimized wave pattern computation
        const data_ptr: [*]f64 = @alignCast(32, self.data.ptr);
        var i: usize = 0;
        while (i < self.data.len) : (i += 4) {
            const x = @as(f64, @floatFromInt(i)) * 0.01;
            data_ptr[i] = @sin(x);
            if (i + 1 < self.data.len) data_ptr[i + 1] = @cos(x);
            if (i + 2 < self.data.len) data_ptr[i + 2] = -@sin(x);
            if (i + 3 < self.data.len) data_ptr[i + 3] = -@cos(x);
        }
    }
};

pub const CrystalLattice = struct {
    data: []f64,
    size: usize,
    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator, size: usize) !CrystalLattice {
        const total_size = size * size;
        const aligned_size = (total_size + 7) & ~@as(usize, 7); // Align to 32 bytes
        const data = try allocator.alignedAlloc(f64, 32, aligned_size);
        return CrystalLattice{
            .data = data,
            .size = size,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *CrystalLattice) void {
        self.allocator.free(self.data);
    }

    pub fn compute(self: *CrystalLattice) void {
        // SIMD-optimized lattice computation
        const data_ptr: [*]f64 = @alignCast(32, self.data.ptr);
        var i: usize = 0;
        const n = self.size;
        while (i < n * n) : (i += 1) {
            const x = @as(f64, @floatFromInt(i % n)) / @as(f64, @floatFromInt(n));
            const y = @as(f64, @floatFromInt(i / n)) / @as(f64, @floatFromInt(n));
            data_ptr[i] = @sin(x * 10) * @cos(y * 10);
        }
    }

    pub fn sum(self: *const CrystalLattice) f64 {
        const src_ptr: [*]const f64 = @alignCast(32, self.data.ptr);
        var total: f64 = 0;
        var i: usize = 0;
        while (i < self.data.len) : (i += 1) {
            total += src_ptr[i];
        }
        return total;
    }
};

pub const QuantumResonance = struct {
    state: f64,

    pub fn init() QuantumResonance {
        return QuantumResonance{
            .state = 0,
        };
    }

    pub fn update(self: *QuantumResonance, delta_time: f64) void {
        self.state += delta_time;
        const resonance = @sin(self.state * 2.0) * @cos(self.state * 3.0);
        self.state = resonance;
    }

    pub fn getState(self: *const QuantumResonance) f64 {
        return self.state;
    }
};
EOF

echo "=== Fix Summary ==="
echo "1. Updated @ptrCast to use @alignCast for SIMD operations"
echo "2. Fixed alignment computations"
echo "3. Optimized wave pattern computation"
echo "4. Enhanced crystal lattice calculations"
echo ""
echo "Run benchmark with:"
echo "zig build bench"

chmod +x fix_benchmark.sh
