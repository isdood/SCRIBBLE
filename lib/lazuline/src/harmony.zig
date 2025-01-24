const std = @import("std");

pub const Harmony = struct {
    resonance: f64,

    pub fn init() Harmony {
        return Harmony{ .resonance = 1.0 };
    }
};
