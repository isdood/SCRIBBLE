const std = @import("std");

pub const DNA = struct {
    sequence: []u2,

    pub fn init(allocator: std.mem.Allocator, size: usize) !DNA {
        return DNA{
            .sequence = try allocator.alloc(u2, size),
        };
    }

    pub fn encode(base: u8) u2 {
        return switch (base) {
            'A' => 0,
            'T' => 1,
            'C' => 2,
            'G' => 3,
            else => unreachable,
        };
    }
};

pub const ProteinFolder = struct {
    energy: f64,
    angles: []f64,

    pub fn init(allocator: std.mem.Allocator, size: usize) !ProteinFolder {
        return ProteinFolder{
            .energy = 0.0,
            .angles = try allocator.alloc(f64, size),
        };
    }
};
