//! Facet Crystal Lattice Manager
//! Author: @isdood
//! Created: 2025-01-21 12:58:33 UTC

const std = @import("std");
const resonance = @import("resonance.zig");
const types = @import("../core/types.zig");

const Result = types.Result;

/// Crystal lattice configuration
pub const LatticeConfig = struct {
    /// Crystal clarity level (0.0 - 1.0)
    clarity: f64 = 0.9,
    /// Number of crystal facets
    facets: u8 = 6,
    /// Lattice symmetry factor
    symmetry: f64 = 1.0,
    /// Enable prismatic dispersion
    enable_dispersion: bool = true,
    /// Sparkle factor
    sparkle_factor: f64 = 0.7,
};

/// Crystal facet structure
const Facet = struct {
    /// Facet angle
    angle: f64,
    /// Facet clarity
    clarity: f64,
    /// Resonance alignment
    alignment: f64,
    /// Sparkle intensity
    sparkle: f64,
};

/// Crystal lattice formation patterns
pub const LatticePattern = enum {
    /// Cubic crystal system
    Cubic,
    /// Hexagonal crystal system
    Hexagonal,
    /// Tetragonal crystal system
    Tetragonal,
    /// Orthorhombic crystal system
    Orthorhombic,
    /// Monoclinic crystal system
    Monoclinic,

    /// Get symmetry operations count
    pub fn symmetryCount(self: LatticePattern) u8 {
        return switch (self) {
            .Cubic => 48,
            .Hexagonal => 24,
            .Tetragonal => 16,
            .Orthorhombic => 8,
            .Monoclinic => 4,
        };
    }
};

/// Crystal lattice manager
pub const CrystalLattice = struct {
    config: LatticeConfig,
    pattern: LatticePattern,
    facets: std.ArrayList(Facet),
    clarity: f64,
    symmetry: f64,
    resonance_factor: f64,

    const Self = @This();

    /// Initialize new crystal lattice
    pub fn init(config: ?LatticeConfig) !*Self {
        const lattice = try std.heap.page_allocator.create(Self);

        lattice.* = .{
            .config = config orelse LatticeConfig{},
            .pattern = .Hexagonal, // Default to hexagonal system
            .facets = std.ArrayList(Facet).init(std.heap.page_allocator),
            .clarity = 0.0,
            .symmetry = 0.0,
            .resonance_factor = 1.0,
        };

        // Initialize crystal structure
        try lattice.initializeLattice();

        return lattice;
    }

    /// Clean up lattice resources
    pub fn deinit(self: *Self) void {
        self.facets.deinit();
        std.heap.page_allocator.destroy(self);
    }

    /// Initialize crystal lattice structure
    fn initializeLattice(self: *Self) !void {
        // Clear existing facets
        self.facets.clearAndFree();

        // Calculate base angle for facet distribution
        const base_angle = std.math.pi * 2.0 / @intToFloat(f64, self.config.facets);

        // Create facets
        var i: u8 = 0;
        while (i < self.config.facets) : (i += 1) {
            const facet = Facet{
                .angle = base_angle * @intToFloat(f64, i),
                .clarity = self.config.clarity,
                .alignment = 1.0,
                .sparkle = self.config.sparkle_factor,
            };
            try self.facets.append(facet);
        }

        // Initialize lattice properties
        self.clarity = self.config.clarity;
        self.symmetry = self.config.symmetry;
        self.updateResonanceFactor();
    }

    /// Update resonance factor based on lattice state
    fn updateResonanceFactor(self: *Self) void {
        const clarity_factor = self.clarity * self.symmetry;
        const pattern_factor = @intToFloat(f64, self.pattern.symmetryCount()) / 48.0;
        self.resonance_factor = clarity_factor * pattern_factor;
    }

    /// Attune crystal lattice
    pub fn attune(self: *Self, resonance: f64) !void {
        // Apply resonance to each facet
        for (self.facets.items) |*facet| {
            facet.alignment = @min(1.0, facet.alignment * resonance);
            facet.clarity = @min(1.0, facet.clarity * facet.alignment);

            // Update sparkle based on alignment
            if (facet.alignment > 0.95) {
                facet.sparkle = @min(1.0, facet.sparkle * 1.1);
            }
        }

        // Update overall lattice properties
        self.updateLatticeState();
    }

    /// Apply prismatic dispersion
    pub fn applyDispersion(self: *Self) !void {
        if (!self.config.enable_dispersion) return;

        for (self.facets.items) |*facet| {
            // Calculate dispersion effect
            const dispersion = @sin(facet.angle) * self.clarity;
            facet.clarity = @min(1.0, facet.clarity * (1.0 + dispersion * 0.1));
        }

        self.updateLatticeState();
    }

    /// Update overall lattice state
    fn updateLatticeState(self: *Self) void {
        var total_clarity: f64 = 0.0;
        var total_alignment: f64 = 0.0;

        for (self.facets.items) |facet| {
            total_clarity += facet.clarity;
            total_alignment += facet.alignment;
        }

        const facet_count = @intToFloat(f64, self.facets.items.len);
        self.clarity = total_clarity / facet_count;
        self.symmetry = total_alignment / facet_count;

        self.updateResonanceFactor();
    }

    /// Get lattice metrics
    pub fn getMetrics(self: *const Self) struct {
        clarity: f64,
        symmetry: f64,
        resonance_factor: f64,
        facet_count: usize,
        pattern: LatticePattern,
    } {
        return .{
            .clarity = self.clarity,
            .symmetry = self.symmetry,
            .resonance_factor = self.resonance_factor,
            .facet_count = self.facets.items.len,
            .pattern = self.pattern,
        };
    }

    /// Check if lattice is perfectly aligned
    pub fn isPerfect(self: *const Self) bool {
        return self.clarity >= 0.99 and self.symmetry >= 0.99;
    }
};

test "lattice_basic" {
    var lattice = try CrystalLattice.init(null);
    defer lattice.deinit();

    try std.testing.expect(lattice.facets.items.len > 0);
    try std.testing.expect(lattice.clarity > 0.0);
}

test "lattice_attunement" {
    var lattice = try CrystalLattice.init(.{
        .clarity = 0.95,
        .facets = 8,
    });
    defer lattice.deinit();

    try lattice.attune(1.1);
    const metrics = lattice.getMetrics();

    try std.testing.expect(metrics.clarity >= 0.95);
    try std.testing.expect(metrics.symmetry > 0.0);
}

test "lattice_dispersion" {
    var lattice = try CrystalLattice.init(.{
        .clarity = 1.0,
        .enable_dispersion = true,
    });
    defer lattice.deinit();

    const initial_clarity = lattice.clarity;
    try lattice.applyDispersion();

    try std.testing.expect(lattice.clarity != initial_clarity);
}
