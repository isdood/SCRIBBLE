const std = @import("std");
const constants = @import("constants.zig");

pub const HarmonicState = struct {
    coherence: f64 = 1.0,
    field_strength: f64 = 0.0,
    const VectorType = @Vector(8, f64);

    pub fn new() HarmonicState {
        return .{};
    }

    pub inline fn apply_field(self: *HarmonicState, field: f64) void {
        self.field_strength = field;
        self.coherence = @exp(-@abs(field) / constants.QUANTUM_DECAY_RATE);
    }

    pub inline fn apply_field_vector(self: *HarmonicState, fields: VectorType) void {
        const abs_fields = @abs(fields);
        const decay_rate = @as(VectorType, @splat(constants.QUANTUM_DECAY_RATE));
        const coherence_vec = @exp(-abs_fields / decay_rate);

        // Use average coherence for state update
        var sum: f64 = 0;
        for (@as([8]f64, coherence_vec)) |val| {
            sum += val;
        }
        self.coherence = sum / 8.0;

        // Store max field strength
        var max_field: f64 = 0;
        for (@as([8]f64, fields)) |val| {
            max_field = @max(max_field, @abs(val));
        }
        self.field_strength = max_field;
    }

    pub inline fn get_coherence(self: *const HarmonicState) f64 {
        return self.coherence;
    }

    pub inline fn get_field_strength(self: *const HarmonicState) f64 {
        return self.field_strength;
    }
};
