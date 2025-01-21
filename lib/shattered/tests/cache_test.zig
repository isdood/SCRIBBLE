//! Shattered Cache Test Suite
//! Created: 2025-01-21 04:37:21 UTC
//! Author: isdood
//! 
//! Comprehensive test suite for the shattered cache system,
//! validating harmony maintenance, pattern recognition,
//! and cache operations.

const std = @import("std");
const testing = std.testing;
const expectEqual = testing.expectEqual;
const expect = testing.expect;
const Time = std.time;

const cache = @import("../src/cache.zig");
const patterns = @import("../src/patterns.zig");
const harmony = @import("../src/harmony.zig");

const ShatteredCache = cache.ShatteredCache;
const Config = cache.Config;
const AccessPattern = patterns.AccessPattern;
const CacheError = cache.CacheError;

/// Test utilities
fn sleep(ms: u64) void {
    std.time.sleep(ms * std.time.ns_per_ms);
}

/// Test suite
test "Cache initialization" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    const config = Config{
        .harmony_threshold = 0.87,
        .prediction_depth = 3,
        .resonance_freq = 432.0,
        .max_shards = 1024,
    };

    const cache_system = try ShatteredCache.init(allocator, config);
    defer cache_system.deinit();

    try expectEqual(cache_system.config.harmony_threshold, 0.87);
    try expect(cache_system.getHarmony() > 0.9);
}

test "Shard creation and management" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    const cache_system = try ShatteredCache.init(allocator, Config{});
    defer cache_system.deinit();

    // Create multiple shards
    const shard_sizes = [_]usize{ 1024, 2048, 4096 };
    var shards = std.ArrayList(*cache.Shard).init(allocator);
    defer shards.deinit();

    for (shard_sizes) |size| {
        const shard = try cache_system.createShard(size);
        try shards.append(shard);
        try expect(shard.getSize() == size);
    }

    // Verify shard count
    try expectEqual(cache_system.shards.items.len, shard_sizes.len);
}

test "Access pattern recognition" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    const cache_system = try ShatteredCache.init(allocator, Config{});
    defer cache_system.deinit();

    const shard = try cache_system.createShard(1024);
    const patterns_to_test = [_]AccessPattern{
        .Sequential,
        .Strided,
        .Random,
        .Clustered,
        .Hybrid,
    };

    // Test each access pattern
    for (patterns_to_test) |pattern| {
        try cache_system.preShatter(pattern);
        try expect(shard.pattern == pattern);
        sleep(100); // Allow for harmony stabilization
    }
}

test "Harmony maintenance during operations" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    const cache_system = try ShatteredCache.init(allocator, Config{});
    defer cache_system.deinit();

    const shard = try cache_system.createShard(1024);
    const initial_harmony = cache_system.getHarmony();

    // Perform multiple operations
    var i: usize = 0;
    while (i < 10) : (i += 1) {
        try cache_system.preShatter(.Sequential);
        sleep(50);
    }

    const final_harmony = cache_system.getHarmony();
    try expect(final_harmony >= 0.8 * initial_harmony);
}

test "Cache eviction under pressure" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    const config = Config{
        .max_shards = 3,
        .harmony_threshold = 0.87,
    };

    const cache_system = try ShatteredCache.init(allocator, config);
    defer cache_system.deinit();

    // Create shards until eviction
    var i: usize = 0;
    while (i < 5) : (i += 1) {
        _ = try cache_system.createShard(1024);
        sleep(10);
    }

    try expect(cache_system.shards.items.len <= config.max_shards);
}

test "Pattern prediction accuracy" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    const cache_system = try ShatteredCache.init(allocator, Config{});
    defer cache_system.deinit();

    const shard = try cache_system.createShard(1024);
    
    // Train the system with sequential access
    var i: usize = 0;
    while (i < 5) : (i += 1) {
        try cache_system.preShatter(.Sequential);
        sleep(20);
    }

    const stats = cache_system.getStats();
    const prediction_accuracy = @intToFloat(f64, stats.correct_predictions.load(.Monotonic)) /
                              @intToFloat(f64, stats.predictions.load(.Monotonic));
    
    try expect(prediction_accuracy > 0.7);
}

test "Resonance stability under load" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    const cache_system = try ShatteredCache.init(allocator, Config{});
    defer cache_system.deinit();

    // Create multiple shards with different patterns
    const patterns_to_test = [_]AccessPattern{
        .Sequential,
        .Strided,
        .Random,
    };

    for (patterns_to_test) |pattern| {
        const shard = try cache_system.createShard(1024);
        try cache_system.preShatter(pattern);
        try expect(shard.resonance > 0.0);
        sleep(50);
    }

    // Verify overall stability
    try expect(cache_system.getHarmony() > 0.8);
}

test "Error handling" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    const config = Config{
        .max_shards = 1,
        .harmony_threshold = 0.99, // Very high threshold
    };

    const cache_system = try ShatteredCache.init(allocator, config);
    defer cache_system.deinit();

    // Test overflow handling
    _ = try cache_system.createShard(1024);
    const result = cache_system.createShard(1024);
    try testing.expectError(CacheError.CacheFull, result);
}

test "Performance metrics" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    const cache_system = try ShatteredCache.init(allocator, Config{});
    defer cache_system.deinit();

    const shard = try cache_system.createShard(1024);
    
    // Generate some activity
    var i: usize = 0;
    while (i < 10) : (i += 1) {
        try cache_system.preShatter(
            if (i % 2 == 0) AccessPattern.Sequential else AccessPattern.Random
        );
        sleep(10);
    }

    const stats = cache_system.getStats();
    try expect(stats.predictions.load(.Monotonic) > 0);
    try expect(stats.hits.load(.Monotonic) > 0);
}
