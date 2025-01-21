//! Crystalline FFT implementation for wave pattern analysis
//! Created: 2025-01-21 13:53:26 UTC
//! Author: @isdood

const std = @import("std");
const math = std.math;
const mem = std.mem;
const Complex = std.math.Complex;
const Vector = std.meta.Vector;

pub const Error = error{
    InvalidDimensions,
    NotPowerOfTwo,
    OutOfMemory,
};

/// Crystal wave pattern configuration
pub const CrystalConfig = struct {
    sample_rate: f64 = 44100.0,
    harmonic_depth: usize = 7,
    resonance_threshold: f64 = 0.001,
    phase_alignment: f64 = math.pi / 4.0,
};

/// Optimized FFT for crystal lattice analysis
pub const CrystallineFFT = struct {
    const Self = @This();

    allocator: mem.Allocator,
    size: usize,
    config: CrystalConfig,
    twiddle_factors: []Complex(f64),
    scratch_buffer: []Complex(f64),
    bit_reverse: []usize,

    /// Initialize the FFT with given size
    pub fn init(allocator: mem.Allocator, size: usize, config: CrystalConfig) Error!*Self {
        if (!isPowerOfTwo(size)) {
            return Error.NotPowerOfTwo;
        }

        const self = try allocator.create(Self);
        errdefer allocator.destroy(self);

        self.allocator = allocator;
        self.size = size;
        self.config = config;

        // Allocate working buffers
        self.twiddle_factors = try allocator.alloc(Complex(f64), size);
        errdefer allocator.free(self.twiddle_factors);

        self.scratch_buffer = try allocator.alloc(Complex(f64), size);
        errdefer allocator.free(self.scratch_buffer);

        self.bit_reverse = try allocator.alloc(usize, size);
        errdefer allocator.free(self.bit_reverse);

        // Initialize twiddle factors
        try self.initTwiddleFactors();
        try self.initBitReverse();

        return self;
    }

    /// Clean up allocated resources
    pub fn deinit(self: *Self) void {
        self.allocator.free(self.twiddle_factors);
        self.allocator.free(self.scratch_buffer);
        self.allocator.free(self.bit_reverse);
        self.allocator.destroy(self);
    }

    /// Initialize twiddle factors for FFT
    fn initTwiddleFactors(self: *Self) !void {
        const theta = -2.0 * math.pi / @intToFloat(f64, self.size);
        var i: usize = 0;
        while (i < self.size) : (i += 1) {
            const angle = theta * @intToFloat(f64, i);
            self.twiddle_factors[i] = Complex(f64).init(
                math.cos(angle),
                                                        math.sin(angle),
            );
        }
    }

    /// Initialize bit-reverse mapping
    fn initBitReverse(self: *Self) !void {
        const bits = math.log2(self.size);
        var i: usize = 0;
        while (i < self.size) : (i += 1) {
            self.bit_reverse[i] = reverseBits(i, bits);
        }
    }

    /// Perform forward FFT on crystal wave data
    pub fn forward(self: *Self, data: []Complex(f64)) !void {
        if (data.len != self.size) {
            return Error.InvalidDimensions;
        }

        // Bit-reverse copy
        for (data) |_, i| {
            const j = self.bit_reverse[i];
            if (j > i) {
                const temp = data[i];
                data[i] = data[j];
                data[j] = temp;
            }
        }

        // Butterfly operations
        var step: usize = 1;
        while (step < self.size) : (step *= 2) {
            const jump = step * 2;
            var group: usize = 0;
            while (group < self.size) : (group += jump) {
                var pair: usize = 0;
                while (pair < step) : (pair += 1) {
                    const match = group + pair + step;
                    const product = data[match].mul(self.twiddle_factors[pair * self.size / jump]);
                    data[match] = data[group + pair].sub(product);
                    data[group + pair] = data[group + pair].add(product);
                }
            }
        }
    }

    /// Analyze crystal harmonics from FFT data
    pub fn analyzeHarmonics(self: *Self, data: []const Complex(f64)) ![]f64 {
        var harmonics = try self.allocator.alloc(f64, self.config.harmonic_depth);
        errdefer self.allocator.free(harmonics);

        const fundamental = self.findFundamentalFrequency(data);
        var i: usize = 0;
        while (i < self.config.harmonic_depth) : (i += 1) {
            harmonics[i] = self.measureHarmonicStrength(data, fundamental * @intToFloat(f64, i + 1));
        }

        return harmonics;
    }

    /// Find fundamental frequency in crystal pattern
    fn findFundamentalFrequency(self: *Self, data: []const Complex(f64)) f64 {
        var max_magnitude: f64 = 0;
        var fundamental_idx: usize = 1;

        var i: usize = 1;
        while (i < self.size / 2) : (i += 1) {
            const magnitude = data[i].magnitude();
            if (magnitude > max_magnitude) {
                max_magnitude = magnitude;
                fundamental_idx = i;
            }
        }

        return @intToFloat(f64, fundamental_idx) * self.config.sample_rate / @intToFloat(f64, self.size);
    }

    /// Measure strength of specific harmonic
    fn measureHarmonicStrength(self: *Self, data: []const Complex(f64), frequency: f64) f64 {
        const bin = @floatToInt(usize, frequency * @intToFloat(f64, self.size) / self.config.sample_rate);
        if (bin >= self.size / 2) {
            return 0;
        }

        var strength: f64 = 0;
        const window = 3; // Look at nearby bins
        var i: isize = -window;
        while (i <= window) : (i += 1) {
            const idx = @intCast(usize, @intCast(isize, bin) + i);
            if (idx < self.size / 2) {
                strength += data[idx].magnitude();
            }
        }

        return strength;
    }

    /// Detect resonance patterns in crystal data
    pub fn detectResonance(self: *Self, data: []const Complex(f64)) !bool {
        const harmonics = try self.analyzeHarmonics(data);
        defer self.allocator.free(harmonics);

        // Check harmonic ratios for resonance
        var resonance_score: f64 = 0;
        var i: usize = 1;
        while (i < harmonics.len) : (i += 1) {
            const ratio = harmonics[i] / harmonics[0];
            const expected = @intToFloat(f64, i + 1);
            resonance_score += math.fabs(ratio - expected);
        }

        return resonance_score < self.config.resonance_threshold;
    }

    /// Apply phase alignment to crystal pattern
    pub fn alignPhase(self: *Self, data: []Complex(f64)) !void {
        const phase_correction = Complex(f64).init(
            math.cos(self.config.phase_alignment),
                                                   math.sin(self.config.phase_alignment),
        );

        for (data) |*value| {
            value.* = value.mul(phase_correction);
        }
    }
};

