//! Facet Crystal Resonance Manager
//! Author: @isdood
//! Created: 2025-01-21 12:57:30 UTC

const std = @import("std");
const lattice = @import("lattice.zig");
const types = @import("../core/types.zig");

const CrystalLattice = lattice.CrystalLattice;
const Result = types.Result;

/// Crystal resonance configuration
pub const ResonanceConfig = struct {
    /// Base resonance frequency
    base_frequency: f64 = 432.0,
    /// Crystal attunement threshold
    attunement_threshold: f64 = 0.87,
    /// Resonance stability factor
    stability_factor: f64 = 0.93,
    /// Enable harmonic overtones
    enable_overtones: bool = true,
    /// Number of resonance nodes
    node_count: u8 = 7,
};

/// Crystal resonance patterns
pub const ResonancePattern = enum {
    /// Clear, pristine resonance
    Pure,
    /// Flowing, dynamic resonance
    Flowing,
    /// Deep, grounding resonance
    Earthen,
    /// High, ethereal resonance
    Celestial,
    /// Perfect crystal resonance
    Prismatic,

    /// Get pattern frequency modifier
    pub fn frequency(self: ResonancePattern) f64 {
        return switch (self) {
            .Pure => 1.0,
            .Flowing => 1.618, // Golden ratio
            .Earthen => 0.882, // Earth frequency
            .Celestial => 2.0, // Octave up
            .Prismatic => 2.718, // e (natural resonance)
        };
    }
};

/// Resonance node in crystal structure
const ResonanceNode = struct {
    frequency: f64,
    amplitude: f64,
    phase: f64,
    stability: f64,
};

/// Crystal resonance manager
pub const Resonance = struct {
    config: ResonanceConfig,
    pattern: ResonancePattern,
    nodes: std.ArrayList(ResonanceNode),
    current_frequency: f64,
    stability: f64,
    crystal_lattice: *CrystalLattice,

    const Self = @This();

    /// Initialize new resonance manager
    pub fn init(crystal_lattice: *CrystalLattice, config: ?ResonanceConfig) !*Self {
        const resonance = try std.heap.page_allocator.create(Self);

        resonance.* = .{
            .config = config orelse ResonanceConfig{},
            .pattern = .Pure,
            .nodes = std.ArrayList(ResonanceNode).init(std.heap.page_allocator),
            .current_frequency = 0.0,
            .stability = 1.0,
            .crystal_lattice = crystal_lattice,
        };

        // Initialize resonance nodes
        try resonance.initNodes();

        return resonance;
    }

    /// Clean up resonance resources
    pub fn deinit(self: *Self) void {
        self.nodes.deinit();
        std.heap.page_allocator.destroy(self);
    }

    /// Initialize resonance nodes
    fn initNodes(self: *Self) !void {
        var i: u8 = 0;
        while (i < self.config.node_count) : (i += 1) {
            const node = ResonanceNode{
                .frequency = self.config.base_frequency * (1.0 + @intToFloat(f64, i) * 0.5),
                .amplitude = 1.0 / @intToFloat(f64, i + 1),
                .phase = 0.0,
                .stability = 1.0,
            };
            try self.nodes.append(node);
        }
    }

    /// Apply resonance patterns to crystal
    pub fn applyResonance(self: *Self, result: *Result) !void {
        // Select appropriate resonance pattern
        self.pattern = self.selectPattern(result);

        // Apply base frequency modulation
        self.current_frequency = self.config.base_frequency * self.pattern.frequency();

        // Process resonance nodes
        try self.processNodes();

        // Calculate final resonance stability
        self.calculateStability();

        // Update result with resonance metrics
        result.resonance = self.getResonanceLevel();
        result.clarity *= self.stability;
    }

    /// Process resonance nodes
    fn processNodes(self: *Self) !void {
        const clarity = self.crystal_lattice.clarity;

        for (self.nodes.items) |*node| {
            // Adjust node frequency based on pattern
            node.frequency *= self.pattern.frequency();

            // Apply crystal clarity to amplitude
            node.amplitude *= clarity;

            // Update stability based on resonance
            node.stability = @min(1.0, node.stability * self.config.stability_factor);
        }

        // Apply harmonic overtones if enabled
        if (self.config.enable_overtones) {
            try self.processOvertones();
        }
    }

    /// Process harmonic overtones
    fn processOvertones(self: *Self) !void {
        const base_freq = self.current_frequency;

        for (self.nodes.items) |*node| {
            // Generate harmonic overtones
            const overtone_freq = base_freq * node.frequency;
            node.frequency = (node.frequency + overtone_freq) * 0.5;

            // Adjust amplitude for harmonics
            node.amplitude *= self.pattern.frequency();
        }
    }

    /// Calculate overall stability
    fn calculateStability(self: *Self) void {
        var total_stability: f64 = 0.0;

        for (self.nodes.items) |node| {
            total_stability += node.stability;
        }

        self.stability = total_stability / @intToFloat(f64, self.nodes.items.len);
    }

    /// Select resonance pattern based on crystal state
    fn selectPattern(self: *Self, result: *const Result) ResonancePattern {
        const clarity = self.crystal_lattice.clarity;

        return if (clarity >= 0.98) .Prismatic
        else if (clarity >= 0.95) .Celestial
            else if (clarity >= 0.90) .Flowing
                else if (clarity >= 0.85) .Earthen
                    else .Pure;
    }

    /// Get current resonance level
    pub fn getResonanceLevel(self: *const Self) f64 {
        var total_resonance: f64 = 0.0;

        for (self.nodes.items) |node| {
            total_resonance += node.amplitude * node.stability;
        }

        return @min(1.0, total_resonance / @intToFloat(f64, self.nodes.items.len));
    }

    /// Get resonance metrics
    pub fn getMetrics(self: *const Self) struct {
        frequency: f64,
        pattern: ResonancePattern,
        stability: f64,
        node_count: usize,
    } {
        return .{
            .frequency = self.current_frequency,
            .pattern = self.pattern,
            .stability = self.stability,
            .node_count = self.nodes.items.len,
        };
    }
};

test "resonance_basic" {
    var lattice = try CrystalLattice.init(.{
        .clarity = 0.95,
        .facets = 3,
    });
    defer lattice.deinit();

    var resonance = try Resonance.init(&lattice, null);
    defer resonance.deinit();

    var result = Result{
        .value = 42.0,
        .resonance = 0.0,
        .clarity = 0.95,
    };

    try resonance.applyResonance(&result);
    try std.testing.expect(result.resonance > 0.0);
}

test "resonance_patterns" {
    var lattice = try CrystalLattice.init(.{
        .clarity = 1.0,
        .facets = 4,
    });
    defer lattice.deinit();

    var resonance = try Resonance.init(&lattice, null);
    defer resonance.deinit();

    var result = Result{
        .value = 42.0,
        .resonance = 0.0,
        .clarity = 1.0,
    };

    try resonance.applyResonance(&result);
    try std.testing.expect(resonance.pattern == .Prismatic);
}
