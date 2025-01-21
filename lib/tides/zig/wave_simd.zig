//! Wave SIMD operations for crystal pattern analysis
//! Created: 2025-01-21 13:54:51 UTC
//! Author: @isdood

const std = @import("std");
const math = std.math;
const mem = std.mem;
const Vector = std.meta.Vector;
const Complex = std.math.Complex;

// SIMD vector types for different precisions
pub const Vec4f32 = Vector(4, f32);
pub const Vec8f32 = Vector(8, f32);
pub const Vec4f64 = Vector(4, f64);
pub const Vec2Complex = Vector(4, f32); // Packed complex numbers [r1,i1,r2,i2]

pub const WaveError = error{
    InvalidDimensions,
    UnsupportedOperation,
    AlignmentError,
};

/// Configuration for wave processing
pub const WaveConfig = struct {
    batch_size: usize = 256,
    alignment: usize = 32,
    precision: enum { f32, f64 } = .f32,
    use_fma: bool = true,
};

/// SIMD-optimized wave pattern processor
pub const WaveSIMD = struct {
    const Self = @This();

    allocator: mem.Allocator,
    config: WaveConfig,
    aligned_buffer: []align(32) u8,
    wave_table: []align(32) f32,

    /// Initialize wave processor with configuration
    pub fn init(allocator: mem.Allocator, config: WaveConfig) !*Self {
        const self = try allocator.create(Self);
        errdefer allocator.destroy(self);

        self.allocator = allocator;
        self.config = config;

        // Allocate aligned buffers
        self.aligned_buffer = try allocator.alignedAlloc(u8, config.alignment, config.batch_size * 32);
        errdefer allocator.free(self.aligned_buffer);

        // Initialize wave lookup table
        self.wave_table = try allocator.alignedAlloc(f32, config.alignment, 1024);
        errdefer allocator.free(self.wave_table);
        try self.initWaveTable();

        return self;
    }

    /// Clean up resources
    pub fn deinit(self: *Self) void {
        self.allocator.free(self.aligned_buffer);
        self.allocator.free(self.wave_table);
        self.allocator.destroy(self);
    }

    /// Initialize wave lookup table with sine patterns
    fn initWaveTable(self: *Self) !void {
        var i: usize = 0;
        while (i < 1024) : (i += 1) {
            const phase = @intToFloat(f32, i) * 2.0 * math.pi / 1024.0;
            self.wave_table[i] = math.sin(phase);
        }
    }

    /// Process wave pattern with SIMD operations
    pub fn processWavePattern(self: *Self, data: []f32, pattern: []const f32) !void {
        if (data.len != pattern.len or data.len % 8 != 0) {
            return WaveError.InvalidDimensions;
        }

        var i: usize = 0;
        while (i < data.len) : (i += 8) {
            const wave_vec = self.loadAlignedVector(pattern[i..]);
            const data_vec = self.loadAlignedVector(data[i..]);
            const result = self.applyWavePattern(wave_vec, data_vec);
            try self.storeAlignedVector(data[i..], result);
        }
    }

    /// Apply harmonic resonance using SIMD
    pub fn applyHarmonicResonance(self: *Self, data: []f32, frequency: f32) !void {
        const vec_freq = Vec4f32{ frequency, frequency, frequency, frequency };
        var i: usize = 0;
        while (i < data.len) : (i += 4) {
            const phase_vec = self.calculatePhaseVector(i, vec_freq);
            const resonance = self.calculateResonance(phase_vec);
            try self.storeAlignedVector(data[i..], resonance);
        }
    }

    /// Calculate wave interference patterns using SIMD
    pub fn calculateInterference(self: *Self, wave1: []const f32, wave2: []const f32, result: []f32) !void {
        if (wave1.len != wave2.len or wave1.len != result.len or wave1.len % 8 != 0) {
            return WaveError.InvalidDimensions;
        }

        var i: usize = 0;
        while (i < wave1.len) : (i += 8) {
            const v1 = self.loadAlignedVector(wave1[i..]);
            const v2 = self.loadAlignedVector(wave2[i..]);
            const interference = self.computeInterference(v1, v2);
            try self.storeAlignedVector(result[i..], interference);
        }
    }

    /// SIMD-optimized wave pattern application
    fn applyWavePattern(self: *Self, wave: Vec8f32, data: Vec8f32) Vec8f32 {
        if (self.config.use_fma) {
            return @mulAdd(Vec8f32, wave, data, data);
        } else {
            return wave * data;
        }
    }

    /// Calculate phase vector for resonance
    fn calculatePhaseVector(self: *Self, offset: usize, frequency: Vec4f32) Vec4f32 {
        const time_vec = Vec4f32{
            @intToFloat(f32, offset),
            @intToFloat(f32, offset + 1),
            @intToFloat(f32, offset + 2),
            @intToFloat(f32, offset + 3),
        };
        return time_vec * frequency;
    }

    /// Calculate resonance using wave table lookup
    fn calculateResonance(self: *Self, phase: Vec4f32) Vec4f32 {
        const table_indices = @floatToInt(Vec4i32, @mod(phase, 1024.0));
        return Vec4f32{
            self.wave_table[@intCast(usize, table_indices[0])],
            self.wave_table[@intCast(usize, table_indices[1])],
            self.wave_table[@intCast(usize, table_indices[2])],
            self.wave_table[@intCast(usize, table_indices[3])],
        };
    }

    /// Compute wave interference pattern
    fn computeInterference(self: *Self, wave1: Vec8f32, wave2: Vec8f32) Vec8f32 {
        const product = wave1 * wave2;
        const phase_diff = @abs(wave1 - wave2);
        return product * (Vec8f32{1.0} + phase_diff);
    }

    /// Load aligned vector from memory
    fn loadAlignedVector(self: *Self, data: []const f32) Vec8f32 {
        const ptr = @ptrCast(*const [8]f32, data.ptr);
        return ptr.*;
    }

    /// Store aligned vector to memory
    fn storeAlignedVector(self: *Self, data: []f32, vector: Vec8f32) !void {
        if (@ptrToInt(data.ptr) % @sizeOf(Vec8f32) != 0) {
            return WaveError.AlignmentError;
        }
        const ptr = @ptrCast(*[8]f32, data.ptr);
        ptr.* = vector;
    }

    /// Process crystal harmonics using SIMD
    pub fn processCrystalHarmonics(self: *Self, data: []f32, harmonics: []const f32) !void {
        if (data.len % 8 != 0 or harmonics.len == 0) {
            return WaveError.InvalidDimensions;
        }

        var i: usize = 0;
        while (i < data.len) : (i += 8) {
            var acc = Vec8f32{ 0, 0, 0, 0, 0, 0, 0, 0 };

            // Apply each harmonic
            for (harmonics) |harmonic| {
                const phase = self.calculatePhaseVector(i, Vec4f32{ harmonic, harmonic, harmonic, harmonic });
                const harmonic_wave = self.calculateResonance(phase);
                acc += harmonic_wave;
            }

            // Normalize and store
            acc = acc * @splat(8, @as(f32, 1.0 / @intToFloat(f32, harmonics.len)));
            try self.storeAlignedVector(data[i..], acc);
        }
    }
};

test "wave-simd-basic" {
    const testing = std.testing;
    const allocator = testing.allocator;

    const config = WaveConfig{};
    var processor = try WaveSIMD.init(allocator, config);
    defer processor.deinit();

    var data = try allocator.alignedAlloc(f32, 32, 256);
    defer allocator.free(data);

    var pattern = try allocator.alignedAlloc(f32, 32, 256);
    defer allocator.free(pattern);

    // Initialize test data
    @memset(data, 1.0);
    @memset(pattern, 0.5);

    try processor.processWavePattern(data, pattern);

    // Verify results
    for (data) |value| {
        try testing.expectApproxEqAbs(value, 1.5, 0.0001);
    }
}
