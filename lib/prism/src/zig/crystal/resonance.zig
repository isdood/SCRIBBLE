//! resonance.zig - Resonance management for Prism crystal structures
//! Created by: isdood
//! Date: 2025-01-21 10:52:15 UTC

const std = @import("std");
const math = std.math;
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

/// Resonance system errors
pub const ResonanceError = error{
    HarmonyLost,
    FrequencyMismatch,
    AmplitudeOverflow,
    PatternCollapse,
    ResonanceCascade,
};

/// Represents a harmonic frequency component
pub const Harmonic = struct {
    frequency: f64,
    amplitude: f64,
    phase: f64,
    weight: f64,

    pub fn init(freq: f64, amp: f64, ph: f64) Harmonic {
        return .{
            .frequency = freq,
            .amplitude = amp,
            .phase = ph,
            .weight = 1.0,
        };
    }

    /// Calculate the harmonic value at a given time
    pub fn valueAt(self: Harmonic, time: f64) f64 {
        return self.amplitude * 
               math.sin(self.frequency * time + self.phase) * 
               self.weight;
    }
};

/// Manages the resonance state of a crystal structure
pub const ResonanceState = struct {
    harmonics: ArrayList(Harmonic),
    base_frequency: f64,
    stability: f64,
    harmony_level: f64,

    const Self = @This();

    /// Initialize a new resonance state
    pub fn init() ResonanceState {
        return .{
            .harmonics = ArrayList(Harmonic).init(std.heap.page_allocator),
            .base_frequency = 1.0,
            .stability = 1.0,
            .harmony_level = 1.0,
        };
    }

    /// Clean up resources
    pub fn deinit(self: *Self) void {
        self.harmonics.deinit();
    }

    /// Add a new harmonic to the resonance state
    pub fn addHarmonic(self: *Self, freq: f64, amp: f64, phase: f64) !void {
        const harmonic = Harmonic.init(freq, amp, phase);
        try self.validateHarmonic(harmonic);
        try self.harmonics.append(harmonic);
        try self.updateHarmony();
    }

    /// Update the resonance state based on a stability factor
    pub fn update(self: *Self, stability_factor: f64) void {
        self.stability = stability_factor;
        self.harmony_level *= stability_factor;

        // Prevent harmony level from exceeding 1.0
        if (self.harmony_level > 1.0) {
            self.harmony_level = 1.0;
        }

        // Apply stability factor to all harmonics
        for (self.harmonics.items) |*harmonic| {
            harmonic.weight *= stability_factor;
            if (harmonic.weight > 1.0) harmonic.weight = 1.0;
        }
    }

    /// Get the current resonance level
    pub fn getLevel(self: Self) f64 {
        return self.harmony_level * self.stability;
    }

    /// Check if the resonance state is stable
    pub fn isStable(self: Self) bool {
        return self.harmony_level >= 0.8 and self.stability >= 0.8;
    }

    /// Calculate total resonance at a given time
    pub fn calculateResonance(self: Self, time: f64) !f64 {
        var total: f64 = 0;
        for (self.harmonics.items) |harmonic| {
            total += harmonic.valueAt(time);
        }

        if (math.fabs(total) > 10.0) {
            return ResonanceError.AmplitudeOverflow;
        }

        return total * self.harmony_level;
    }

    /// Optimize the resonance pattern
    pub fn optimize(self: *Self) !void {
        if (self.harmonics.items.len == 0) return;

        // Sort harmonics by frequency
        std.sort.sort(Harmonic, self.harmonics.items, {}, struct {
            fn lessThan(_: void, a: Harmonic, b: Harmonic) bool {
                return a.frequency < b.frequency;
            }
        }.lessThan);

        // Adjust phases for constructive interference
        var base_phase = self.harmonics.items[0].phase;
        for (self.harmonics.items[1..]) |*harmonic| {
            harmonic.phase = alignPhase(base_phase, harmonic.frequency);
            base_phase = harmonic.phase;
        }

        try self.validateResonance();
    }

    /// Validate a new harmonic before adding
    fn validateHarmonic(self: Self, harmonic: Harmonic) !void {
        // Check frequency ratio with base frequency
        const freq_ratio = harmonic.frequency / self.base_frequency;
        if (math.fabs(freq_ratio - math.round(freq_ratio)) > 0.1) {
            return ResonanceError.FrequencyMismatch;
        }

        // Check amplitude
        if (harmonic.amplitude > 2.0) {
            return ResonanceError.AmplitudeOverflow;
        }

        // Validate against existing harmonics
        for (self.harmonics.items) |existing| {
            if (math.fabs(existing.frequency - harmonic.frequency) < 0.1) {
                return ResonanceError.FrequencyMismatch;
            }
        }
    }

    /// Update harmony level based on current state
    fn updateHarmony(self: *Self) !void {
        const resonance_value = try self.calculateResonance(0);
        const harmony_factor = 1.0 / (1.0 + math.fabs(resonance_value));
        
        self.harmony_level *= harmony_factor;

        if (self.harmony_level < 0.5) {
            return ResonanceError.HarmonyLost;
        }
    }

    /// Validate overall resonance pattern
    fn validateResonance(self: Self) !void {
        var total_amplitude: f64 = 0;
        for (self.harmonics.items) |harmonic| {
            total_amplitude += harmonic.amplitude * harmonic.weight;
        }

        if (total_amplitude > 10.0) {
            return ResonanceError.ResonanceCascade;
        }

        if (self.harmony_level < 0.5) {
            return ResonanceError.PatternCollapse;
        }
    }
};

/// Align a phase with the base phase considering frequency ratio
fn alignPhase(base_phase: f64, frequency: f64) f64 {
    const phase_shift = math.pi / (4.0 * frequency);
    return math.mod(base_phase + phase_shift, math.tau);
}

test "resonance state basic functionality" {
    var state = ResonanceState.init();
    defer state.deinit();

    try state.addHarmonic(1.0, 1.0, 0.0);
    try state.addHarmonic(2.0, 0.5, math.pi / 4.0);

    try std.testing.expect(state.isStable());
    try std.testing.expect(state.getLevel() >= 0.8);
}

test "harmonic validation" {
    var state = ResonanceState.init();
    defer state.deinit();

    // Valid harmonic
    try state.addHarmonic(1.0, 1.0, 0.0);

    // Invalid frequency (should fail)
    const result = state.addHarmonic(1.1, 1.0, 0.0);
    try std.testing.expectError(ResonanceError.FrequencyMismatch, result);
}

test "resonance optimization" {
    var state = ResonanceState.init();
    defer state.deinit();

    try state.addHarmonic(1.0, 1.0, 0.0);
    try state.addHarmonic(2.0, 0.5, math.pi);

    try state.optimize();
    const resonance = try state.calculateResonance(0);
    try std.testing.expect(resonance >= -10.0 and resonance <= 10.0);
}
