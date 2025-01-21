//! Crystal Harmony System
//! Created: 2025-01-21 04:30:54 UTC
//! Author: isdood
//! 
//! Manages the harmony state of crystal structures within the
//! shattered cache system, ensuring stable resonance patterns
//! and optimal energy flow.

const std = @import("std");
const math = std.math;
const assert = std.debug.assert;

/// Error types specific to harmony operations
pub const HarmonyError = error{
    HarmonyLost,
    ResonanceMismatch,
    CrystalInstability,
    PatternDisruption,
};

/// Vector representation for harmony components
pub const HarmonyVector = struct {
    x: f64, // Material plane
    y: f64, // Energy plane
    z: f64, // Time plane
    w: f64, // Resonance plane

    const Self = @This();

    pub fn init() Self {
        return .{
            .x = 1.0,
            .y = 1.0,
            .z = 1.0,
            .w = 1.0,
        };
    }

    pub fn magnitude(self: Self) f64 {
        return math.sqrt(
            self.x * self.x +
            self.y * self.y +
            self.z * self.z +
            self.w * self.w
        );
    }

    pub fn normalize(self: Self) Self {
        const mag = self.magnitude();
        if (mag == 0.0) return self;
        
        return .{
            .x = self.x / mag,
            .y = self.y / mag,
            .z = self.z / mag,
            .w = self.w / mag,
        };
    }
};

/// Constants for harmony calculations
const HARMONY_CONSTANTS = struct {
    /// Golden ratio (φ)
    const PHI: f64 = 1.618033988749895;
    /// Base resonance frequency (Hz)
    const BASE_FREQUENCY: f64 = 432.0;
    /// Harmony decay rate
    const DECAY_RATE: f64 = 0.01;
    /// Maximum harmony instability
    const MAX_INSTABILITY: f64 = 0.2;
    /// Minimum viable harmony
    const MIN_HARMONY: f64 = 0.5;
};

/// Represents the current state of crystal harmony
pub const HarmonyState = struct {
    /// Current harmony vector
    vector: HarmonyVector,
    /// Overall harmony level
    level: f64,
    /// Current resonance frequency
    frequency: f64,
    /// Stability metric
    stability: f64,
    /// Pattern strength
    pattern_strength: f64,
    /// Last evolution timestamp
    last_evolution: i64,

    const Self = @This();

    /// Initialize a new harmony state
    pub fn init() !Self {
        return Self{
            .vector = HarmonyVector.init(),
            .level = 1.0,
            .frequency = HARMONY_CONSTANTS.BASE_FREQUENCY,
            .stability = 1.0,
            .pattern_strength = 1.0,
            .last_evolution = std.time.milliTimestamp(),
        };
    }

    /// Evolve the harmony state based on time and frequency
    pub fn evolve(self: *Self, base_freq: f64) !void {
        const current_time = std.time.milliTimestamp();
        const time_delta = @intToFloat(f64, current_time - self.last_evolution) / 1000.0;

        // Natural harmony decay
        const decay = math.exp(-HARMONY_CONSTANTS.DECAY_RATE * time_delta);
        self.level *= decay;

        // Frequency resonance
        const freq_ratio = base_freq / HARMONY_CONSTANTS.BASE_FREQUENCY;
        const resonance_factor = math.sin(freq_ratio * math.pi * 2.0 * time_delta);
        
        // Update harmony vector
        self.vector = .{
            .x = self.vector.x * decay * (1.0 + resonance_factor * 0.1),
            .y = self.vector.y * decay * (1.0 + resonance_factor * 0.15),
            .z = self.vector.z * decay * (1.0 + resonance_factor * 0.05),
            .w = self.vector.w * decay * (1.0 + resonance_factor * 0.2),
        };

        // Update stability
        self.stability = @maximum(
            0.0,
            self.stability + resonance_factor * 0.1 * HARMONY_CONSTANTS.PHI
        );

        // Check for harmony loss
        if (self.level < HARMONY_CONSTANTS.MIN_HARMONY) {
            return HarmonyError.HarmonyLost;
        }

        // Update pattern strength
        self.pattern_strength = math.pow(f64, self.level, HARMONY_CONSTANTS.PHI);
        
        self.last_evolution = current_time;
    }

    /// Get temporal component of harmony
    pub fn temporalComponent(self: Self) f64 {
        return (self.vector.z + self.vector.w) / 2.0;
    }

    /// Get spatial component of harmony
    pub fn spatialComponent(self: Self) f64 {
        return (self.vector.x + self.vector.y) / 2.0;
    }

    /// Apply resonance pattern
    pub fn applyResonance(self: *Self, frequency: f64) !void {
        const ratio = frequency / self.frequency;
        if (math.fabs(1.0 - ratio) > HARMONY_CONSTANTS.MAX_INSTABILITY) {
            return HarmonyError.ResonanceMismatch;
        }

        self.frequency = frequency;
        self.level *= math.pow(f64, HARMONY_CONSTANTS.PHI, -math.fabs(1.0 - ratio));
        try self.evolve(frequency);
    }

    /// Get overall harmony metric
    pub fn getHarmonyMetric(self: Self) f64 {
        return (self.level * 
                self.stability * 
                self.pattern_strength * 
                self.vector.magnitude()) / 4.0;
    }

    /// Check if harmony is stable
    pub fn isStable(self: Self) bool {
        return self.stability >= 0.8 and 
               self.level >= HARMONY_CONSTANTS.MIN_HARMONY and
               self.pattern_strength >= 0.7;
    }
};

test "HarmonyState initialization" {
    const testing = std.testing;
    var state = try HarmonyState.init();

    try testing.expectEqual(state.level, 1.0);
    try testing.expectEqual(state.frequency, HARMONY_CONSTANTS.BASE_FREQUENCY);
    try testing.expect(state.isStable());
}

test "HarmonyState evolution" {
    const testing = std.testing;
    var state = try HarmonyState.init();

    try state.evolve(HARMONY_CONSTANTS.BASE_FREQUENCY);
    try testing.expect(state.level <= 1.0);
    try testing.expect(state.level > HARMONY_CONSTANTS.MIN_HARMONY);
}

test "HarmonyState resonance" {
    const testing = std.testing;
    var state = try HarmonyState.init();

    // Test resonance within stability bounds
    try state.applyResonance(HARMONY_CONSTANTS.BASE_FREQUENCY * 1.1);
    try testing.expect(state.isStable());

    // Test resonance mismatch
    const result = state.applyResonance(HARMONY_CONSTANTS.BASE_FREQUENCY * 2.0);
    try testing.expectError(HarmonyError.ResonanceMismatch, result);
}
