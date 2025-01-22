const std = @import("std");

pub const HarmonyState = struct {
    resonance: f64,

    pub fn init() HarmonyState {
        return .{
            .resonance = 1.0,
        };
    }

    pub fn process(self: *HarmonyState) void {
        self.resonance *= 0.99;
    }
};
