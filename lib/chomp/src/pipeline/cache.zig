///! Build Cache Manager
///! ================
///! Author: Caleb J.D. Terkovics <isdood>
///! Created: 2025-01-21 02:54:58 UTC
///! License: MIT

const std = @import("std");
const safety = @import("../safety/mod.zig");

pub const Cache = struct {
    allocator: std.mem.Allocator,
    cache_dir: []const u8,
    entries: std.StringHashMap(CacheEntry),

    pub const CacheEntry = struct {
        hash: [32]u8,
        timestamp: i64,
        data: []const u8,
        metadata: std.json.Value,
    };

    pub fn init(allocator: std.mem.Allocator) !*Cache {
        var self = try allocator.create(Cache);
        self.* = .{
            .allocator = allocator,
            .cache_dir = try std.fs.path.join(allocator, &[_][]const u8{
                try std.fs.getApplicationCacheDir(allocator),
                                              "chomp-cache",
            }),
            .entries = std.StringHashMap(CacheEntry).init(allocator),
        };

        try std.fs.makeDirAbsolute(self.cache_dir);
        try self.loadExistingCache();

        return self;
    }

    pub fn get(self: *Cache, key: []const u8) ?CacheEntry {
        return self.entries.get(key);
    }

    pub fn put(self: *Cache, key: []const u8, data: []const u8, metadata: std.json.Value) !void {
        const entry = CacheEntry{
            .hash = try self.calculateHash(data),
            .timestamp = std.time.milliTimestamp(),
            .data = try self.allocator.dupe(u8, data),
            .metadata = metadata,
        };

        try self.entries.put(key, entry);
        try self.persistEntry(key, entry);
    }

    fn loadExistingCache(self: *Cache) !void {
        var dir = try std.fs.openDirAbsolute(self.cache_dir, .{});
        defer dir.close();

        var walker = try dir.walk(self.allocator);
        defer walker.deinit();

        while (try walker.next()) |entry| {
            if (entry.kind != .File) continue;

            const file_data = try std.fs.tree.readFileAlloc(
                self.allocator,
                try std.fs.path.join(self.allocator, &[_][]const u8{
                    self.cache_dir,
                    entry.path,
                }),
                std.math.maxInt(usize),
            );
            defer self.allocator.free(file_data);

            const cache_entry = try std.json.parse(
                CacheEntry,
                &std.json.TokenStream.init(file_data),
                                                   .{
                                                       .allocator = self.allocator,
                                                   },
            );

            try self.entries.put(entry.path, cache_entry);
        }
    }

    fn persistEntry(self: *Cache, key: []const u8, entry: CacheEntry) !void {
        const file_path = try std.fs.path.join(self.allocator, &[_][]const u8{
            self.cache_dir,
            key,
        });

        const file = try std.fs.createFileAbsolute(file_path, .{});
        defer file.close();

        try std.json.stringify(entry, .{}, file.writer());
    }

    fn calculateHash(self: *Cache, data: []const u8) ![32]u8 {
        var hash: [32]u8 = undefined;
        std.crypto.hash.sha2.Sha256.hash(data, &hash, .{});
        return hash;
    }
};
