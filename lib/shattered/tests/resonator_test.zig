//! Crystal Resonator Test Suite
//! Created: 2025-01-21 04:42:38 UTC
//! Author: isdood
//! 
//! Test suite for crystal resonance patterns, harmonic
//! stability, and frequency management in the shattered
//! cache system.

const std = @import("std");
const testing = std.testing;
const expectEqual = testing.expectEqual;
const expect = testing.expect;
const math = std.math;
const Time = std.time;

const resonator = @import("../src/resonator.zig");
const Resonator = resonator.Resonator;
const ResonatorError = resonator.ResonatorError;
const RESONANCE_CONSTANTS = resonator.RESONANCE_CONSTANTS;

/// Test utilities
fn isHarmonic(base: f64, freq: f64) bool {
    const ratio = freq / base;
    const nearest = @round(ratio);
    return math.fabs(ratio - nearest) < 0.01;
}

fn measureStability(samples: []const f64) f64 {
    if (samples.len < 2) return 1.0;
    
    var variance: f64 = 0;
    var mean: f64 = 0;

    for (samples) |sample| {
        mean += sample;
    }
    mean /= @intToFloat(f64, samples.len);

    for (samples) |sample| {
        const diff = sample - mean;
        variance += diff * diff;
    }
    variance /= @intToFloat(f64, samples.len - 1);

    return 1.0 / (1.0 + math.sqrt(variance));
}

test "Resonator initialization" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    // Test valid initialization
    const resonator1 = try Resonator.init(allocator, 432.0);
    defer resonator1.deinit();
    try expectEqual(resonator1.frequency, 432.0);
    try expectEqual(resonator1.stability, 1.0);

    // Test frequency bounds
    const result_low = Resonator.init(allocator, 10.0);
    try testing.expectError(ResonatorError.FrequencyOutOfRange, result_low);

    const result_high = Resonator.init(allocator, 25000.0);
    try testing.expectError(ResonatorError.FrequencyOutOfRange, result_high);
}

test "Harmonic generation" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    const resonator1 = try Resonator.init(allocator, 432.0);
    defer resonator1.deinit();

    // Test harmonic frequencies
    for (resonator1.harmonics.items) |harmonic, i| {
        const expected_freq = 432.0 * @intToFloat(f64, i + 1);
        try expect(isHarmonic(432.0, harmonic.frequency));
        if (expected_freq <= RESONANCE_CONSTANTS.MAX_FREQUENCY) {
            try expect(math.fabs(harmonic.frequency - expected_freq) < 0.01);
        }
    }
}

test "Resonance application" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    const resonator1 = try Resonator.init(allocator, 432.0);
    defer resonator1.deinit();

    const test_values = [_]f64{ 0.5, 0.7, 0.9 };
    var results: [test_values.len]f64 = undefined;

    // Apply resonance to test values
    for (test_values) |value, i| {
        results[i] = resonator1.apply(value);
        try expect(results[i] >= 0.0 and results[i] <= 1.0);
    }

    // Verify stability
    const stability = measureStability(&results);
    try expect(stability > 0.8);
}

test "Phase coherence" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    const resonator1 = try Resonator.init(allocator, 432.0);
    defer resonator1.deinit();

    var samples = std.ArrayList(f64).init(allocator);
    defer samples.deinit();

    // Collect samples over time
    var i: usize = 0;
    while (i < 10) : (i += 1) {
        const value = resonator1.apply(0.5);
        try samples.append(value);
        std.time.sleep(10 * std.time.ns_per_ms);
    }

    // Verify phase coherence
    const coherence = measureStability(samples.items);
    try expect(coherence > 0.7);
}

test "Stability under updates" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    const resonator1 = try Resonator.init(allocator, 432.0);
    defer resonator1.deinit();

    const initial_stability = resonator1.getStability();

    // Perform multiple updates
    var i: usize = 0;
    while (i < 10) : (i += 1) {
        try resonator1.update();
        std.time.sleep(10 * std.time.ns_per_ms);
    }

    const final_stability = resonator1.getStability();
    try expect(final_stability >= 0.8 * initial_stability);
}

test "Harmonic interaction" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    const resonator1 = try Resonator.init(allocator, 432.0);
    defer resonator1.deinit();

    // Test harmonic relationships
    const fundamental = resonator1.getHarmonic(0);
    const octave = resonator1.getHarmonic(1);
    const fifth = resonator1.getHarmonic(2);

    try expect(octave > 0.0);
    try expect(fifth > 0.0);
    try expect(isHarmonic(fundamental, octave));
    try expect(isHarmonic(fundamental, fifth));
}

test "Resonance patterns" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    const resonator1 = try Resonator.init(allocator, 432.0);
    defer resonator1.deinit();

    var samples = std.ArrayList(f64).init(allocator);
    defer samples.deinit();

    // Generate resonance pattern
    var i: usize = 0;
    while (i < 20) : (i += 1) {
        const phase = @intToFloat(f64, i) * math.pi / 10.0;
        const value = resonator1.apply(math.sin(phase));
        try samples.append(value);
    }

    // Verify pattern characteristics
    var zero_crossings: usize = 0;
    for (samples.items[1..]) |sample, j| {
        if ((sample >= 0 and samples.items[j] < 0) or
            (sample < 0 and samples.items[j] >= 0)) {
            zero_crossings += 1;
        }
    }

    try expect(zero_crossings > 0);
}

test "Frequency response" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    const frequencies = [_]f64{ 432.0, 440.0, 444.0 };
    var responses = std.ArrayList(f64).init(allocator);
    defer responses.deinit();

    // Test response at different frequencies
    for (frequencies) |freq| {
        const res = try Resonator.init(allocator, freq);
        defer res.deinit();

        const response = res.apply(1.0);
        try responses.append(response);
    }

    // Verify frequency selectivity
    const response_stability = measureStability(responses.items);
    try expect(response_stability < 0.99); // Should show some variation
}

test "Long-term stability" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    const resonator1 = try Resonator.init(allocator, 432.0);
    defer resonator1.deinit();

    var stability_samples = std.ArrayList(f64).init(allocator);
    defer stability_samples.deinit();

    // Monitor stability over time
    var i: usize = 0;
    while (i < 50) : (i += 1) {
        try resonator1.update();
        try stability_samples.append(resonator1.getStability());
        std.time.sleep(10 * std.time.ns_per_ms);
    }

    const long_term_stability = measureStability(stability_samples.items);
    try expect(long_term_stability > 0.85);
}
