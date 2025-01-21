//! Crystal Resonator System
//! Created: 2025-01-21 04:35:14 UTC
//! Author: isdood
//! 
//! Manages crystal resonance patterns and harmonic frequencies
//! for the shattered cache system, ensuring stable energy flow
//! and optimal pattern maintenance.

const std = @import("std");
const math = std.math;
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

/// Error set for resonator operations
pub const ResonatorError = error{
    FrequencyOutOfRange,
    HarmonicInstability,
    ResonanceLost,
    PatternCollapse,
};

/// Constants for resonance calculations
const RESONANCE_CONSTANTS = struct {
    /// Base resonance frequency (A=432Hz)
    const BASE_FREQUENCY: f64 = 432.0;
    /// Golden ratio (φ)
    const PHI: f64 = 1.618033988749895;
    /// Maximum harmonic count
    const MAX_HARMONICS: usize = 12;
    /// Minimum stable frequency
    const MIN_FREQUENCY: f64 = 20.0;
    /// Maximum stable frequency
    const MAX_FREQUENCY: f64 = 20000.0;
};

/// Represents a harmonic pattern in the resonance field
const HarmonicPattern = struct {
    frequency: f64,
    amplitude: f64,
    phase: f64,
    stability: f64,

    pub fn init(freq: f64) HarmonicPattern {
        return .{
            .frequency = freq,
            .amplitude = 1.0,
            .phase = 0.0,
            .stability = 1.0,
        };
    }
};

/// The main resonator implementation
pub const Resonator = struct {
    /// Base frequency
    frequency: f64,
    /// Active harmonics
    harmonics: ArrayList(HarmonicPattern),
    /// Current stability
    stability: f64,
    /// Phase accumulator
    phase_acc: f64,
    /// Timestamp of last update
    last_update: i64,
    /// Memory allocator
    allocator: *Allocator,

    const Self = @This();

    /// Initialize a new resonator
    pub fn init(allocator: *Allocator, freq: f64) !*Self {
        if (freq < RESONANCE_CONSTANTS.MIN_FREQUENCY or 
            freq > RESONANCE_CONSTANTS.MAX_FREQUENCY) {
            return ResonatorError.FrequencyOutOfRange;
        }

        const resonator = try allocator.create(Self);
        errdefer allocator.destroy(resonator);

        resonator.* = .{
            .frequency = freq,
            .harmonics = ArrayList(HarmonicPattern).init(allocator),
            .stability = 1.0,
            .phase_acc = 0.0,
            .last_update = std.time.milliTimestamp(),
            .allocator = allocator,
        };

        // Initialize harmonic series
        try resonator.initHarmonics();

        return resonator;
    }

    /// Clean up resources
    pub fn deinit(self: *Self) void {
        self.harmonics.deinit();
        self.allocator.destroy(self);
    }

    /// Apply resonance to a value
    pub fn apply(self: *Self, input: f64) f64 {
        const now = std.time.milliTimestamp();
        const time_delta = @intToFloat(f64, now - self.last_update) / 1000.0;
        
        self.updatePhase(time_delta);
        self.last_update = now;

        var result = input;
        for (self.harmonics.items) |harmonic| {
            const harmonic_effect = harmonic.amplitude * 
                math.sin(harmonic.phase + self.phase_acc) *
                harmonic.stability;
            result *= 1.0 + (harmonic_effect * 0.1);
        }

        return math.clamp(result, 0.0, 1.0);
    }

    /// Get specific harmonic
    pub fn getHarmonic(self: Self, index: usize) f64 {
        if (index >= self.harmonics.items.len) return 0.0;
        return self.harmonics.items[index].amplitude;
    }

    /// Update resonator state
    pub fn update(self: *Self) !void {
        const now = std.time.milliTimestamp();
        const time_delta = @intToFloat(f64, now - self.last_update) / 1000.0;

        // Update phase accumulator
        self.updatePhase(time_delta);

        // Update harmonics
        for (self.harmonics.items) |*harmonic| {
            // Natural stability decay
            harmonic.stability *= math.exp(-0.1 * time_delta);
            
            // Adjust amplitude based on phase coherence
            const phase_factor = math.cos(harmonic.phase + self.phase_acc);
            harmonic.amplitude *= 1.0 + (phase_factor * 0.01);

            // Clamp values
            harmonic.amplitude = math.clamp(harmonic.amplitude, 0.1, 1.0);
            harmonic.stability = math.clamp(harmonic.stability, 0.5, 1.0);
        }

        // Update overall stability
        self.updateStability();

        if (self.stability < 0.5) {
            return ResonatorError.ResonanceLost;
        }

        self.last_update = now;
    }

    /// Get current stability level
    pub fn getStability(self: Self) f64 {
        return self.stability;
    }

    // Private methods

    fn initHarmonics(self: *Self) !void {
        var i: usize = 0;
        while (i < RESONANCE_CONSTANTS.MAX_HARMONICS) : (i += 1) {
            const harmonic_freq = self.frequency * @intToFloat(f64, i + 1);
            if (harmonic_freq > RESONANCE_CONSTANTS.MAX_FREQUENCY) break;
            
            const harmonic = HarmonicPattern.init(harmonic_freq);
            try self.harmonics.append(harmonic);
        }
    }

    fn updatePhase(self: *Self, time_delta: f64) void {
        self.phase_acc += 2.0 * math.pi * self.frequency * time_delta;
        self.phase_acc = @mod(self.phase_acc, 2.0 * math.pi);
    }

    fn updateStability(self: *Self) void {
        var total_stability: f64 = 0.0;
        const harmonic_count = @intToFloat(f64, self.harmonics.items.len);

        for (self.harmonics.items) |harmonic| {
            total_stability += harmonic.stability;
        }

        self.stability = total_stability / harmonic_count;
    }
};

test "Resonator initialization" {
    const testing = std.testing;
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    const resonator = try Resonator.init(allocator, 432.0);
    defer resonator.deinit();

    try testing.expectEqual(resonator.frequency, 432.0);
    try testing.expectEqual(resonator.stability, 1.0);
}

test "Resonator harmonics" {
    const testing = std.testing;
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    const resonator = try Resonator.init(allocator, 432.0);
    defer resonator.deinit();

    // Test harmonic generation
    try testing.expect(resonator.harmonics.items.len > 0);
    try testing.expect(resonator.harmonics.items.len <= RESONANCE_CONSTANTS.MAX_HARMONICS);

    // Test first harmonic
    const first_harmonic = resonator.harmonics.items[0];
    try testing.expectEqual(first_harmonic.frequency, 432.0);
    try testing.expectEqual(first_harmonic.amplitude, 1.0);
}

test "Resonance application" {
    const testing = std.testing;
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    const resonator = try Resonator.init(allocator, 432.0);
    defer resonator.deinit();

    const input_value = 0.5;
    const resonated = resonator.apply(input_value);
    try testing.expect(resonated >= 0.0 and resonated <= 1.0);
}
