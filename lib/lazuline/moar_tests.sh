# Update tests/main.zig with more comprehensive tests
cat > tests/main.zig << 'EOF'
//! Crystal Wave Runtime Tests
//! Created: 2025-01-22 01:34:33
//! Author: isdood

const std = @import("std");
const testing = std.testing;
const lazuline = @import("lazuline");

test "basic wave runtime initialization" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var runtime = try lazuline.runtime.WaveRuntime.init(allocator);
    defer runtime.deinit();

    // Basic initialization tests
    try testing.expect(runtime.wave_computer.waves.items.len == 0);
    try testing.expect(runtime.lattice.nodes.items.len == 0);
}

test "wave function interference" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var wave1 = try lazuline.wave.WaveFunction.init(allocator, 4);
    defer wave1.deinit();
    var wave2 = try lazuline.wave.WaveFunction.init(allocator, 4);
    defer wave2.deinit();

    // Set up wave patterns
    wave1.amplitude[0] = 1.0;
    wave1.amplitude[1] = 0.5;
    wave1.amplitude[2] = 0.25;
    wave1.amplitude[3] = 0.125;

    wave2.amplitude[0] = 0.5;
    wave2.amplitude[1] = 0.25;
    wave2.amplitude[2] = 0.125;
    wave2.amplitude[3] = 0.0625;

    // Test interference
    wave1.interfere(&wave2);

    try testing.expectEqual(wave1.amplitude[0], 1.5);
    try testing.expectEqual(wave1.amplitude[1], 0.75);
    try testing.expectEqual(wave1.amplitude[2], 0.375);
    try testing.expectEqual(wave1.amplitude[3], 0.1875);
}

test "crystal lattice operations" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var lattice = try lazuline.lattice.CrystalLattice.init(allocator);
    defer lattice.deinit();

    // Test lattice initialization
    try testing.expect(lattice.nodes.items.len == 0);
    try testing.expect(lattice.connections.items.len == 0);
}

test {
    _ = @import("std").testing.refAllDecls(@This());
}
EOF

# Update bench/main.zig with more comprehensive benchmarks
cat > bench/main.zig << 'EOF'
//! Crystal Wave Runtime Benchmarks
//! Created: 2025-01-22 01:34:33
//! Author: isdood

const std = @import("std");
const lazuline = @import("lazuline");

fn benchmarkWaveInterference(allocator: std.mem.Allocator, size: usize, iterations: usize) !u64 {
    var wave1 = try lazuline.wave.WaveFunction.init(allocator, size);
    defer wave1.deinit();
    var wave2 = try lazuline.wave.WaveFunction.init(allocator, size);
    defer wave2.deinit();

    // Initialize waves
    for (0..size) |i| {
        wave1.amplitude[i] = @as(f64, @floatFromInt(i)) / @as(f64, @floatFromInt(size));
        wave2.amplitude[i] = 1.0 - wave1.amplitude[i];
    }

    var timer = try std.time.Timer.start();
    const start = timer.lap();

    // Benchmark interference operations
    for (0..iterations) |_| {
        wave1.interfere(&wave2);
    }

    return timer.lap() - start;
}

pub fn main() !void {
    // Setup
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    // Benchmark parameters
    const sizes = [_]usize{ 64, 256, 1024, 4096 };
    const iterations: usize = 100_000;

    // Run benchmarks
    std.debug.print("\nCrystal Wave Runtime Benchmarks\n", .{});
    std.debug.print("==========================\n\n", .{});

    for (sizes) |size| {
        const elapsed = try benchmarkWaveInterference(allocator, size, iterations);
        const ns_per_op = @as(f64, @floatFromInt(elapsed)) / @as(f64, @floatFromInt(iterations));

        std.debug.print("Wave Interference Benchmark (size={}):\n", .{size});
        std.debug.print("  Operations: {d}\n", .{iterations});
        std.debug.print("  Total time: {d}ns\n", .{elapsed});
        std.debug.print("  {d:.2} ns/op\n\n", .{ns_per_op});
    }
}
EOF

echo "[BUILD] Updated tests with comprehensive test cases"
echo "[BUILD] Updated benchmarks with wave interference performance tests"
echo "[BUILD] Try running tests and benchmarks:"
echo "  zig build test    # Run tests"
echo "  zig build bench   # Run benchmarks"
