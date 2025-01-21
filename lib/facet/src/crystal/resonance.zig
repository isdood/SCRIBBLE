//! Crystal Resonance Network
//! Author: @isdood
//! Created: 2025-01-21 15:58:34 UTC

const std = @import("std");
const crystal_lattice = @import("lattice.zig");
const types = @import("../core/types.zig");

const CrystalLattice = crystal_lattice.CrystalLattice;
const Result = types.Result;

/// Resonance node configuration
pub const ResonanceConfig = struct {
    /// Base frequency for resonance calculations
    base_frequency: f64 = 440.0,
    /// Resonance stability threshold
    stability_threshold: f64 = 0.85,
    /// Enable harmonic patterns
    enable_harmonics: bool = true,
    /// Node count
    node_count: u8 = 8,
};

/// Resonance node structure
const ResonanceNode = struct {
    /// Node frequency
    frequency: f64,
    /// Node amplitude
    amplitude: f64,
    /// Node phase
    phase: f64,
    /// Node stability
    stability: f64,
};

/// Resonance pattern types
const ResonancePattern = enum {
    /// Standing wave pattern
    Standing,
    /// Traveling wave pattern
    Traveling,
    /// Harmonic series pattern
    Harmonic,
    /// Chaotic pattern
    Chaotic,
    /// Perfect resonance pattern
    Perfect,
};

/// Crystal resonance network
pub const ResonanceNetwork = struct {
    config: ResonanceConfig,
    nodes: std.ArrayList(ResonanceNode),
    pattern: ResonancePattern,
    stability: f64,
    allocator: std.mem.Allocator,

    const Self = @This();

    /// Initialize new resonance network
    pub fn init(config: ?ResonanceConfig) !*Self {
        const network = try std.heap.page_allocator.create(Self);

        network.* = .{
            .config = config orelse ResonanceConfig{},
            .nodes = std.ArrayList(ResonanceNode).init(std.heap.page_allocator),
            .pattern = .Standing,
            .stability = 0.0,
            .allocator = std.heap.page_allocator,
        };

        try network.initializeNodes();
        return network;
    }

    /// Clean up network resources
    pub fn deinit(self: *Self) void {
        self.nodes.deinit();
        std.heap.page_allocator.destroy(self);
    }

    /// Initialize resonance nodes
    fn initializeNodes(self: *Self) !void {
        // Clear existing nodes
        self.nodes.clearAndFree();

        // Create resonance nodes
        var i: u8 = 0;
        while (i < self.config.node_count) : (i += 1) {
            const node = ResonanceNode{
                .frequency = self.config.base_frequency * (1.0 + @as(f64, @floatFromInt(i)) * 0.5),
                .amplitude = 1.0,
                .phase = 0.0,
                .stability = 1.0,
            };
            try self.nodes.append(node);
        }

        self.stability = 1.0;
    }

    /// Update network state
    pub fn update(self: *Self) !void {
        var total_stability: f64 = 0.0;

        // Update each node
        for (self.nodes.items) |*node| {
            // Update phase
            node.phase = @mod(node.phase + node.frequency * 0.01, std.math.tau);

            // Calculate node stability using std.math.fabs
            node.stability = std.math.fabs(@sin(node.phase)) * node.amplitude;
            total_stability += node.stability;
        }

        // Update overall stability
        self.stability = total_stability / @as(f64, @floatFromInt(self.nodes.items.len));

        // Update resonance pattern
        self.pattern = self.selectPattern();
    }

    /// Select resonance pattern based on network state
    fn selectPattern(self: *Self) ResonancePattern {
        if (self.stability >= 0.99) return .Perfect;
        if (self.stability >= 0.9) return .Harmonic;
        if (self.stability >= 0.8) return .Standing;
        if (self.stability >= 0.7) return .Traveling;
        return .Chaotic;
    }

    /// Get current resonance level
    pub fn getResonance(self: *Self) f64 {
        var total_resonance: f64 = 0.0;

        for (self.nodes.items) |node| {
            total_resonance += node.stability * node.amplitude;
        }

        return @min(1.0, total_resonance / @as(f64, @floatFromInt(self.nodes.items.len)));
    }

    /// Get network metrics
    pub fn getMetrics(self: *const Self) struct {
        stability: f64,
        pattern: ResonancePattern,
        node_count: usize,
    } {
        return .{
            .stability = self.stability,
            .pattern = self.pattern,
            .node_count = self.nodes.items.len,
        };
    }
};

test "resonance_basic" {
    var test_crystal = try CrystalLattice.init(.{
        .clarity = 0.95,
        .facets = 3,
    });
    defer test_crystal.deinit();

    var network = try ResonanceNetwork.init(null);
    defer network.deinit();

    try network.update();
    const metrics = network.getMetrics();

    try std.testing.expect(metrics.stability > 0.0);
    try std.testing.expect(metrics.node_count > 0);
}

test "resonance_stability" {
    var test_crystal = try CrystalLattice.init(.{
        .clarity = 1.0,
        .facets = 4,
    });
    defer test_crystal.deinit();

    var network = try ResonanceNetwork.init(.{
        .base_frequency = 440.0,
        .stability_threshold = 0.9,
    });
    defer network.deinit();

    try network.update();
    const resonance = network.getResonance();

    try std.testing.expect(resonance > 0.0);
    try std.testing.expect(resonance <= 1.0);
}
