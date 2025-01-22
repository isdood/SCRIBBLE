const std = @import("std");

pub const CrystalLattice = struct {
    const Self = @This();
    nodes: std.ArrayList(Node),
    connections: std.ArrayList(Connection),
    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator) !CrystalLattice {
        return CrystalLattice{
            .nodes = std.ArrayList(Node).init(allocator),
            .connections = std.ArrayList(Connection).init(allocator),
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *Self) void {
        self.nodes.deinit();
        self.connections.deinit();
    }
};

const Node = struct {
    id: u64,
    position: [3]f64,
    energy: f64,
};

const Connection = struct {
    from: u64,
    to: u64,
    strength: f64,
};
