const std = @import("std");

pub const CACHE_SIZE: usize = 16;

pub const Vector3D = struct {
    x: f64,
    y: f64,
    z: f64,
};

pub const BraggCache = struct {
    const Entry = struct {
        vector: Vector3D,
        next: ?*Entry,
    };

    entries: [CACHE_SIZE]?*Entry,
    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator) !BraggCache {
        return BraggCache{
            .entries = .{null} ** CACHE_SIZE,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *BraggCache) void {
        for (self.entries) |maybe_entry| {
            var current = maybe_entry;
            while (current) |entry| {
                const next = entry.next;
                self.allocator.destroy(entry);
                current = next;
            }
        }
    }

    pub fn processVector(self: *BraggCache, vec: Vector3D) !void {
        const hash = @as(usize, @intFromFloat(@fabs(vec.x * 73.0))) % CACHE_SIZE;
        var new_entry = try self.allocator.create(Entry);
        new_entry.* = .{
            .vector = vec,
            .next = self.entries[hash],
        };
        self.entries[hash] = new_entry;
    }
};
