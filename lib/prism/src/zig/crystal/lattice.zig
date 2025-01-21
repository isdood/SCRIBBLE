//! lattice.zig - Crystal lattice system for Prism async runtime
//! Created by: isdood
//! Date: 2025-01-21 10:50:13 UTC

const std = @import("std");
const math = std.math;
const Allocator = std.mem.Allocator;
const AutoHashMap = std.AutoHashMap;
const ArrayList = std.ArrayList;

const resonance = @import("resonance.zig");
const interference = @import("interference.zig");

/// Lattice system errors
pub const LatticeError = error{
    StructureViolation,
    SymmetryBroken,
    EnergyImbalance,
    ResonanceLost,
    NodeOverflow,
};

/// Crystal system types for lattice organization
pub const CrystalSystem = enum {
    Cubic,      // Most symmetric, highest stability
    Tetragonal, // Good for 4-way operations
    Hexagonal,  // Optimal for networking
    Orthorhombic, // Balanced stability
    Monoclinic,   // Flexible but less symmetric
    Triclinic,    // Lowest symmetry, most flexible
    Rhombohedral, // Compromise between cubic and hexagonal
};

/// Represents a node in the crystal lattice
pub const LatticeNode = struct {
    position: [3]f64,
    energy: f64,
    connections: ArrayList(*LatticeNode),
    resonance_state: resonance.ResonanceState,
    system: CrystalSystem,

    pub fn init(allocator: Allocator, pos: [3]f64, system: CrystalSystem) !*LatticeNode {
        const node = try allocator.create(LatticeNode);
        node.* = .{
            .position = pos,
            .energy = 1.0,
            .connections = ArrayList(*LatticeNode).init(allocator),
            .resonance_state = resonance.ResonanceState.init(),
            .system = system,
        };
        return node;
    }

    pub fn deinit(self: *LatticeNode) void {
        self.connections.deinit();
    }
};

