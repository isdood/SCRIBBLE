//! wave.zig - Wave pattern system for Prism async runtime
//! Created by: isdood
//! Date: 2025-01-21 10:46:24 UTC

const std = @import("std");
const math = std.math;
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

/// Wave pattern error set
pub const WaveError = error{
    InvalidFrequency,
    ResonanceLost,
    InterferenceOverload,
    PatternDestabilized,
    AmplitudeOverflow,
};

/// Represents a single wave component
pub const Component = struct {
    frequency: f64,
    amplitude: f64,
    phase: f64,
    stability: f64,

    pub fn init(frequency: f64, amplitude: f64, phase: f64) !Component {
        if (frequency <= 0) return WaveError.InvalidFrequency;
        
        return Component{
            .frequency = frequency,
            .amplitude = amplitude,
            .phase = phase,
            .stability = 1.0,
        };
    }

    /// Calculate the wave value at a given time point
    pub fn valueAt(self: Component, time: f64) f64 {
        return self.amplitude * 
               math.sin(self.frequency * time + self.phase) * 
               self.stability;
    }
};

/// Represents interference between two wave components
pub const Interference = struct {
    strength: f64,
    phase_diff: f64,
    resonance: f64,

    pub fn calculate(comp1: Component, comp2: Component) Interference {
        const phase_difference = math.fabs(comp1.phase - comp2.phase);
        const freq_ratio = comp1.frequency / comp2.frequency;
        const amplitude_product = comp1.amplitude * comp2.amplitude;
        
        return Interference{
            .strength = amplitude_product * math.cos(phase_difference),
            .phase_diff = phase_difference,
            .resonance = 1.0 / (1.0 + math.fabs(1.0 - freq_ratio)),
        };
    }
};

/// Main wave pattern structure
pub const Pattern = struct {
    components: ArrayList(Component),
    interference_threshold: f64,
    total_energy: f64,
    resonance_factor: f64,

    const Self = @This();

    /// Initialize a new wave pattern
    pub fn init() !Pattern {
        // Start with default values optimized for task scheduling
        return Pattern{
            .components = ArrayList(Component).init(std.heap.page_allocator),
            .interference_threshold = 0.85,
            .total_energy = 0,
            .resonance_factor = 1.0,
        };
    }

    /// Clean up resources
    pub fn deinit(self: *Self) void {
        self.components.deinit();
    }

    /// Add a new wave component to the pattern
    pub fn addComponent(self: *Self, frequency: f64, amplitude: f64, phase: f64) !void {
        const component = try Component.init(frequency, amplitude, phase);
        try self.components.append(component);
        try self.validatePattern();
    }

    /// Calculate the total wave value at a given time
    pub fn totalValueAt(self: Self, time: f64) f64 {
        var total: f64 = 0;
        for (self.components.items) |component| {
            total += component.valueAt(time);
        }
        return total * self.resonance_factor;
    }

    /// Calculate interference between all components
    pub fn calculateInterference(self: Self) !f64 {
        var total_interference: f64 = 0;
        var interference_count: usize = 0;

        const items = self.components.items;
        var i: usize = 0;
        while (i < items.len) : (i += 1) {
            var j: usize = i + 1;
            while (j < items.len) : (j += 1) {
                const interference = Interference.calculate(items[i], items[j]);
                total_interference += interference.strength * interference.resonance;
                interference_count += 1;
            }
        }

        if (interference_count == 0) return 0;
        const avg_interference = total_interference / @intToFloat(f64, interference_count);
        
        if (avg_interference > self.interference_threshold) {
            return WaveError.InterferenceOverload;
        }

        return avg_interference;
    }

    /// Update the pattern's resonance based on current state
    pub fn updateResonance(self: *Self) !void {
        const interference = try self.calculateInterference();
        const stability = self.calculateStability();
        
        self.resonance_factor = (1.0 - interference) * stability;
        
        if (self.resonance_factor < 0.5) {
            return WaveError.ResonanceLost;
        }
    }

    /// Calculate the current stability of the pattern
    fn calculateStability(self: Self) f64 {
        var total_stability: f64 = 0;
        for (self.components.items) |component| {
            total_stability += component.stability;
        }
        
        if (self.components.items.len == 0) return 1.0;
        return total_stability / @intToFloat(f64, self.components.items.len);
    }

    /// Validate the overall pattern
    fn validatePattern(self: Self) !void {
        var total_amplitude: f64 = 0;
        for (self.components.items) |component| {
            total_amplitude += component.amplitude;
        }

        if (total_amplitude > 10.0) {
            return WaveError.AmplitudeOverflow;
        }

        const stability = self.calculateStability();
        if (stability < 0.5) {
            return WaveError.PatternDestabilized;
        }
    }

    /// Optimize the wave pattern for better resonance
    pub fn optimize(self: *Self) !void {
        // Sort components by frequency for optimal interference
        std.sort.sort(Component, self.components.items, {}, struct {
            fn lessThan(_: void, a: Component, b: Component) bool {
                return a.frequency < b.frequency;
            }
        }.lessThan);

        // Adjust phases for minimal destructive interference
        if (self.components.items.len > 1) {
            var base_phase = self.components.items[0].phase;
            for (self.components.items[1..]) |*component| {
                component.phase = base_phase + math.pi / 4.0;
                base_phase = component.phase;
            }
        }

        try self.updateResonance();
    }
};

test "wave pattern basic functionality" {
    var pattern = try Pattern.init();
    defer pattern.deinit();

    // Add some test components
    try pattern.addComponent(1.0, 0.5, 0.0);
    try pattern.addComponent(2.0, 0.3, math.pi / 4.0);

    // Test interference calculation
    const interference = try pattern.calculateInterference();
    try std.testing.expect(interference >= 0.0 and interference <= 1.0);

    // Test pattern optimization
    try pattern.optimize();
    try std.testing.expect(pattern.resonance_factor > 0.5);
}

test "wave component creation" {
    const component = try Component.init(2.0, 1.0, 0.0);
    try std.testing.expect(component.frequency == 2.0);
    try std.testing.expect(component.amplitude == 1.0);
    try std.testing.expect(component.phase == 0.0);
    try std.testing.expect(component.stability == 1.0);
}

test "wave interference calculation" {
    const comp1 = try Component.init(1.0, 1.0, 0.0);
    const comp2 = try Component.init(1.0, 1.0, math.pi);
    
    const interference = Interference.calculate(comp1, comp2);
    try std.testing.expect(interference.strength < 0.0); // Destructive interference
    try std.testing.expect(interference.resonance == 1.0); // Same frequency
}
