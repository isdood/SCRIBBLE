//! Facet Core Types
//! Author: @isdood
//! Created: 2025-01-21 13:04:54 UTC

const std = @import("std");

/// Computation result with crystal metrics
pub const Result = struct {
    /// Computed value
    value: f64 = 0.0,
    /// Crystal resonance level
    resonance: f64 = 1.0,
    /// Crystal clarity
    clarity: f64 = 1.0,
    /// Sparkle indicator
    sparkle: bool = false,
    /// Computation timestamp
    timestamp: i64 = 0,
    /// Whimsy factor
    whimsy: f64 = 1.0,

    const Self = @This();

    /// Create new result
    pub fn init(value: f64) Self {
        return .{
            .value = value,
            .timestamp = std.time.timestamp(),
        };
    }

    /// Check if result is perfect
    pub fn isPerfect(self: Self) bool {
        return self.clarity >= 0.99 and
        self.resonance >= 0.99 and
        self.sparkle;
    }

    /// Get result quality metric
    pub fn getQuality(self: Self) f64 {
        return (self.clarity * self.resonance *
        (if (self.sparkle) 1.1 else 1.0) *
        self.whimsy);
    }
};

/// Crystal facet type
pub const Facet = enum(u8) {
    /// Primary computational facet
    Primary,
    /// Resonance amplification facet
    Resonance,
    /// Clarity enhancement facet
    Clarity,
    /// Whimsy generation facet
    Whimsy,
    /// Sparkle production facet
    Sparkle,

    /// Get facet efficiency multiplier
    pub fn efficiency(self: Facet) f64 {
        return switch (self) {
            .Primary => 1.0,
            .Resonance => 1.2,
            .Clarity => 1.15,
            .Whimsy => 1.3,
            .Sparkle => 1.25,
        };
    }
};

/// Crystal lattice symmetry
pub const Symmetry = enum {
    /// Cubic crystal system
    Cubic,
    /// Tetragonal crystal system
    Tetragonal,
    /// Hexagonal crystal system
    Hexagonal,
    /// Trigonal crystal system
    Trigonal,
    /// Orthorhombic crystal system
    Orthorhombic,

    /// Get symmetry operations count
    pub fn operationCount(self: Symmetry) u8 {
        return switch (self) {
            .Cubic => 48,
            .Tetragonal => 16,
            .Hexagonal => 24,
            .Trigonal => 12,
            .Orthorhombic => 8,
        };
    }
};

/// Resonance pattern type
pub const Pattern = enum {
    /// Serene computational pattern
    Serene,
    /// Vibrant resonance pattern
    Vibrant,
    /// Ethereal clarity pattern
    Ethereal,
    /// Mystical whimsy pattern
    Mystical,
    /// Radiant sparkle pattern
    Radiant,

    /// Get pattern strength multiplier
    pub fn strength(self: Pattern) f64 {
        return switch (self) {
            .Serene => 1.0,
            .Vibrant => 1.2,
            .Ethereal => 1.4,
            .Mystical => 1.6,
            .Radiant => 1.8,
        };
    }
};

/// Computation operation type
pub const Operation = enum {
    /// Basic arithmetic
    Arithmetic,
    /// Crystal resonance
    Resonance,
    /// Pattern matching
    Pattern,
    /// Whimsy generation
    Whimsy,
    /// State transition
    Transition,

    /// Get operation complexity factor
    pub fn complexity(self: Operation) f64 {
        return switch (self) {
            .Arithmetic => 1.0,
            .Resonance => 1.3,
            .Pattern => 1.5,
            .Whimsy => 1.7,
            .Transition => 1.4,
        };
    }
};

/// Crystal metrics bundle
pub const Metrics = struct {
    /// Crystal clarity level
    clarity: f64,
    /// Resonance strength
    resonance: f64,
    /// Pattern stability
    stability: f64,
    /// Whimsy factor
    whimsy: f64,
    /// Sparkle presence
    sparkle: bool,
    /// Computation timestamp
    timestamp: i64,

    /// Create new metrics
    pub fn init() Metrics {
        return .{
            .clarity = 1.0,
            .resonance = 1.0,
            .stability = 1.0,
            .whimsy = 1.0,
            .sparkle = false,
            .timestamp = std.time.timestamp(),
        };
    }

    /// Get overall quality metric
    pub fn quality(self: Metrics) f64 {
        return (self.clarity *
        self.resonance *
        self.stability *
        self.whimsy *
        (if (self.sparkle) 1.1 else 1.0));
    }
};

/// Crystal configuration bundle
pub const Config = struct {
    /// Base clarity level
    base_clarity: f64 = 0.9,
    /// Resonance threshold
    resonance_threshold: f64 = 0.87,
    /// Pattern stability factor
    stability_factor: f64 = 0.93,
    /// Whimsy generation rate
    whimsy_rate: f64 = 0.1,
    /// Sparkle threshold
    sparkle_threshold: f64 = 0.95,
};

test "result_basic" {
    const result = Result.init(42.0);
    try std.testing.expectEqual(result.value, 42.0);
    try std.testing.expect(result.timestamp > 0);
}

test "result_perfect" {
    var result = Result.init(42.0);
    result.clarity = 1.0;
    result.resonance = 1.0;
    result.sparkle = true;

    try std.testing.expect(result.isPerfect());
    try std.testing.expect(result.getQuality() > 1.0);
}

test "facet_efficiency" {
    try std.testing.expect(Facet.Whimsy.efficiency() > Facet.Primary.efficiency());
}

test "pattern_strength" {
    try std.testing.expect(Pattern.Radiant.strength() > Pattern.Serene.strength());
}

test "metrics_quality" {
    var metrics = Metrics.init();
    metrics.sparkle = true;

    try std.testing.expect(metrics.quality() > 1.0);
}