/// Check if number is power of two
fn isPowerOfTwo(n: usize) bool {
    return n > 0 and (n & (n - 1)) == 0;
}

/// Reverse bits for given value
fn reverseBits(value: usize, bits: usize) usize {
    var v = value;
    var r: usize = 0;
    var i: usize = 0;
    while (i < bits) : (i += 1) {
        r = (r << 1) | (v & 1);
        v >>= 1;
    }
    return r;
}

/// SIMD-optimized complex multiplication
fn complexMultiplyVec(comptime T: type, a: Vector(4, T), b: Vector(4, T)) Vector(4, T) {
    const ar = @shuffle(T, a, undefined, [4]i32{ 0, 0, 2, 2 });
    const ai = @shuffle(T, a, undefined, [4]i32{ 1, 1, 3, 3 });
    const br = @shuffle(T, b, undefined, [4]i32{ 0, 2, 0, 2 });
    const bi = @shuffle(T, b, undefined, [4]i32{ 1, 3, 1, 3 });
    return ar * br - ai * bi;
}

test "crystalline-fft-basic" {
    const testing = std.testing;
    const allocator = testing.allocator;

    const size = 1024;
    const config = CrystalConfig{};

    var fft = try CrystallineFFT.init(allocator, size, config);
    defer fft.deinit();

    var data = try allocator.alloc(Complex(f64), size);
    defer allocator.free(data);

    // Initialize test data
    for (data) |*value, i| {
        const t = @intToFloat(f64, i) / @intToFloat(f64, size);
        value.* = Complex(f64).init(
            math.sin(2 * math.pi * 440.0 * t),
                                    0,
        );
    }

    try fft.forward(data);
    const resonant = try fft.detectResonance(data);
    try testing.expect(resonant);
}
