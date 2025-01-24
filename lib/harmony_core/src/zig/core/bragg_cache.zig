const std = @import("std");

pub const CACHE_SIZE: usize = 16;

pub const Vector3D = struct {
    x: f64,
    y: f64,
    z: f64,

    pub fn init(x: f64, y: f64, z: f64) Vector3D {
        return .{ .x = x, .y = y, .z = z };
    }
};

pub const BraggCache = struct {
    const Entry = struct {
        vector: Vector3D,
        next: ?*Entry,
    };

    entries: [CACHE_SIZE]?*Entry,
    allocator: std.mem.Allocator,
    hit_count: usize,
    miss_count: usize,
    debug_enabled: bool,

    pub fn init(allocator: std.mem.Allocator) BraggCache {
        return .{
            .entries = .{null} ** CACHE_SIZE,
            .allocator = allocator,
            .hit_count = 0,
            .miss_count = 0,
            .debug_enabled = false,
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

    pub fn calculateHash(self: *const BraggCache, vec: Vector3D) usize {
        _ = self;
        const h1 = @as(usize, @intFromFloat(@abs(vec.x * 73.0)));
        const h2 = @as(usize, @intFromFloat(@abs(vec.y * 37.0)));
        const h3 = @as(usize, @intFromFloat(@abs(vec.z * 59.0)));
        return (h1 +% h2 +% h3) % CACHE_SIZE;
    }

    pub fn processVector(self: *BraggCache, vec: Vector3D) !void {
        const hash = self.calculateHash(vec);
        var current = self.entries[hash];

        while (current) |entry| {
            if (entry.vector.x == vec.x and
                entry.vector.y == vec.y and
                entry.vector.z == vec.z) {
                self.hit_count += 1;
                return;
            }
            current = entry.next;
        }

        self.miss_count += 1;
        if (self.debug_enabled) {
            std.debug.print("Initial cache miss for vector ({d}, {d}, {d}) -> hash: {d}\n",
                .{ vec.x, vec.y, vec.z, hash });
        }

        var new_entry = try self.allocator.create(Entry);
        new_entry.* = .{
            .vector = vec,
            .next = self.entries[hash],
        };
        self.entries[hash] = new_entry;
    }

    pub fn getMemoryUsage(self: *const BraggCache) usize {
        var total: usize = @sizeOf(BraggCache);
        for (self.entries) |maybe_entry| {
            var current = maybe_entry;
            while (current) |entry| {
                total += @sizeOf(Entry);
                current = entry.next;
            }
        }
        return total;
    }
};
