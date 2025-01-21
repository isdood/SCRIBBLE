//! Facet Harmony Manager
//! Author: @isdood
//! Created: 2025-01-21 12:55:57 UTC

const std = @import("std");
const crystal = @import("../crystal/lattice.zig");
const types = @import("../core/types.zig");

const CrystalLattice = crystal.CrystalLattice;
const Result = types.Result;

/// Harmony configuration
pub const HarmonyConfig = struct {
    /// Base resonance level
    base_resonance: f64 = 0.87,
    /// Crystal attunement strength
    attunement_strength: f64 = 0.93,
    /// Whimsy factor
    whimsy_factor: f64 = 0.7,
    /// Enable harmonic resonance
    enable_harmonics: bool = true,
    /// Sparkle threshold
    sparkle_threshold: f64 = 0.95,
};

/// Harmonic patterns for crystal attunement
const HarmonicPattern = enum {
    Serene,
    Vibrant,
    Ethereal,
    Mystical,
    Radiant,

    /// Get pattern multiplier
    pub fn multiplier(self: HarmonicPattern) f64 {
        return switch (self) {
            .Serene => 1.0,
            .Vibrant => 1.2,
            .Ethereal => 1.5,
            .Mystical => 1.8,
            .Radiant => 2.0,
        };
    }
};

/// Harmony manager for crystal resonance
pub const Harmony = struct {
    config: HarmonyConfig,
    current_resonance: f64,
    attunement: f64,
    whimsy: f64,
    pattern: HarmonicPattern,
    crystal_lattice: *CrystalLattice,

    const Self = @This();

    /// Initialize new harmony manager
    pub fn init(crystal_lattice: *CrystalLattice, config: ?HarmonyConfig) !*Self {
        const harmony = try std.heap.page_allocator.create(Self);
        harmony.* = .{
            .config = config orelse HarmonyConfig{},
            .current_resonance = 0.0,
            .attunement = 1.0,
            .whimsy = 1.0,
            .pattern = .Serene,
            .crystal_lattice = crystal_lattice,
        };
        return harmony;
    }

    /// Clean up harmony resources
    pub fn deinit(self: *Self) void {
        std.heap.page_allocator.destroy(self);
    }

    /// Apply harmonic resonance to result
    pub fn applyHarmony(self: *Self, result: *Result) !void {
        // Initialize base resonance
        self.current_resonance = self.config.base_resonance;

        // Apply crystal attunement
        try self.attuneCrystals();

        // Apply harmonic patterns if enabled
        if (self.config.enable_harmonics) {
            try self.applyHarmonics(result);
        }

        // Add whimsy factor
        self.applyWhimsy();

        // Update result with final resonance
        result.resonance = self.current_resonance;

        // Check for sparkle threshold
        if (self.current_resonance >= self.config.sparkle_threshold) {
            result.sparkle = true;
        }
    }

    /// Attune crystals for resonance
    fn attuneCrystals(self: *Self) !void {
        const clarity = self.crystal_lattice.clarity;
        const attuned_resonance = self.current_resonance *
        (self.config.attunement_strength * clarity);

        self.current_resonance = @min(1.0, attuned_resonance);
        self.attunement *= clarity;
    }

    /// Apply harmonic patterns
    fn applyHarmonics(self: *Self, result: *Result) !void {
        // Select harmonic pattern based on calculation complexity
        self.pattern = self.selectPattern(result);

        // Apply pattern multiplier
        const harmonic_boost = self.pattern.multiplier() * self.attunement;
        self.current_resonance = @min(1.0, self.current_resonance * harmonic_boost);
    }

    /// Apply whimsy factor
    fn applyWhimsy(self: *Self) void {
        const whimsy_boost = 1.0 + (self.config.whimsy_factor * self.whimsy);
        self.current_resonance = @min(1.0, self.current_resonance * whimsy_boost);
    }

    /// Select appropriate harmonic pattern
    fn selectPattern(self: *Self, result: *const Result) HarmonicPattern {
        const clarity = self.crystal_lattice.clarity;

        return if (clarity >= 0.98) .Radiant
        else if (clarity >= 0.95) .Mystical
            else if (clarity >= 0.90) .Ethereal
                else if (clarity >= 0.85) .Vibrant
                    else .Serene;
    }

    /// Get current harmony metrics
    pub fn getMetrics(self: *const Self) struct {
        resonance: f64,
        attunement: f64,
        pattern: HarmonicPattern,
        whimsy: f64,
    } {
        return .{
            .resonance = self.current_resonance,
            .attunement = self.attunement,
            .pattern = self.pattern,
            .whimsy = self.whimsy,
        };
    }

    /// Boost resonance with whimsy
    pub fn boostWhimsy(self: *Self, factor: f64) void {
        self.whimsy = @min(1.0, self.whimsy * factor);
    }

    /// Check if resonance is at sparkle level
    pub fn isSparkly(self: *const Self) bool {
        return self.current_resonance >= self.config.sparkle_threshold;
    }
};

test "harmony_basic" {
    var lattice = try CrystalLattice.init(.{
        .clarity = 0.95,
        .facets = 3,
    });
    defer lattice.deinit();

    var harmony = try Harmony.init(&lattice, null);
    defer harmony.deinit();

    var result = Result{
        .value = 42.0,
        .resonance = 0.0,
        .clarity = 0.95,
        .sparkle = false,
    };

    try harmony.applyHarmony(&result);
    try std.testing.expect(result.resonance >= harmony.config.base_resonance);
}

test "harmony_sparkle" {
    var lattice = try CrystalLattice.init(.{
        .clarity = 1.0,
        .facets = 4,
    });
    defer lattice.deinit();

    var harmony = try Harmony.init(&lattice, .{
        .base_resonance = 0.9,
        .sparkle_threshold = 0.95,
    });
    defer harmony.deinit();

    var result = Result{
        .value = 42.0,
        .resonance = 0.0,
        .clarity = 1.0,
        .sparkle = false,
    };

    try harmony.applyHarmony(&result);
    try std.testing.expect(harmony.isSparkly());
    try std.testing.expect(result.sparkle);
}
