//! interference.zig - Crystal interference pattern management for Prism
//! Created by: isdood
//! Date: 2025-01-21 10:48:41 UTC

const std = @import("std");
const math = std.math;
const Allocator = std.mem.Allocator;
const AutoHashMap = std.AutoHashMap;

const lattice = @import("lattice.zig");
const resonance = @import("resonance.zig");

/// Interference pattern errors
pub const InterferenceError = error{
    PatternOverflow,
    DestructiveInterference,
    ResonanceMismatch,
    StructureDestabilized,
    EnergyImbalance,
};

/// Represents a point of interference in the crystal structure
pub const InterferenceNode = struct {
    position: [3]f64,
    intensity: f64,
    phase: f64,
    stability: f64,

    pub fn init(pos: [3]f64, intensity: f64, phase: f64) InterferenceNode {
        return .{
            .position = pos,
            .intensity = intensity,
            .phase = phase,
            .stability = 1.0,
        };
    }

    /// Calculate interference strength at a given point
    pub fn strengthAt(self: InterferenceNode, point: [3]f64) f64 {
        const distance = calculateDistance(self.position, point);
        return self.intensity * math.exp(-distance) * self.stability;
    }
};

/// Manages interference patterns within the crystal structure
pub const InterferencePattern = struct {
    allocator: Allocator,
    nodes: AutoHashMap([3]f64, InterferenceNode),
    total_energy: f64,
    stability_threshold: f64,
    resonance_state: resonance.ResonanceState,

    const Self = @This();

    /// Initialize a new interference pattern
    pub fn init(allocator: Allocator) !*Self {
        const self = try allocator.create(Self);
        self.* = .{
            .allocator = allocator,
            .nodes = AutoHashMap([3]f64, InterferenceNode).init(allocator),
            .total_energy = 0,
            .stability_threshold = 0.85,
            .resonance_state = resonance.ResonanceState.init(),
        };
        return self;
    }

    /// Clean up resources
    pub fn deinit(self: *Self) void {
        self.nodes.deinit();
        self.allocator.destroy(self);
    }

    /// Add a new interference node
    pub fn addNode(self: *Self, pos: [3]f64, intensity: f64, phase: f64) !void {
        const node = InterferenceNode.init(pos, intensity, phase);
        
        try self.validateAddition(node);
        try self.nodes.put(pos, node);
        
        self.total_energy += intensity;
        try self.updateResonance();
    }

    /// Calculate total interference at a point
    pub fn calculateInterference(self: Self, point: [3]f64) !f64 {
        var total: f64 = 0;
        var iterator = self.nodes.valueIterator();

        while (iterator.next()) |node| {
            const contribution = node.strengthAt(point);
            if (contribution < 0) {
                return InterferenceError.DestructiveInterference;
            }
            total += contribution;
        }

        if (total > self.stability_threshold) {
            return InterferenceError.PatternOverflow;
        }

        return total;
    }

    /// Optimize interference pattern for stability
    pub fn optimize(self: *Self) !void {
        var modified = false;
        var iterator = self.nodes.valueIterator();

        while (iterator.next()) |node| {
            if (node.stability < 0.8) {
                _ = try self.stabilizeNode(node);
                modified = true;
            }
        }

        if (modified) {
            try self.updateResonance();
        }
    }

    /// Stabilize a specific interference node
    fn stabilizeNode(self: *Self, node: *InterferenceNode) !bool {
        const original_stability = node.stability;
        
        // Attempt to increase stability through phase adjustment
        node.phase += math.pi / 8.0;
        node.stability += 0.1;

        if (node.stability > 1.0) {
            node.stability = 1.0;
        }

        try self.validatePattern();
        return node.stability > original_stability;
    }

    /// Update the resonance state based on current pattern
    fn updateResonance(self: *Self) !void {
        const pattern_stability = try self.calculatePatternStability();
        self.resonance_state.update(pattern_stability);

        if (!self.resonance_state.isStable()) {
            return InterferenceError.ResonanceMismatch;
        }
    }

    /// Calculate overall pattern stability
    fn calculatePatternStability(self: Self) !f64 {
        if (self.nodes.count() == 0) return 1.0;

        var total_stability: f64 = 0;
        var iterator = self.nodes.valueIterator();

        while (iterator.next()) |node| {
            total_stability += node.stability;
        }

        return total_stability / @intToFloat(f64, self.nodes.count());
    }

    /// Validate a new node addition
    fn validateAddition(self: Self, node: InterferenceNode) !void {
        // Check energy balance
        if (self.total_energy + node.intensity > 10.0) {
            return InterferenceError.EnergyImbalance;
        }

        // Check potential interference
        var iterator = self.nodes.valueIterator();
        while (iterator.next()) |existing| {
            const distance = calculateDistance(existing.position, node.position);
            if (distance < 0.1 and 
                math.fabs(existing.phase - node.phase) > math.pi / 2.0) {
                return InterferenceError.DestructiveInterference;
            }
        }
    }

    /// Validate entire interference pattern
    fn validatePattern(self: Self) !void {
        const stability = try self.calculatePatternStability();
        
        if (stability < self.stability_threshold) {
            return InterferenceError.StructureDestabilized;
        }
    }

    /// Get the current resonance level
    pub fn getResonanceLevel(self: Self) f64 {
        return self.resonance_state.getLevel();
    }
};

/// Calculate distance between two points in 3D space
fn calculateDistance(a: [3]f64, b: [3]f64) f64 {
    var sum: f64 = 0;
    for (a) |_, i| {
        const diff = a[i] - b[i];
        sum += diff * diff;
    }
    return math.sqrt(sum);
}

test "interference pattern basic functionality" {
    const allocator = std.testing.allocator;
    
    const pattern = try InterferencePattern.init(allocator);
    defer pattern.deinit();

    // Add test nodes
    try pattern.addNode(.{ 0, 0, 0 }, 1.0, 0.0);
    try pattern.addNode(.{ 1, 1, 1 }, 0.5, math.pi / 4.0);

    // Test interference calculation
    const interference = try pattern.calculateInterference(.{ 0.5, 0.5, 0.5 });
    try std.testing.expect(interference >= 0.0 and interference <= 2.0);

    // Test pattern optimization
    try pattern.optimize();
    try std.testing.expect(pattern.getResonanceLevel() >= 0.8);
}

test "interference node creation" {
    const node = InterferenceNode.init(.{ 1, 2, 3 }, 1.0, 0.0);
    try std.testing.expect(node.position[0] == 1.0);
    try std.testing.expect(node.intensity == 1.0);
    try std.testing.expect(node.stability == 1.0);
}

test "destructive interference detection" {
    const allocator = std.testing.allocator;
    
    const pattern = try InterferencePattern.init(allocator);
    defer pattern.deinit();

    try pattern.addNode(.{ 0, 0, 0 }, 1.0, 0.0);
    
    // This should fail due to destructive interference
    const result = pattern.addNode(.{ 0, 0, 0.1 }, 1.0, math.pi);
    try std.testing.expectError(InterferenceError.DestructiveInterference, result);
}
