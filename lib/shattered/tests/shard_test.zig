//! Shard Management Test Suite
//! Created: 2025-01-21 04:44:16 UTC
//! Author: isdood
//! 
//! Test suite for shard operations, data handling, and
//! harmony maintenance in the shattered cache system.

const std = @import("std");
const testing = std.testing;
const expectEqual = testing.expectEqual;
const expect = testing.expect;
const Time = std.time;

const shard = @import("../src/shard.zig");
const harmony = @import("../src/harmony.zig");
const patterns = @import("../src/patterns.zig");

const Shard = shard.Shard;
const ShardError = shard.ShardError;
const AccessPattern = patterns.AccessPattern;

/// Test utilities
const TestPattern = struct {
    pattern: AccessPattern,
    data: []const u8,
    offset: usize,
    expected_harmony: f64,
};

fn validateData(data: []const u8, pattern: []const u8) bool {
    if (data.len != pattern.len) return false;
    return std.mem.eql(u8, data, pattern);
}

test "Shard initialization" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    // Test various sizes
    const sizes = [_]usize{ 64, 1024, 4096 };
    for (sizes) |size| {
        const test_shard = try Shard.init(allocator, size);
        defer test_shard.deinit(allocator);

        try expectEqual(test_shard.getSize(), size);
        try expect(test_shard.isStable());
        try expect(test_shard.getHarmony() > 0.9);
    }
}

test "Data operations" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    const test_shard = try Shard.init(allocator, 1024);
    defer test_shard.deinit(allocator);

    // Test write
    const write_data = "test data pattern";
    try test_shard.write(0, write_data);

    // Test read
    var read_buffer: [16]u8 = undefined;
    try test_shard.read(0, read_buffer[0..write_data.len]);
    try testing.expectEqualStrings(write_data, read_buffer[0..write_data.len]);

    // Test bounds checking
    const result_write = test_shard.write(1020, "too long for space");
    try testing.expectError(ShardError.OutOfBounds, result_write);
}

test "Harmony maintenance" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    const test_shard = try Shard.init(allocator, 1024);
    defer test_shard.deinit(allocator);

    const initial_harmony = test_shard.getHarmony();

    // Perform multiple operations
    var i: usize = 0;
    while (i < 10) : (i += 1) {
        const data = "test data";
        try test_shard.write(i * data.len, data);
        std.time.sleep(10 * std.time.ns_per_ms);
    }

    const final_harmony = test_shard.getHarmony();
    try expect(final_harmony >= 0.8 * initial_harmony);
}

test "Pattern recognition" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    const test_shard = try Shard.init(allocator, 1024);
    defer test_shard.deinit(allocator);

    const test_patterns = [_]TestPattern{
        .{
            .pattern = .Sequential,
            .data = "sequential",
            .offset = 0,
            .expected_harmony = 0.9,
        },
        .{
            .pattern = .Strided,
            .data = "strided",
            .offset = 16,
            .expected_harmony = 0.85,
        },
    };

    for (test_patterns) |tp| {
        test_shard.pattern = tp.pattern;
        try test_shard.write(tp.offset, tp.data);
        try expect(test_shard.getHarmony() >= tp.expected_harmony);
    }
}

test "Resonance stability" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    const test_shard = try Shard.init(allocator, 1024);
    defer test_shard.deinit(allocator);

    // Apply various resonance patterns
    const resonance_levels = [_]f64{ 1.0, 0.8, 0.9 };
    for (resonance_levels) |level| {
        try test_shard.applyResonance(level);
        try expect(test_shard.resonance == level);
        std.time.sleep(10 * std.time.ns_per_ms);
    }

    try expect(test_shard.isStable());
}

test "Access tracking" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    const test_shard = try Shard.init(allocator, 1024);
    defer test_shard.deinit(allocator);

    const initial_count = test_shard.getAccessCount();

    // Perform various accesses
    var i: usize = 0;
    while (i < 5) : (i += 1) {
        try test_shard.write(i * 8, "test");
        var buf: [4]u8 = undefined;
        try test_shard.read(i * 8, &buf);
    }

    const final_count = test_shard.getAccessCount();
    try expect(final_count == initial_count + 10); // 5 writes + 5 reads
}

test "Concurrent access" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    const test_shard = try Shard.init(allocator, 1024);
    defer test_shard.deinit(allocator);

    const thread_count = 4;
    var threads: [thread_count]std.Thread = undefined;

    const thread_fn = struct {
        fn run(shard_ptr: *Shard) !void {
            var i: usize = 0;
            while (i < 10) : (i += 1) {
                try shard_ptr.write(i * 8, "test");
                std.time.sleep(5 * std.time.ns_per_ms);
            }
        }
    }.run;

    // Create threads
    for (&threads) |*thread| {
        thread.* = try std.Thread.spawn(.{}, thread_fn, .{test_shard});
    }

    // Wait for threads
    for (threads) |thread| {
        thread.join();
    }

    try expect(test_shard.isStable());
}

test "Pattern strength evolution" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    const test_shard = try Shard.init(allocator, 1024);
    defer test_shard.deinit(allocator);

    const initial_strength = test_shard.getPatternStrength();

    // Establish a strong sequential pattern
    var i: usize = 0;
    while (i < 20) : (i += 1) {
        try test_shard.write(i * 8, "test");
        std.time.sleep(5 * std.time.ns_per_ms);
    }

    const final_strength = test_shard.getPatternStrength();
    try expect(final_strength > initial_strength);
}

test "Age and lifecycle" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    const test_shard = try Shard.init(allocator, 1024);
    defer test_shard.deinit(allocator);

    const creation_time = test_shard.getAge();
    std.time.sleep(100 * std.time.ns_per_ms);
    const current_age = test_shard.getAge();

    try expect(current_age > creation_time);
    try expect(current_age >= 100);
}
