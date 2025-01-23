#!/bin/bash
# Initialize Vector-Aligned Shatter Cache System
# Created: 2025-01-23 17:24:38 UTC
# Author: isdood

set -euo pipefail

echo "Cleaning up previous installation..."
rm -rf {src,benches}
rm -f build.zig

echo "Creating project structure..."
mkdir -p {src/zig/core,benches/zig}

echo "Creating build.zig..."
cat > build.zig << 'EOF'
const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});

    const shatter_module = b.addModule("shatter_cache", .{
        .root_source_file = .{ .cwd_relative = "src/zig/core/shatter_cache.zig" },
    });

    const bench = b.addExecutable(.{
        .name = "bench",
        .root_source_file = .{ .cwd_relative = "benches/zig/bench_main.zig" },
        .target = target,
        .optimize = .ReleaseFast,
    });
    bench.root_module.addImport("shatter_cache", shatter_module);
    
    const run_bench = b.addRunArtifact(bench);
    const bench_step = b.step("bench", "Run benchmarks");
    bench_step.dependOn(&run_bench.step);
}
EOF

echo "Creating core implementation..."
cat > src/zig/core/shatter_cache.zig << 'EOF'
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
EOF

echo "Creating benchmark..."
cat > benches/zig/bench_main.zig << 'EOF'
const std = @import("std");
const shatter_cache = @import("shatter_cache");

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    var timer = try std.time.Timer.start();
    
    var cache = shatter_cache.ShatterCache.init(allocator);
    defer cache.deinit();

    const vectors = [_]shatter_cache.Vector3D{
        shatter_cache.Vector3D.init(1, 0, 0),
        shatter_cache.Vector3D.init(0, 1, 0),
        shatter_cache.Vector3D.init(0, 0, 1),
        shatter_cache.Vector3D.init(1, 1, 1),
        shatter_cache.Vector3D.init(-1, 0, 0),
        shatter_cache.Vector3D.init(0, -1, 0),
        shatter_cache.Vector3D.init(0, 0, -1),
        shatter_cache.Vector3D.init(-1, -1, -1),
    };

    const views = [_]shatter_cache.ViewportAngle{
        .{
            .direction = shatter_cache.Vector3D.init(1, 0, 0),
            .up = shatter_cache.Vector3D.init(0, 1, 0),
        },
        .{
            .direction = shatter_cache.Vector3D.init(0, 1, 0),
            .up = shatter_cache.Vector3D.init(0, 0, 1),
        },
    };

    const iterations: usize = 10_000;
    
    std.debug.print("\nBenchmark: Vector Shatter Cache ({d} iterations)\n", .{iterations});
    std.debug.print("Memory usage before: {d} bytes\n", .{cache.getMemoryUsage()});
    
    timer.reset();
    
    var i: usize = 0;
    while (i < iterations) : (i += 1) {
        try cache.preAlignGeometry(1, &vectors, &views);
    }
    
    const elapsed = timer.lap();
    const avg_ns = @as(f64, @floatFromInt(elapsed)) / @as(f64, @floatFromInt(iterations));
    
    std.debug.print("Average: {d:.2} ns/op\n", .{avg_ns});
    std.debug.print("Memory usage after: {d} bytes\n", .{cache.getMemoryUsage()});
    std.debug.print("Vectors processed per second: {d:.2}\n", 
        .{@as(f64, @floatFromInt(vectors.len * iterations)) / (@as(f64, @floatFromInt(elapsed)) / 1e9)});
}
EOF

chmod +x "$0"

echo "Vector-Aligned Shatter Cache system initialized with basic implementation!"
echo "Run benchmarks with: zig build bench"