/// Main crystal lattice structure
pub const Lattice = struct {
    allocator: Allocator,
    nodes: AutoHashMap([3]f64, *LatticeNode),
    interference_pattern: *interference.InterferencePattern,
    system: CrystalSystem,
    stability: f64,
    energy_total: f64,

    const Self = @This();

    /// Initialize a new crystal lattice
    pub fn init(allocator: Allocator) !*Self {
        const self = try allocator.create(Self);
        self.* = .{
            .allocator = allocator,
            .nodes = AutoHashMap([3]f64, *LatticeNode).init(allocator),
            .interference_pattern = try interference.InterferencePattern.init(allocator),
            .system = .Cubic, // Start with most stable system
            .stability = 1.0,
            .energy_total = 0,
        };
        return self;
    }

    /// Clean up resources
    pub fn deinit(self: *Self) void {
        var iterator = self.nodes.valueIterator();
        while (iterator.next()) |node| {
            node.deinit();
            self.allocator.destroy(node);
        }
        self.nodes.deinit();
        self.interference_pattern.deinit();
        self.allocator.destroy(self);
    }

    /// Add a new node to the lattice
    pub fn addNode(self: *Self, pos: [3]f64) !*LatticeNode {
        // Check if position is already occupied
        if (self.nodes.contains(pos)) {
            return LatticeError.NodeOverflow;
        }

        const node = try LatticeNode.init(self.allocator, pos, self.system);
        try self.nodes.put(pos, node);

        try self.validateStructure();
        try self.updateConnections(node);
        try self.maintainSymmetry();

        return node;
    }

    /// Create connections between nodes based on crystal system
    fn updateConnections(self: *Self, new_node: *LatticeNode) !void {
        var iterator = self.nodes.valueIterator();
        while (iterator.next()) |node| {
            if (node == new_node) continue;

            const distance = calculateDistance(node.position, new_node.position);
            const max_distance = switch (self.system) {
                .Cubic => 1.0,
                .Tetragonal => 1.2,
                .Hexagonal => 1.15,
                .Orthorhombic => 1.3,
                .Monoclinic => 1.4,
                .Triclinic => 1.5,
                .Rhombohedral => 1.1,
            };

            if (distance <= max_distance) {
                try node.connections.append(new_node);
                try new_node.connections.append(node);
            }
        }
    }

    /// Validate crystal structure integrity
    fn validateStructure(self: *Self) !void {
        const avg_connections = try self.calculateAverageConnections();
        const min_connections = switch (self.system) {
            .Cubic => 6,
            .Tetragonal => 4,
            .Hexagonal => 6,
            .Orthorhombic => 4,
            .Monoclinic => 3,
            .Triclinic => 2,
            .Rhombohedral => 5,
        };

        if (self.nodes.count() > 0 and avg_connections < @intToFloat(f64, min_connections)) {
            return LatticeError.StructureViolation;
        }
    }

    /// Calculate average number of connections per node
    fn calculateAverageConnections(self: Self) !f64 {
        if (self.nodes.count() == 0) return 0;

        var total_connections: usize = 0;
        var iterator = self.nodes.valueIterator();
        while (iterator.next()) |node| {
            total_connections += node.connections.items.len;
        }

        return @intToFloat(f64, total_connections) / @intToFloat(f64, self.nodes.count());
    }

    /// Maintain crystal symmetry
    fn maintainSymmetry(self: *Self) !void {
        const symmetry_factor = try self.calculateSymmetryFactor();
        if (symmetry_factor < 0.8) {
            return LatticeError.SymmetryBroken;
        }

        try self.interference_pattern.optimize();
    }

    /// Calculate symmetry factor based on node distribution
    fn calculateSymmetryFactor(self: Self) !f64 {
        if (self.nodes.count() < 2) return 1.0;

        var center = [_]f64{0} ** 3;
        var iterator = self.nodes.valueIterator();
        while (iterator.next()) |node| {
            for (node.position) |val, i| {
                center[i] += val;
            }
        }

        // Calculate center of mass
        for (center) |*val| {
            val.* /= @intToFloat(f64, self.nodes.count());
        }

        // Calculate average distance from center
        var total_deviation: f64 = 0;
        iterator = self.nodes.valueIterator();
        while (iterator.next()) |node| {
            total_deviation += calculateDistance(node.position, center);
        }

        const avg_deviation = total_deviation / @intToFloat(f64, self.nodes.count());
        return 1.0 / (1.0 + avg_deviation);
    }

    /// Get current lattice stability
    pub fn getStability(self: Self) f64 {
        return self.stability;
    }

    /// Check if a task aligns with current crystal structure
    pub fn checkAlignment(self: Self, task: anytype) bool {
        _ = task; // Task alignment will be implemented based on final task structure
        return self.stability >= 0.8;
    }

    /// Align a task with the crystal structure
    pub fn alignTask(self: *Self, task: anytype) !void {
        _ = task; // Task alignment will be implemented based on final task structure
        if (self.stability < 0.8) {
            return LatticeError.ResonanceLost;
        }
    }

    /// Integrate a new task into the crystal structure
    pub fn integrateTask(self: *Self, task: anytype) !void {
        _ = task; // Task integration will be implemented based on final task structure
        try self.validateStructure();
        try self.maintainSymmetry();
    }

    /// Check if the lattice is in a stable state
    pub fn isStable(self: Self) bool {
        return self.stability >= 0.8 and self.energy_total <= 10.0;
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

test "lattice basic functionality" {
    const allocator = std.testing.allocator;
    
    const lattice = try Lattice.init(allocator);
    defer lattice.deinit();

    // Add test nodes
    const node1 = try lattice.addNode(.{ 0, 0, 0 });
    const node2 = try lattice.addNode(.{ 1, 0, 0 });

    try std.testing.expect(node1.connections.items.len > 0);
    try std.testing.expect(node2.connections.items.len > 0);
    try std.testing.expect(lattice.isStable());
}

test "crystal system constraints" {
    const allocator = std.testing.allocator;
    
    const lattice = try Lattice.init(allocator);
    defer lattice.deinit();

    // Test cubic system constraints
    try lattice.addNode(.{ 0, 0, 0 });
    try lattice.addNode(.{ 1, 0, 0 });
    try lattice.addNode(.{ 0, 1, 0 });
    try lattice.addNode(.{ 0, 0, 1 });

    try std.testing.expect(lattice.getStability() >= 0.8);
}
