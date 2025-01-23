const std = @import("std");
const harmony = @import("harmony.zig");
const crystals = @import("crystals.zig");

pub const HarmonyCore = struct {
    resonance_field: harmony.Field,
    crystal_matrix: crystals.Matrix,
    whimsy_factor: f64,

    pub fn init() !HarmonyCore {
        return HarmonyCore{
            .resonance_field = try harmony.Field.init(),
            .crystal_matrix = try crystals.Matrix.init(),
            .whimsy_factor = 0.618033988749895, // Golden ratio for maximum whimsy
        };
    }

    pub fn attuneCrystals(self: *HarmonyCore) !void {
        try self.crystal_matrix.attune(self.resonance_field, self.whimsy_factor);
    }
};
