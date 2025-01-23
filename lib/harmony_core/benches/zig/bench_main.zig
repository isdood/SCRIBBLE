const std = @import("std");
const shatter_cache = @import("shatter_cache");

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    
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
    
    std.debug.print("\nHybrid Quantum-Crystal Cache Benchmark ({d} iterations)\n", .{iterations});
    std.debug.print("Memory usage before: {d} bytes\n", .{cache.getMemoryUsage()});
    std.debug.print("Features: Crystal Symmetry ({d} ops), Quantum States ({d})\n", 
        .{shatter_cache.SYMMETRY_OPS, shatter_cache.QUANTUM_STATES});
    
    var timer = try std.time.Timer.start();
    
    var i: usize = 0;
    while (i < iterations) : (i += 1) {
        try cache.preAlignGeometry(1, &vectors, &views);
    }
    
    const elapsed = timer.lap();
    const avg_ns = @as(f64, @floatFromInt(elapsed)) / @as(f64, @floatFromInt(iterations));
    
    std.debug.print("\nResults:\n", .{});
    std.debug.print("Average: {d:.2} ns/op\n", .{avg_ns});
    std.debug.print("Memory usage after: {d} bytes\n", .{cache.getMemoryUsage()});
    std.debug.print("Vectors processed per second: {d:.2}\n", 
        .{@as(f64, @floatFromInt(vectors.len * iterations)) / (@as(f64, @floatFromInt(elapsed)) / 1e9)});
    std.debug.print("Cache hit rate: {d:.2}%\n", 
        .{@as(f64, @floatFromInt(cache.hit_count)) / @as(f64, @floatFromInt(cache.hit_count + cache.miss_count)) * 100});
}
