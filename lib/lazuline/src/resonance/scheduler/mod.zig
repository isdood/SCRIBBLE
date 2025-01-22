const std = @import("std");
const root = @import("../../lib.zig");

pub const Task = struct {
    id: u64,
    priority: u8,
    wave_pattern: root.wave.WaveFunction,
};

pub const ResonanceScheduler = struct {
    const Self = @This();

    lattice: *const root.lattice.CrystalLattice,
    wave_computer: *const root.wave.WaveComputer,
    task_waves: std.ArrayList(root.wave.WaveFunction),
    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator,
                lattice: *const root.lattice.CrystalLattice,
                wave_computer: *const root.wave.WaveComputer) !ResonanceScheduler {
        return ResonanceScheduler{
            .lattice = lattice,
            .wave_computer = wave_computer,
            .task_waves = std.ArrayList(root.wave.WaveFunction).init(allocator),
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *Self) void {
        for (self.task_waves.items) |*wave| {
            wave.deinit();
        }
        self.task_waves.deinit();
    }

    pub fn schedule(self: *Self, task: *const Task) !void {
        _ = task;
        _ = self;
    }
};
