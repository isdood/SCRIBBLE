const std = @import("std");
const mathplz = @import("mathplz");

const CACHE_LINE_SIZE = 64;
const CACHE_BUCKET_SIZE = 8;
pub const SYMMETRY_OPS = 24;
pub const QUANTUM_STATES = 8;

pub const Vector3D = struct {
    x: f64 align(CACHE_LINE_SIZE),
    y: f64,
    z: f64,
    state: u8 = 0,

    pub fn init(x: f64, y: f64, z: f64) @This() {
        return .{ .x = x, .y = y, .z = z, .state = 0 };
    }

    pub fn energy(self: @This()) f64 {
        return mathplz.abs(self.x) + mathplz.abs(self.y) + mathplz.abs(self.z);
    }
};

pub const ViewportAngle = struct {
    direction: Vector3D,
    up: Vector3D,
};

const CacheBucket = struct {
    keys: [CACHE_BUCKET_SIZE]u64 align(CACHE_LINE_SIZE),
    values: [CACHE_BUCKET_SIZE]Vector3D align(CACHE_LINE_SIZE),
    used: [CACHE_BUCKET_SIZE]bool align(CACHE_LINE_SIZE),
    next: ?*CacheBucket,

    pub fn init(allocator: std.mem.Allocator) !*CacheBucket {
        const bucket = try allocator.create(CacheBucket);
        @memset(&bucket.keys, 0);
        @memset(&bucket.used, false);
        bucket.next = null;
        return bucket;
    }

    pub fn deinit(self: *CacheBucket, allocator: std.mem.Allocator) void {
        if (self.next) |next| {
            next.deinit(allocator);
        }
        allocator.destroy(self);
    }
};

pub const ShatterCache = struct {
    allocator: std.mem.Allocator,
    vectors: std.ArrayList(Vector3D),
    buckets: std.ArrayList(*CacheBucket),
    hit_count: usize,
    miss_count: usize,

    pub fn init(allocator: std.mem.Allocator) ShatterCache {
        return .{
            .allocator = allocator,
            .vectors = std.ArrayList(Vector3D).init(allocator),
            .buckets = std.ArrayList(*CacheBucket).init(allocator),
            .hit_count = 0,
            .miss_count = 0,
        };
    }

    pub fn deinit(self: *ShatterCache) void {
        for (self.buckets.items) |bucket| {
            bucket.deinit(self.allocator);
        }
        self.buckets.deinit();
        self.vectors.deinit();
    }

    pub fn getMemoryUsage(self: *const ShatterCache) usize {
        return self.vectors.items.len * @sizeOf(Vector3D) +
               self.buckets.items.len * @sizeOf(CacheBucket);
    }

    pub fn preAlignGeometry(self: *ShatterCache, _: u64, vectors: []const Vector3D, _: []const ViewportAngle) !void {
        for (vectors) |vec| {
            const hash = self.hashVector(&vec);
            if (try self.getFromCache(hash)) |cached| {
                try self.vectors.append(cached);
                self.hit_count += 1;
            } else {
                try self.putInCache(hash, vec);
                try self.vectors.append(vec);
                self.miss_count += 1;
            }
        }
    }

    fn hashVector(self: *const ShatterCache, vec: *const Vector3D) u64 {
        _ = self;
        const precision: f64 = 1000.0;
        const x = @as(u64, @intFromFloat(mathplz.abs(vec.x * precision)));
        const y = @as(u64, @intFromFloat(mathplz.abs(vec.y * precision)));
        const z = @as(u64, @intFromFloat(mathplz.abs(vec.z * precision)));
        return x ^ (y << 21) ^ (z << 42);
    }

    fn getFromCache(self: *ShatterCache, hash: u64) !?Vector3D {
        if (self.buckets.items.len == 0) return null;
        
        const bucket_index = hash % self.buckets.items.len;
        var current_bucket: ?*CacheBucket = self.buckets.items[bucket_index];
        
        while (current_bucket) |bucket| {
            for (bucket.keys, bucket.used, bucket.values) |key, used, value| {
                if (used and key == hash) {
                    return value;
                }
            }
            current_bucket = bucket.next;
        }
        return null;
    }

    fn putInCache(self: *ShatterCache, hash: u64, value: Vector3D) !void {
        if (self.buckets.items.len == 0) {
            const bucket = try CacheBucket.init(self.allocator);
            try self.buckets.append(bucket);
        }

        const bucket_index = hash % self.buckets.items.len;
        var current_bucket = self.buckets.items[bucket_index];

        while (true) {
            for (0..CACHE_BUCKET_SIZE) |i| {
                if (!current_bucket.used[i]) {
                    current_bucket.keys[i] = hash;
                    current_bucket.values[i] = value;
                    current_bucket.used[i] = true;
                    return;
                }
            }

            if (current_bucket.next == null) {
                current_bucket.next = try CacheBucket.init(self.allocator);
            }
            current_bucket = current_bucket.next.?;
        }
    }
};
