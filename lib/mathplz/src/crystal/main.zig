const std = @import("std");

pub const Lattice = struct {
    dimension: u8,
    coherence: f64,
    nodes: []Node,

    pub const Node = struct {
        position: [3]f64,
        energy: f64,
    };

    pub fn init(allocator: std.mem.Allocator, dimension: u8) !Lattice {
        return Lattice{
            .dimension = dimension,
            .coherence = 0.93,
            .nodes = try allocator.alloc(Node, dimension * dimension * dimension),
        };
    }
};
