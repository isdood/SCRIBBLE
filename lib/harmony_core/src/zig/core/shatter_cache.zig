const std = @import("std");

const Vector3DSIMD = @Vector(4, f64);

pub const Vector3D = struct {
    x: f64,
    y: f64,
    z: f64,

    const Self = @This();

    pub fn init(x: f64, y: f64, z: f64) Self {
        return Self{ .x = x, .y = y, .z = z };
    }

    pub fn toSIMD(self: Self) Vector3DSIMD {
        return Vector3DSIMD{ self.x, self.y, self.z, 0.0 };
    }

    pub fn fromSIMD(vec: Vector3DSIMD) Self {
        return Self{
            .x = vec[0],
            .y = vec[1],
            .z = vec[2],
        };
    }
};

pub const ViewportAngle = struct {
    direction: Vector3D,
    up: Vector3D,
};

pub const ShatterCache = struct {
    allocator: std.mem.Allocator,
    vector_alignments: std.AutoHashMap(u64, std.ArrayList(Vector3D)),
    last_update: i64,

    const Self = @This();

    pub fn init(allocator: std.mem.Allocator) Self {
        return Self{
            .allocator = allocator,
            .vector_alignments = std.AutoHashMap(u64, std.ArrayList(Vector3D)).init(allocator),
            .last_update = std.time.timestamp(),
        };
    }

    pub fn deinit(self: *Self) void {
        var it = self.vector_alignments.valueIterator();
        while (it.next()) |vectors| {
            vectors.deinit();
        }
        self.vector_alignments.deinit();
    }

    pub fn getMemoryUsage(self: *const Self) usize {
        var total: usize = 0;
        var it = self.vector_alignments.iterator();
        while (it.next()) |entry| {
            total += entry.value_ptr.items.len * @sizeOf(Vector3D);
        }
        return total;
    }

    pub fn preAlignGeometry(
        self: *Self, 
        asset_id: u64,
        vectors: []const Vector3D,
        _: []const ViewportAngle,
    ) !void {
        var aligned = std.ArrayList(Vector3D).init(self.allocator);
        errdefer aligned.deinit();

        for (vectors) |vec| {
            try aligned.append(vec);
        }

        if (self.vector_alignments.get(asset_id)) |old_aligned| {
            old_aligned.deinit();
        }
        try self.vector_alignments.put(asset_id, aligned);
        self.last_update = std.time.timestamp();
    }
};
