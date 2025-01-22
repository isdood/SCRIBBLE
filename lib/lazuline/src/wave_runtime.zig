const std = @import("std");
const lib = @import("lib.zig");

pub const WaveRuntime = struct {
    const Self = @This();

    lattice: lib.lattice.CrystalLattice,
    scheduler: lib.resonance.ResonanceScheduler,
    io_system: lib.harmony.HarmonicIO,
    wave_computer: lib.wave.WaveComputer,
    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator) !Self {
        const lattice = try lib.lattice.CrystalLattice.init(allocator);
        const wave_computer = try lib.wave.WaveComputer.init(allocator);
        const scheduler = try lib.resonance.ResonanceScheduler.init(allocator, &lattice, &wave_computer);
        const io_system = try lib.harmony.HarmonicIO.init(allocator, &lattice);

        return Self{
            .lattice = lattice,
            .scheduler = scheduler,
            .io_system = io_system,
            .wave_computer = wave_computer,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *Self) void {
        self.io_system.deinit();
        self.scheduler.deinit();
        self.wave_computer.deinit();
        self.lattice.deinit();
    }
};
