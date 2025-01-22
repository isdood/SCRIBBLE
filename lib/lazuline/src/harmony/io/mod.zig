const std = @import("std");
const root = @import("../../lib.zig");

pub const IOMode = enum {
    Read,
    Write,
};

pub const HarmonicIO = struct {
    const Self = @This();
    wave_patterns: std.ArrayList(root.wave.WaveFunction),
    io_lattice: *const root.lattice.CrystalLattice,
    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator, io_lattice: *const root.lattice.CrystalLattice) !HarmonicIO {
        return HarmonicIO{
            .wave_patterns = std.ArrayList(root.wave.WaveFunction).init(allocator),
            .io_lattice = io_lattice,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *Self) void {
        for (self.wave_patterns.items) |*wave| {
            wave.deinit();
        }
        self.wave_patterns.deinit();
    }

    pub fn read(self: *Self, buffer: []u8) !usize {
        _ = buffer;
        _ = self;
        return 0;
    }

    pub fn write(self: *Self, data: []const u8) !usize {
        _ = data;
        _ = self;
        return 0;
    }
};
