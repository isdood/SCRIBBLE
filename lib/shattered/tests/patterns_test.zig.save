//! Access Pattern Test Suite
//! Created: 2025-01-21 04:40:27 UTC
//! Author: isdood
//! 
//! Test suite for access pattern recognition, transitions,
//! and optimization in the shattered cache system.

const std = @import("std");
const testing = std.testing;
const expectEqual = testing.expectEqual;
const expect = testing.expect;
const Time = std.time;

const patterns = @import("../src/patterns.zig");
const cache = @import("../src/cache.zig");
const shard = @import("../src/shard.zig");

const AccessPattern = patterns.AccessPattern;
const ShatteredCache = cache.ShatteredCache;
const Shard = shard.Shard;

/// Test utilities
fn generateAccessSequence(allocator: *std.mem.Allocator, pattern: AccessPattern, size: usize) ![]usize {
    var sequence = try allocator.alloc(usize, size);
    errdefer allocator.free(sequence);

    switch (pattern) {
        .Sequential => {
            for (sequence) |*value, i| {
                value.* = i;
            }
        },
        .Strided => {
            const stride = 16;
            var i: usize = 0;
            while (i < size) : (i += 1) {
                sequence[i] = (i * stride) % size;
            }
        },
        .Random => {
            var prng = std.rand.DefaultPrng.init(@intCast(u64, Time.milliTimestamp()));
            var i: usize = 0;
            while (i < size) : (i += 1) {
                sequence[i] = prng.random().uintLessThan(usize, size);
            }
        },
        .Clustered => {
            const cluster_size = 8;
            var i: usize = 0;
            while (i < size) : (i += cluster_size) {
                var j: usize = 0;
                while (j < cluster_size and i + j < size) : (j += 1) {
                    sequence[i + j] = i;
                }
            }
        },
        .Hybrid => {
            // Mix of sequential and strided
            for (sequence) |*value, i| {
                value.* = if (i % 2 == 0) i else (i * 16) % size;
            }
        },
    }

    return sequence;
}

fn validatePattern(sequence: []const usize, pattern: AccessPattern) bool {
    switch (pattern) {
        .Sequential => {
            for (sequence) |value, i| {
                if (value != i) return false;
            }
        },
        .Strided => {
            const stride = sequence[1] - sequence[0];
            for (sequence[1..]) |value, i| {
                if ((value + stride) % sequence.len != sequence[(i + 2) % sequence.len]) {
                    return false;
                }
            }
        },
        .Random => {
            // Verify non-sequential nature
            var sequential_count: usize = 0;
            for (sequence[1..]) |value, i| {
                if (value == sequence[i] + 1) {
                    sequential_count += 1;
                }
            }
            if (sequential_count > sequence.len / 4) return false;
        },
        .Clustered => {
            var cluster_starts: usize = 0;
            var i: usize = 1;
            while (i < sequence.len) : (i += 1) {
                if (sequence[i] != sequence[i - 1]) {
                    cluster_starts += 1;
                }
            }
            if (cluster_starts > sequence.len / 8) return false;
        },
        .Hybrid => {
            // Check for mixed pattern characteristics
            var sequential: usize = 0;
            var strided: usize = 0;
            
            for (sequence[1..]) |value, i| {
                if (value == sequence[i] + 1) sequential += 1;
                if (value == (sequence[i] * 16) % sequence.len) strided += 1;
            }
            
            if (sequential == 0 or strided == 0) return false;
        },
    }
    return true;
}

test "Access pattern generation" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    const size = 1024;
    const patterns_to_test = [_]AccessPattern{
        .Sequential,
        .Strided,
        .Random,
        .Clustered,
        .Hybrid,
    };

    for (patterns_to_test) |pattern| {
        const sequence = try generateAccessSequence(allocator, pattern, size);
        defer allocator.free(sequence);
        
        try expect(validatePattern(sequence, pattern));
    }
}

test "Pattern transition detection" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    const cache_system = try ShatteredCache.init(allocator, .{});
    defer cache_system.deinit();

    const shard = try cache_system.createShard(1024);
    const test_patterns = [_]AccessPattern{
        .Sequential,
        .Strided,
        .Random,
        .Clustered,
        .Hybrid,
    };

    for (test_patterns) |pattern| {
        try cache_system.preShatter(pattern);
        try expectEqual(shard.pattern, pattern);
        std.time.sleep(50 * std.time.ns_per_ms);
    }
}

test "Pattern stability under noise" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    const cache_system = try ShatteredCache.init(allocator, .{});
    defer cache_system.deinit();

    const shard = try cache_system.createShard(1024);
    
    // Establish sequential pattern
    try cache_system.preShatter(.Sequential);
    const initial_harmony = shard.getHarmony();

    // Introduce noise
    var i: usize = 0;
    while (i < 10) : (i += 1) {
        if (i % 3 == 0) {
            try cache_system.preShatter(.Random);
        } else {
            try cache_system.preShatter(.Sequential);
        }
        std.time.sleep(10 * std.time.ns_per_ms);
    }

    // Verify pattern stability
    try expect(shard.pattern == .Sequential);
    try expect(shard.getHarmony() >= 0.7 * initial_harmony);
}

test "Pattern optimization effectiveness" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    const cache_system = try ShatteredCache.init(allocator, .{});
    defer cache_system.deinit();

    const shard = try cache_system.createShard(1024);
    const sequence = try generateAccessSequence(allocator, .Sequential, 1024);
    defer allocator.free(sequence);

    // Train the system
    var i: usize = 0;
    while (i < sequence.len) : (i += 64) {
        try cache_system.preShatter(.Sequential);
        std.time.sleep(5 * std.time.ns_per_ms);
    }

    // Verify optimization
    try expect(shard.pattern == .Sequential);
    try expect(shard.getPatternStrength() > 0.8);
}

test "Hybrid pattern recognition" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    const cache_system = try ShatteredCache.init(allocator, .{});
    defer cache_system.deinit();

    const shard = try cache_system.createShard(1024);
    
    // Generate mixed access patterns
    var i: usize = 0;
    while (i < 20) : (i += 1) {
        const pattern = if (i % 2 == 0) AccessPattern.Sequential else AccessPattern.Strided;
        try cache_system.preShatter(pattern);
        std.time.sleep(10 * std.time.ns_per_ms);
    }

    // System should detect hybrid pattern
    try expectEqual(shard.pattern, .Hybrid);
}

test "Pattern prediction accuracy" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    const cache_system = try ShatteredCache.init(allocator, .{});
    defer cache_system.deinit();

    // Train with clustered access
    const sequence = try generateAccessSequence(allocator, .Clustered, 1024);
    defer allocator.free(sequence);

    var i: usize = 0;
    while (i < 10) : (i += 1) {
        try cache_system.preShatter(.Clustered);
        std.time.sleep(10 * std.time.ns_per_ms);
    }

    const stats = cache_system.getStats();
    const prediction_rate = @intToFloat(f64, stats.correct_predictions.load(.Monotonic)) /
                          @intToFloat(f64, stats.predictions.load(.Monotonic));
    
    try expect(prediction_rate > 0.8);
}
