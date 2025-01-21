//! Shattered Cache Integration Test Suite
//! Created: 2025-01-21 04:39:01 UTC
//! Author: isdood
//! 
//! Main test runner and integration tests for the shattered cache system.
//! Validates component interaction and system-wide behaviors.

const std = @import("std");
const testing = std.testing;
const expectEqual = testing.expectEqual;
const expect = testing.expect;
const Time = std.time;

// Import all test modules
const cache_test = @import("cache_test.zig");
const shard_test = @import("shard_test.zig");
const resonator_test = @import("resonator_test.zig");
const harmony_test = @import("harmony_test.zig");
const patterns_test = @import("patterns_test.zig");

// Import implementation modules for integration tests
const cache = @import("../src/cache.zig");
const shard = @import("../src/shard.zig");
const resonator = @import("../src/resonator.zig");
const harmony = @import("../src/harmony.zig");
const patterns = @import("../src/patterns.zig");

const ShatteredCache = cache.ShatteredCache;
const Config = cache.Config;
const AccessPattern = patterns.AccessPattern;

/// Test utilities
const TestContext = struct {
    cache: *ShatteredCache,
    allocator: *std.mem.Allocator,
    start_time: i64,

    pub fn init() !TestContext {
        var arena = std.heap.ArenaAllocator.init(testing.allocator);
        const allocator = &arena.allocator;

        const cache_system = try ShatteredCache.init(allocator, Config{});

        return TestContext{
            .cache = cache_system,
            .allocator = allocator,
            .start_time = Time.milliTimestamp(),
        };
    }

    pub fn deinit(self: *TestContext) void {
        self.cache.deinit();
        // Arena allocator cleanup is handled by the test framework
    }

    pub fn elapsed(self: TestContext) i64 {
        return Time.milliTimestamp() - self.start_time;
    }
};

// Run all component tests
test {
    // Component test suites
    testing.refAllDecls(cache_test);
    testing.refAllDecls(shard_test);
    testing.refAllDecls(resonator_test);
    testing.refAllDecls(harmony_test);
    testing.refAllDecls(patterns_test);
}

// Integration tests
test "Full system integration" {
    var ctx = try TestContext.init();
    defer ctx.deinit();

    // Create a diverse set of shards
    const shard_configs = [_]struct { size: usize, pattern: AccessPattern }{
        .{ .size = 1024, .pattern = .Sequential },
        .{ .size = 2048, .pattern = .Strided },
        .{ .size = 4096, .pattern = .Random },
    };

    for (shard_configs) |config| {
        const shard = try ctx.cache.createShard(config.size);
        try ctx.cache.preShatter(config.pattern);
        try expect(shard.getSize() == config.size);
    }

    // Verify system stability
    try expect(ctx.cache.getHarmony() > 0.8);
}

test "Pattern transition cascade" {
    var ctx = try TestContext.init();
    defer ctx.deinit();

    const shard = try ctx.cache.createShard(1024);
    const patterns_sequence = [_]AccessPattern{
        .Sequential,
        .Strided,
        .Clustered,
        .Random,
        .Hybrid,
    };

    for (patterns_sequence) |pattern| {
        try ctx.cache.preShatter(pattern);
        try expectEqual(shard.pattern, pattern);
        std.time.sleep(50 * std.time.ns_per_ms);
    }

    // Check harmony maintenance through transitions
    try expect(ctx.cache.getHarmony() > 0.75);
}

test "Concurrent pattern optimization" {
    var ctx = try TestContext.init();
    defer ctx.deinit();

    const thread_count = 4;
    var threads: [thread_count]std.Thread = undefined;

    const thread_fn = struct {
        fn run(cache_ptr: *ShatteredCache) !void {
            var i: usize = 0;
            while (i < 10) : (i += 1) {
                try cache_ptr.preShatter(.Sequential);
                std.time.sleep(10 * std.time.ns_per_ms);
            }
        }
    }.run;

    // Create threads
    for (&threads) |*thread| {
        thread.* = try std.Thread.spawn(.{}, thread_fn, .{ctx.cache});
    }

    // Wait for threads
    for (threads) |thread| {
        thread.join();
    }

    // Verify system stability after concurrent operations
    try expect(ctx.cache.getHarmony() > 0.7);
}

test "Resource management under pressure" {
    var ctx = try TestContext.init();
    defer ctx.deinit();

    // Fill cache to capacity
    const max_shards = ctx.cache.config.max_shards;
    var i: usize = 0;
    while (i < max_shards + 5) : (i += 1) {
        _ = try ctx.cache.createShard(1024);
        std.time.sleep(10 * std.time.ns_per_ms);
    }

    // Verify automatic resource management
    try expect(ctx.cache.shards.items.len <= max_shards);
    try expect(ctx.cache.getHarmony() > 0.8);
}

test "System recovery after disruption" {
    var ctx = try TestContext.init();
    defer ctx.deinit();

    // Create and stabilize initial state
    const shard = try ctx.cache.createShard(1024);
    try ctx.cache.preShatter(.Sequential);

    // Simulate disruption
    var i: usize = 0;
    while (i < 20) : (i += 1) {
        try ctx.cache.preShatter(
            if (i % 2 == 0) AccessPattern.Random else AccessPattern.Sequential
        );
        std.time.sleep(5 * std.time.ns_per_ms);
    }

    // Allow system to stabilize
    std.time.sleep(100 * std.time.ns_per_ms);

    // Verify recovery
    try expect(ctx.cache.getHarmony() > 0.85);
    try expect(shard.isStable());
}

test "Long-term harmony stability" {
    var ctx = try TestContext.init();
    defer ctx.deinit();

    const iterations = 50;
    const patterns = [_]AccessPattern{
        .Sequential, .Strided, .Random, .Clustered, .Hybrid
    };

    const initial_harmony = ctx.cache.getHarmony();
    
    // Run extended operation sequence
    var i: usize = 0;
    while (i < iterations) : (i += 1) {
        const pattern = patterns[i % patterns.len];
        try ctx.cache.preShatter(pattern);
        std.time.sleep(10 * std.time.ns_per_ms);
    }

    const final_harmony = ctx.cache.getHarmony();
    
    // Verify long-term stability
    try expect(final_harmony >= 0.9 * initial_harmony);
    try expect(ctx.elapsed() > 500); // Ensure sufficient test duration
}
