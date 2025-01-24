const std = @import("std");
const builtin = @import("builtin");

pub const CACHE_SIZE: usize = 16;
const PREALLOCATE_SIZE: usize = 8192;

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
    free_list: ?*Entry,
    arena: std.heap.ArenaAllocator,

    comptime {
        if (builtin.mode == .Debug) {
            @compileError("This code must be compiled with -O ReleaseFast for maximum performance");
        }
    }

    pub fn init(allocator: std.mem.Allocator) !BraggCache {
        var self = BraggCache{
            .entries = .{null} ** CACHE_SIZE,
            .free_list = null,
            .arena = std.heap.ArenaAllocator.init(allocator),
        };

        // Bulk pre-allocation
        var entries = try self.arena.allocator().alloc(Entry, PREALLOCATE_SIZE);
        var i: usize = 0;
        while (i < PREALLOCATE_SIZE - 1) : (i += 1) {
            entries[i].next = &entries[i + 1];
        }
        entries[PREALLOCATE_SIZE - 1].next = null;
        self.free_list = &entries[0];

        return self;
    }

    pub fn deinit(self: *BraggCache) void {
        self.arena.deinit();
    }

    inline fn getEntry(self: *BraggCache) ?*Entry {
        if (self.free_list) |entry| {
            self.free_list = entry.next;
            return entry;
        }
        return null;
    }

    inline fn hashVector(vec: Vector3D) usize {
        const bits = @as(u64, @bitCast(vec.x));
        return @as(usize, @truncate(bits)) & (CACHE_SIZE - 1);
    }

    pub inline fn processVector(self: *BraggCache, vec: Vector3D) !void {
        if (self.getEntry()) |new_entry| {
            const hash = hashVector(vec);
            new_entry.* = .{
                .vector = vec,
                .next = self.entries[hash],
            };
            self.entries[hash] = new_entry;
        }
    }
};
