const std = @import("std");
const harmony = @import("harmony.zig");
const crystal = @import("crystal.zig");

pub const Opal = struct {
    resonance_field: harmony.ResonanceField,
    lattice_matrix: crystal.LatticeMatrix,

    pub fn init() !Opal {
        return Opal{
            .resonance_field = try harmony.ResonanceField.init(),
            .lattice_matrix = try crystal.LatticeMatrix.init(),
        };
    }
};
