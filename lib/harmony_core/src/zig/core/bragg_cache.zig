const std = @import("std");

pub const Vector3D = struct {
    x: f64,
    y: f64,
    z: f64,

    const EPSILON = 1e-10;
    const GRID_SIZE = 0.01;

    pub fn hash(self: *const Vector3D) u64 {
        if (!self.isValid()) return 0;

        const ix = @as(i64, @intFromFloat(@floor(self.x / GRID_SIZE)));
        const iy = @as(i64, @intFromFloat(@floor(self.y / GRID_SIZE)));
        const iz = @as(i64, @intFromFloat(@floor(self.z / GRID_SIZE)));

        var hasher = std.hash.Wyhash.init(0);
        hasher.update(std.mem.asBytes(&ix));
        hasher.update(std.mem.asBytes(&iy));
        hasher.update(std.mem.asBytes(&iz));
        return hasher.final();
    }

    pub fn eql(self: *const Vector3D, other: *const Vector3D) bool {
        if (!self.isValid() or !other.isValid()) return false;
        return approxEql(self.x, other.x) and
               approxEql(self.y, other.y) and
               approxEql(self.z, other.z);
    }

    fn approxEql(a: f64, b: f64) bool {
        if (std.math.isNan(a) or std.math.isNan(b)) return false;
        if (std.math.isInf(a) or std.math.isInf(b)) return false;
        return @abs(a - b) <= EPSILON;
    }

    pub fn getClusterId(self: *const Vector3D) u3 {
        if (!self.isValid()) return 0;
        const x_bit: u3 = if (self.x >= 0) 1 else 0;
        const y_bit: u3 = if (self.y >= 0) 2 else 0;
        const z_bit: u3 = if (self.z >= 0) 4 else 0;
        return x_bit | y_bit | z_bit;
    }

    pub fn distance(self: *const Vector3D, other: *const Vector3D) f64 {
        if (!self.isValid() or !other.isValid()) return std.math.inf(f64);
        const dx = self.x - other.x;
        const dy = self.y - other.y;
        const dz = self.z - other.z;
        return @sqrt(dx * dx + dy * dy + dz * dz);
    }

    pub fn isValid(self: *const Vector3D) bool {
        return !std.math.isNan(self.x) and !std.math.isInf(self.x) and
               !std.math.isNan(self.y) and !std.math.isInf(self.y) and
               !std.math.isNan(self.z) and !std.math.isInf(self.z);
    }

    pub fn magnitude(self: *const Vector3D) f64 {
        if (!self.isValid()) return 0;
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }
};

pub const BraggCache = struct {
    const Self = @This();

    pub const Stats = struct {
        hits: usize = 0,
        misses: usize = 0,
        stored_vectors: usize = 0,
        clusters: [8]usize = .{0} ** 8,
        invalid_vectors: usize = 0,
    };

    const VectorContext = struct {
        timestamp: u64,
        cluster_id: u3,
    };

    vectors: std.AutoHashMap(*const Vector3D, VectorContext),
    stored: std.ArrayList(Vector3D),
    cluster_neighbors: [8]usize,
    stats: Stats,
    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator) !Self {
        var self = Self{
            .vectors = std.AutoHashMap(*const Vector3D, VectorContext).init(allocator),
            .stored = std.ArrayList(Vector3D).init(allocator),
            .cluster_neighbors = .{0} ** 8,
            .stats = Stats{},
            .allocator = allocator,
        };
        try self.vectors.ensureTotalCapacity(1024);  // Pre-allocate some space
        try self.stored.ensureTotalCapacity(1024);
        return self;
    }

    pub fn deinit(self: *Self) void {
        self.vectors.deinit();
        self.stored.deinit();
    }

    pub fn processVector(self: *Self, vec: Vector3D) !f64 {
        if (!vec.isValid()) {
            self.stats.invalid_vectors += 1;
            return 0;
        }

        const magnitude = vec.magnitude();

        // Store vector safely
        const stored_index = self.stored.items.len;
        try self.stored.append(vec);
        if (stored_index >= self.stored.items.len) {
            return error.StorageError;
        }

        const stored_vec = &self.stored.items[stored_index];
        const cluster_id = stored_vec.getClusterId();

        // Update cluster statistics safely
        if (@as(usize, cluster_id) < self.stats.clusters.len) {
            self.stats.clusters[@as(usize, cluster_id)] += 1;
        }

        var found_match = false;
        var min_dist = std.math.inf(f64);

        var iter = self.vectors.iterator();
        while (iter.next()) |entry| {
            const other_vec = entry.key_ptr.*;
            if (!other_vec.isValid()) continue;

            const other_ctx = entry.value_ptr.*;
            if (other_ctx.cluster_id == cluster_id) {
                const dist = stored_vec.distance(other_vec);
                if (!std.math.isInf(dist) and !std.math.isNan(dist)) {
                    min_dist = @min(min_dist, dist);
                    if (dist <= Vector3D.GRID_SIZE) {
                        found_match = true;
                        if (@as(usize, cluster_id) < self.cluster_neighbors.len) {
                            self.cluster_neighbors[@as(usize, cluster_id)] += 1;
                        }
                        break;
                    }
                }
            }
        }

        if (found_match) {
            self.stats.hits += 1;
        } else {
            try self.vectors.put(stored_vec, .{
                .timestamp = @intCast(stored_index),
                .cluster_id = cluster_id,
            });
            self.stats.misses += 1;
            self.stats.stored_vectors += 1;
        }

        return magnitude;
    }

    pub fn iterator(self: *Self) VectorIterator {
        return VectorIterator{ .items = self.stored.items };
    }

    pub fn getCacheStats(self: *Self) Stats {
        return self.stats;
    }

    pub fn validate(self: *Self) struct {
        sum: f64,
        magnitude: f64,
        cluster_distribution: [8]f64,
        neighbor_counts: [8]usize,
        invalid_count: usize,
    } {
        var sum: f64 = 0;
        var magnitude: f64 = 0;
        var total_vectors: f64 = 0;
        var cluster_dist = [_]f64{0} ** 8;

        for (self.stored.items) |vec| {
            const v = vec;
            if (v.isValid()) {
                sum += v.x + v.y + v.z;
                magnitude += v.magnitude();
                total_vectors += 1;
            }
        }

        if (total_vectors > 0) {
            for (self.stats.clusters, 0..) |count, i| {
                cluster_dist[i] = @as(f64, @floatFromInt(count)) / total_vectors * 100;
            }
        }

        return .{
            .sum = sum,
            .magnitude = magnitude,
            .cluster_distribution = cluster_dist,
            .neighbor_counts = self.cluster_neighbors,
            .invalid_count = self.stats.invalid_vectors,
        };
    }
};

pub const VectorIterator = struct {
    items: []const Vector3D,
    current: usize = 0,

    pub fn next(self: *VectorIterator) ?Vector3D {
        if (self.current >= self.items.len) return null;
        const vec = self.items[self.current];
        self.current += 1;
        return vec;
    }
};
