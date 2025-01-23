const std = @import("std");

pub const QuantumState = struct {
    amplitude: f64,
    phase: f64,
    coherence: f64,

    pub fn init(amplitude: f64, phase: f64) QuantumState {
        return .{
            .amplitude = amplitude,
            .phase = phase,
            .coherence = 0.87,
        };
    }
};
