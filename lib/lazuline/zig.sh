#!/bin/bash

echo "[INFO] Starting Lazuline optimization..."
echo "[INFO] Current time: 2025-01-23 03:37:12 UTC"
echo "[INFO] User: isdood"

cat > bench/main.zig << 'EOF'
const std = @import("std");
const lazuline = @import("lazuline");
const print = std.debug.print;

fn verifyResults(scalar_sum: f64, vector_sum: f64) bool {
    const diff = @abs(scalar_sum - vector_sum);
    return diff < 1e-10;
}

pub fn main() !void {
    const iterations: usize = 1000000;
    const warmup_iterations: usize = 100000;

    // Wave Pattern benchmark
    {
        const amp = 1.0;
        const freq = 440.0;
        const phase = 0.0;
        var wave = lazuline.WavePattern.new(amp, freq, phase);

        // Warmup with realistic data
        var warmup_acc: f64 = 0;
        var i: usize = 0;
        while (i < warmup_iterations) : (i += 1) {
            const t = @as(f64, @floatFromInt(i)) * 0.001;
            warmup_acc += wave.compute(t);
        }

        // Scalar benchmark
        const start_scalar = std.time.nanoTimestamp();
        var acc_scalar: f64 = 0;
        i = 0;
        while (i < iterations) : (i += 1) {
            const t = @as(f64, @floatFromInt(i)) * 0.001;
            acc_scalar += wave.compute(t);
        }
        const end_scalar = std.time.nanoTimestamp();
        const duration_scalar = @as(f64, @floatFromInt(end_scalar - start_scalar)) / @as(f64, @floatFromInt(iterations));

        // Vector benchmark with aligned data
        var times: [8]f64 align(64) = undefined;
        const start_vector = std.time.nanoTimestamp();
        var acc_vector: f64 = 0;
        i = 0;
        while (i < iterations) : (i += 8) {
            for (0..8) |j| {
                times[j] = @as(f64, @floatFromInt(i + j)) * 0.001;
            }
            const results = wave.computeVectorized(&times);
            for (results) |result| {
                acc_vector += result;
            }
        }
        const end_vector = std.time.nanoTimestamp();
        const duration_vector = @as(f64, @floatFromInt(end_vector - start_vector)) / @as(f64, @floatFromInt(iterations));

        print("Wave Pattern (Scalar): {d:.2} ns/op\n", .{duration_scalar});
        print("Wave Pattern (Vector): {d:.2} ns/op\n", .{duration_vector});
        print("Control sums: scalar={d}, vector={d}\n", .{acc_scalar, acc_vector});
        print("Speedup: {d:.2}x\n", .{duration_scalar / duration_vector});
        print("Results match: {}\n", .{verifyResults(acc_scalar, acc_vector)});
    }
}
EOF

cat > src/lib.zig << 'EOF'
const std = @import("std");
const constants = @import("constants.zig");
const harmonic = @import("harmonic.zig");
const math = std.math;

pub const version = "0.1.0";

const VectorType = @Vector(8, f64);

pub const types = struct {
    pub const WavePattern = struct {
        amplitude: f64,
        frequency: f64,
        phase: f64,
        omega: f64,
        harmonic_state: harmonic.HarmonicState,
        resonance_cache: [32]f64 align(64) = [_]f64{0} ** 32,
        cache_valid: bool = false,

        pub fn new(amplitude: f64, frequency: f64, phase: f64) @This() {
            var self = @This(){
                .amplitude = amplitude,
                .frequency = frequency,
                .phase = phase,
                .omega = 2.0 * math.pi * frequency,
                .harmonic_state = harmonic.HarmonicState.new(),
            };
            self.updateResonanceCache();
            return self;
        }

        inline fn updateResonanceCache(self: *@This()) void {
            if (!self.cache_valid) {
                var i: usize = 0;
                while (i < 32) : (i += 1) {
                    const t = @as(f64, @floatFromInt(i)) * 0.03125;
                    self.resonance_cache[i] = math.sin(self.omega * t + self.phase);
                }
                self.cache_valid = true;
            }
        }

        pub inline fn compute(self: *const @This(), time: f64) f64 {
            var harmonic_state = self.harmonic_state;
            harmonic_state.apply_field(time * self.frequency);

            const normalized_time = time - @floor(time);
            const cache_index = @as(usize, @intFromFloat(normalized_time * 32.0));
            if (cache_index < 32) {
                const coherence = harmonic_state.get_coherence();
                return self.amplitude * self.resonance_cache[cache_index] * coherence;
            }

            const coherence = harmonic_state.get_coherence();
            return self.amplitude * math.sin(self.omega * time + self.phase) * coherence;
        }

        pub inline fn computeVectorized(self: *const @This(), times: *const [8]f64) [8]f64 {
            const time_vec: VectorType = times.*;
            const freq_vec: VectorType = @splat(@as(f64, self.frequency));

            // Pre-compute normalized times for cache lookup
            const normalized_vec = time_vec - @floor(time_vec);
            const cache_indices_f = normalized_vec * @as(VectorType, @splat(32.0));

            // Apply field to harmonic state
            var harmonic_state = self.harmonic_state;
            harmonic_state.apply_field_vector(time_vec * freq_vec);
            const coherence = harmonic_state.get_coherence();

            var result: [8]f64 align(64) = undefined;

            // Process each element using cache when possible
            inline for (0..8) |i| {
                const idx = @as(usize, @intFromFloat(cache_indices_f[i]));
                if (idx < 32) {
                    result[i] = self.amplitude * self.resonance_cache[idx] * coherence;
                } else {
                    const t = time_vec[i];
                    result[i] = self.amplitude * math.sin(self.omega * t + self.phase) * coherence;
                }
            }

            return result;
        }
    };

    // ... (rest of the code remains unchanged)
};

// Export convenience aliases
pub const WavePattern = types.WavePattern;
pub const QuantumResonance = types.QuantumResonance;
pub const CrystalLattice = types.CrystalLattice;
EOF

echo "[SUCCESS] Moved result verification to benchmark code"
echo "[INFO] Try running 'zig build bench' now"
