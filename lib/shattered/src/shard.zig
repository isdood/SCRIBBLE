//! Shard Implementation for Shattered Cache
//! Created: 2025-01-21 04:33:05 UTC
//! Author: isdood
//! 
//! Implements the fundamental shard type that represents a fragment
//! of crystallized data in the shattered cache system.

const std = @import("std");
const Allocator = std.mem.Allocator;
const Random = std.rand.Random;
const AtomicU64 = std.atomic.Atomic(u64);
const Time = std.time;

const patterns = @import("patterns.zig");
const harmony = @import("harmony.zig");

const AccessPattern = patterns.AccessPattern;
const HarmonyState = harmony.HarmonyState;

/// Error set for shard operations
pub const ShardError = error{
    HarmonyLost,
    InvalidAccess,
    ResonanceMismatch,
    OutOfBounds,
    AllocationFailed,
};

/// Metadata for shard tracking
const ShardMeta = struct {
    created_at: i64,
    last_access: AtomicU64,
    access_count: AtomicU64,
    size: usize,
    alignment: usize,
};

/// A shard represents a fragment of the crystal cache
pub const Shard = struct {
    /// Unique identifier
    id: u64,
    /// Raw data storage
    data: []align(64) u8,
    /// Current access pattern
    pattern: AccessPattern,
    /// Harmony state
    harmony_state: HarmonyState,
    /// Current resonance level
    resonance: f64,
    /// Metadata
    meta: ShardMeta,
    /// Memory allocator
    allocator: *Allocator,

    const Self = @This();

    /// Initialize a new shard
    pub fn init(allocator: *Allocator, size: usize) !*Self {
        const shard = try allocator.create(Self);
        errdefer allocator.destroy(shard);

        // Align data to cache line
        const aligned_data = try allocator.alignedAlloc(u8, 64, size);
        errdefer allocator.free(aligned_data);

        shard.* = .{
            .id = generateId(),
            .data = aligned_data,
            .pattern = .Sequential,
            .harmony_state = try HarmonyState.init(),
            .resonance = 1.0,
            .meta = .{
                .created_at = Time.milliTimestamp(),
                .last_access = AtomicU64.init(0),
                .access_count = AtomicU64.init(0),
                .size = size,
                .alignment = 64,
            },
            .allocator = allocator,
        };

        return shard;
    }

    /// Clean up shard resources
    pub fn deinit(self: *Self, allocator: *Allocator) void {
        allocator.free(self.data);
        allocator.destroy(self);
    }

    /// Read data from the shard
    pub fn read(self: *Self, offset: usize, buffer: []u8) !void {
        if (offset + buffer.len > self.meta.size) {
            return ShardError.OutOfBounds;
        }

        @memcpy(buffer, self.data[offset..][0..buffer.len]);
        self.recordAccess();
        try self.updateHarmony();
    }

    /// Write data to the shard
    pub fn write(self: *Self, offset: usize, data: []const u8) !void {
        if (offset + data.len > self.meta.size) {
            return ShardError.OutOfBounds;
        }

        @memcpy(self.data[offset..][0..data.len], data);
        self.recordAccess();
        try self.updateHarmony();
    }

    /// Apply resonance pattern to shard
    pub fn applyResonance(self: *Self, resonance: f64) !void {
        try self.harmony_state.applyResonance(resonance);
        self.resonance = resonance;
    }

    /// Get current harmony level
    pub fn getHarmony(self: Self) f64 {
        return self.harmony_state.getHarmonyMetric();
    }

    /// Check if shard is stable
    pub fn isStable(self: Self) bool {
        return self.harmony_state.isStable();
    }

    /// Get shard's access pattern strength
    pub fn getPatternStrength(self: Self) f64 {
        return self.harmony_state.pattern_strength;
    }

    /// Get shard size
    pub fn getSize(self: Self) usize {
        return self.meta.size;
    }

    /// Get access count
    pub fn getAccessCount(self: Self) u64 {
        return self.meta.access_count.load(.Monotonic);
    }

    /// Get age in milliseconds
    pub fn getAge(self: Self) i64 {
        return Time.milliTimestamp() - self.meta.created_at;
    }

    // Private methods

    fn generateId() u64 {
        var prng = std.rand.DefaultPrng.init(@intCast(u64, Time.milliTimestamp()));
        return prng.random().int(u64);
    }

    fn recordAccess(self: *Self) void {
        _ = self.meta.access_count.fetchAdd(1, .Monotonic);
        _ = self.meta.last_access.store(
            @intCast(u64, Time.milliTimestamp()),
            .Release
        );
    }

    fn updateHarmony(self: *Self) !void {
        try self.harmony_state.evolve(432.0); // Base resonance frequency
        
        // Adjust pattern strength based on access history
        const access_interval = @intToFloat(f64, 
            Time.milliTimestamp() - 
            @intCast(i64, self.meta.last_access.load(.Acquire))
        ) / 1000.0;

        if (access_interval > 0.0) {
            const pattern_factor = switch (self.pattern) {
                .Sequential => 1.0,
                .Strided => 0.9,
                .Random => 0.7,
                .Clustered => 0.85,
                .Hybrid => 0.8,
            };

            try self.applyResonance(pattern_factor * self.resonance);
        }
    }
};

test "Shard basic operations" {
    const testing = std.testing;
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    // Create new shard
    const shard = try Shard.init(allocator, 1024);
    defer shard.deinit(allocator);

    // Test initial state
    try testing.expect(shard.isStable());
    try testing.expect(shard.getHarmony() > 0.8);
    try testing.expectEqual(shard.getSize(), 1024);

    // Test read/write
    const write_data = "test data";
    try shard.write(0, write_data);
    var read_buffer: [9]u8 = undefined;
    try shard.read(0, &read_buffer);
    try testing.expectEqualStrings(write_data, &read_buffer);

    // Test access tracking
    try testing.expect(shard.getAccessCount() == 2);
    try testing.expect(shard.getAge() >= 0);
}

test "Shard harmony maintenance" {
    const testing = std.testing;
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    const shard = try Shard.init(allocator, 1024);
    defer shard.deinit(allocator);

    // Test resonance application
    try shard.applyResonance(432.0);
    try testing.expect(shard.resonance == 432.0);

    // Test pattern strength
    try testing.expect(shard.getPatternStrength() > 0.7);
}
