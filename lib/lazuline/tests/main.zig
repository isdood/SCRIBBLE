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
