const std = @import("std");
const constants = @import("constants.zig");

pub const HarmonicState = struct {
    resonance: f64,
    phase: f64,
    field: f64,

    pub fn new() HarmonicState {
        return .{
            .resonance = 1.0,
            .phase = 0.0,
            .field = 0.0,
        };
    }

    pub inline fn apply_field(self: *HarmonicState, value: f64) void {
        self.field = value;
        self.resonance *= constants.AETHER_RESONANCE_FACTOR;
    }

    pub inline fn get_coherence(self: HarmonicState) f64 {
        return self.resonance * @cos(self.phase);
    }
};
