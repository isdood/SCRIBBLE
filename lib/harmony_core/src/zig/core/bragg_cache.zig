const std = @import("std");
const mathplz = @import("mathplz");

pub const CACHE_LINE_SIZE = 64;
pub const MILLER_MAX = 4;
pub const WAVELENGTH = 1.54056; // Copper Kα radiation

pub const MillerIndices = struct {
    h: i32,
    k: i32,
    l: i32,

    pub fn calculate(vec: Vector3D) MillerIndices {
        return .{
            .h = @intFromFloat(vec.x * MILLER_MAX),
            .k = @intFromFloat(vec.y * MILLER_MAX),
            .l = @intFromFloat(vec.z * MILLER_MAX),
        };
    }
};

pub const Vector3D = struct {
    x: f64 align(CACHE_LINE_SIZE),
    y: f64,
    z: f64,
    intensity: f64,

    pub fn init(x: f64, y: f64, z: f64) Vector3D {
        return .{
            .x = x,
            .y = y,
            .z = z,
            .intensity = 1.0,
        };
    }

    pub fn energy(self: Vector3D) f64 {
        return mathplz.abs(self.x) + mathplz.abs(self.y) + mathplz.abs(self.z);
    }

    pub fn dSpacing(self: Vector3D) f64 {
        const miller = MillerIndices.calculate(self);
        return mathplz.Crystal.calculateDSpacing(
            miller.h,
            miller.k,
            miller.l,
            1.0, // Unit cell parameter
        );
    }
};

pub const DiffractionPattern = struct {
    d_spacing: f64,
    intensity: f64,
    miller: MillerIndices,
    theta: f64,

    pub fn fromVector(vec: Vector3D) DiffractionPattern {
        const miller = MillerIndices.calculate(vec);
        const d = mathplz.Crystal.calculateDSpacing(
            miller.h,
            miller.k,
            miller.l,
            1.0,
        );
        return .{
            .d_spacing = d,
            .intensity = vec.intensity,
            .miller = miller,
            .theta = mathplz.Crystal.calculateBraggAngle(d, WAVELENGTH),
        };
    }
};

const BraggCacheBucket = struct {
    patterns: [8]DiffractionPattern align(CACHE_LINE_SIZE),
    vectors: [8]Vector3D align(CACHE_LINE_SIZE),
    used: [8]bool align(CACHE_LINE_SIZE),
    next: ?*BraggCacheBucket,

    pub fn init(allocator: std.mem.Allocator) !*BraggCacheBucket {
        const bucket = try allocator.create(BraggCacheBucket);
        @memset(&bucket.used, false);
        bucket.next = null;
        return bucket;
    }

    pub fn deinit(self: *BraggCacheBucket, allocator: std.mem.Allocator) void {
        if (self.next) |next| {
            next.deinit(allocator);
        }
        allocator.destroy(self);
    }
};

pub const BraggCache = struct {
    allocator: std.mem.Allocator,
    buckets: std.ArrayList(*BraggCacheBucket),
    vectors: std.ArrayList(Vector3D),
    hit_count: usize,
    miss_count: usize,

    pub fn init(allocator: std.mem.Allocator) BraggCache {
        return .{
            .allocator = allocator,
            .buckets = std.ArrayList(*BraggCacheBucket).init(allocator),
            .vectors = std.ArrayList(Vector3D).init(allocator),
            .hit_count = 0,
            .miss_count = 0,
        };
    }

    pub fn deinit(self: *BraggCache) void {
        for (self.buckets.items) |bucket| {
            bucket.deinit(self.allocator);
        }
        self.buckets.deinit();
        self.vectors.deinit();
    }

    pub fn getMemoryUsage(self: *const BraggCache) usize {
        return self.vectors.items.len * @sizeOf(Vector3D) +
               self.buckets.items.len * @sizeOf(BraggCacheBucket);
    }

    pub fn processVector(self: *BraggCache, vec: Vector3D) !Vector3D {
        const pattern = DiffractionPattern.fromVector(vec);
        
        if (try self.findMatchingPattern(&pattern)) |cached| {
            self.hit_count += 1;
            return cached;
        }

        try self.storePattern(pattern, vec);
        self.miss_count += 1;
        return vec;
    }

    fn findMatchingPattern(self: *BraggCache, pattern: *const DiffractionPattern) !?Vector3D {
        if (self.buckets.items.len == 0) return null;

        const bucket_index = @as(usize, @intFromFloat(pattern.d_spacing * 1000.0)) % self.buckets.items.len;
        var current: ?*BraggCacheBucket = self.buckets.items[bucket_index];

        while (current) |bucket| {
            for (bucket.patterns, bucket.vectors, bucket.used) |p, v, used| {
                if (used and patternMatch(&p, pattern)) {
                    return v;
                }
            }
            current = bucket.next;
        }
        return null;
    }

    fn patternMatch(a: *const DiffractionPattern, b: *const DiffractionPattern) bool {
        const d_spacing_diff = mathplz.abs(a.d_spacing - b.d_spacing);
        const theta_diff = mathplz.abs(a.theta - b.theta);
        return d_spacing_diff < 0.001 and theta_diff < 0.001;
    }

    fn storePattern(self: *BraggCache, pattern: DiffractionPattern, vec: Vector3D) !void {
        if (self.buckets.items.len == 0) {
            const bucket = try BraggCacheBucket.init(self.allocator);
            try self.buckets.append(bucket);
        }

        const bucket_index = @as(usize, @intFromFloat(pattern.d_spacing * 1000.0)) % self.buckets.items.len;
        var current = self.buckets.items[bucket_index];

        while (true) {
            for (0..8) |i| {
                if (!current.used[i]) {
                    current.patterns[i] = pattern;
                    current.vectors[i] = vec;
                    current.used[i] = true;
                    return;
                }
            }

            if (current.next == null) {
                current.next = try BraggCacheBucket.init(self.allocator);
            }
            current = current.next.?;
        }
    }
};
