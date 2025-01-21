// crystal_test.zig - Crystal system tests for Prism
// Created by: isdood
// Date: 2025-01-21 11:18:34 UTC

const std = @import("std");
const testing = std.testing;
const math = std.math;
const Crystal = @import("crystal").Crystal;
const Pattern = @import("pattern").Pattern;
const Node = @import("crystal").Node;

test "crystal initialization" {
    const allocator = testing.allocator;
    var crystal = try Crystal.init(allocator, .Cubic);
    defer crystal.deinit();

    try testing.expect(crystal.isInitialized());
    try testing.expectEqual(crystal.getSystem(), .Cubic);
    try testing.expectEqual(crystal.getNodeCount(), 0);
}

test "node management" {
    const allocator = testing.allocator;
    var crystal = try Crystal.init(allocator, .Cubic);
    defer crystal.deinit();

    // Add nodes
    const positions = [_][3]f64{
        .{ 0.0, 0.0, 0.0 },
        .{ 1.0, 0.0, 0.0 },
        .{ 0.0, 1.0, 0.0 },
    };

    var nodes = std.ArrayList(*Node).init(allocator);
    defer nodes.deinit();

    for (positions) |pos| {
        const node = try crystal.addNode(pos);
        try nodes.append(node);
    }

    try testing.expectEqual(crystal.getNodeCount(), 3);

    // Verify positions
    for (nodes.items, positions) |node, pos| {
        try testing.expectEqual(node.position, pos);
    }

    // Remove nodes
    for (nodes.items) |node| {
        try crystal.removeNode(node);
    }

    try testing.expectEqual(crystal.getNodeCount(), 0);
}

test "crystal stability" {
    const allocator = testing.allocator;
    var crystal = try Crystal.init(allocator, .Cubic);
    defer crystal.deinit();

    // Create perfect cubic lattice
    const spacing = 1.0;
    var x: usize = 0;
    while (x < 2) : (x += 1) {
        var y: usize = 0;
        while (y < 2) : (y += 1) {
            var z: usize = 0;
            while (z < 2) : (z += 1) {
                const pos = [3]f64{
                    @as(f64, @floatFromInt(x)) * spacing,
                    @as(f64, @floatFromInt(y)) * spacing,
                    @as(f64, @floatFromInt(z)) * spacing,
                };
                _ = try crystal.addNode(pos);
            }
        }
    }

    const stability = crystal.getStability();
    try testing.expect(stability > 0.9); // Perfect cubic lattice should be very stable
}

test "crystal optimization" {
    const allocator = testing.allocator;
    var crystal = try Crystal.init(allocator, .Cubic);
    defer crystal.deinit();

    // Add slightly perturbed nodes
    const base_positions = [_][3]f64{
        .{ 0.0, 0.0, 0.0 },
        .{ 1.0, 0.0, 0.0 },
        .{ 0.0, 1.0, 0.0 },
        .{ 1.0, 1.0, 0.0 },
    };

    for (base_positions) |pos| {
        var perturbed_pos = pos;
        perturbed_pos[0] += 0.1 * (std.crypto.random.float(f64) - 0.5);
        perturbed_pos[1] += 0.1 * (std.crypto.random.float(f64) - 0.5);
        perturbed_pos[2] += 0.1 * (std.crypto.random.float(f64) - 0.5);
        _ = try crystal.addNode(perturbed_pos);
    }

    const initial_stability = crystal.getStability();
    try crystal.optimize();
    const final_stability = crystal.getStability();

    try testing.expect(final_stability > initial_stability);
}

test "crystal pattern generation" {
    const allocator = testing.allocator;
    var crystal = try Crystal.init(allocator, .Cubic);
    defer crystal.deinit();

    var pattern = try Pattern.init(allocator, .{
        .pattern_type = .Cubic,
        .spacing = 1.0,
        .scale = 1.0,
        .rotation = .{ 0.0, 0.0, 0.0 },
        .symmetry = 8,
    });
    defer pattern.deinit();

    try pattern.generate(&crystal);
    try testing.expect(crystal.getNodeCount() > 0);

    const stability = crystal.getStability();
    try testing.expect(stability > 0.0);
}

test "crystal transformations" {
    const allocator = testing.allocator;
    var crystal = try Crystal.init(allocator, .Cubic);
    defer crystal.deinit();

    // Add some nodes in a square
    const positions = [_][3]f64{
        .{ 0.0, 0.0, 0.0 },
        .{ 1.0, 0.0, 0.0 },
        .{ 0.0, 1.0, 0.0 },
        .{ 1.0, 1.0, 0.0 },
    };

    for (positions) |pos| {
        _ = try crystal.addNode(pos);
    }

    // Rotate crystal
    try crystal.rotate(.{ math.pi / 4.0, 0.0, 0.0 });
    try crystal.optimize();

    // Scale crystal
    try crystal.scale(2.0);
    try testing.expect(crystal.getStability() > 0.0);
}

test "crystal energy calculation" {
    const allocator = testing.allocator;
    var crystal = try Crystal.init(allocator, .Cubic);
    defer crystal.deinit();

    // Add two nodes
    const node1 = try crystal.addNode(.{ 0.0, 0.0, 0.0 });
    const node2 = try crystal.addNode(.{ 1.0, 0.0, 0.0 });

    const energy = crystal.calculateEnergy();
    try testing.expect(energy >= 0.0);

    // Test energy change with distance
    try crystal.moveNode(node2, .{ 2.0, 0.0, 0.0 });
    const new_energy = crystal.calculateEnergy();
    try testing.expect(new_energy != energy);
}

test "crystal system transitions" {
    const allocator = testing.allocator;
    var crystal = try Crystal.init(allocator, .Cubic);
    defer crystal.deinit();

    // Add nodes in cubic pattern
    var pattern = try Pattern.init(allocator, .{
        .pattern_type = .Cubic,
        .spacing = 1.0,
        .scale = 1.0,
        .rotation = .{ 0.0, 0.0, 0.0 },
        .symmetry = 8,
    });
    defer pattern.deinit();

    try pattern.generate(&crystal);
    const cubic_stability = crystal.getStability();

    // Transition to hexagonal
    try crystal.transitionTo(.Hexagonal);
    try crystal.optimize();
    const hex_stability = crystal.getStability();

    try testing.expect(hex_stability > 0.0);
    try testing.expect(crystal.getSystem() == .Hexagonal);
}

test "crystal stress test" {
    const allocator = testing.allocator;
    var crystal = try Crystal.init(allocator, .Cubic);
    defer crystal.deinit();

    // Add many nodes
    var i: usize = 0;
    const node_count = 1000;
    while (i < node_count) : (i += 1) {
        const pos = [3]f64{
            std.crypto.random.float(f64) * 10.0,
            std.crypto.random.float(f64) * 10.0,
            std.crypto.random.float(f64) * 10.0,
        };
        _ = try crystal.addNode(pos);
    }

    try testing.expectEqual(crystal.getNodeCount(), node_count);
    try crystal.optimize();
    try testing.expect(crystal.getStability() > 0.0);
}

test "crystal serialization" {
    const allocator = testing.allocator;
    var crystal = try Crystal.init(allocator, .Cubic);
    defer crystal.deinit();

    // Add some nodes
    _ = try crystal.addNode(.{ 0.0, 0.0, 0.0 });
    _ = try crystal.addNode(.{ 1.0, 0.0, 0.0 });

    // Serialize
    var buffer = std.ArrayList(u8).init(allocator);
    defer buffer.deinit();
    try crystal.serialize(buffer.writer());

    // Deserialize
    var new_crystal = try Crystal.deserialize(allocator, buffer.reader());
    defer new_crystal.deinit();

    try testing.expectEqual(crystal.getNodeCount(), new_crystal.getNodeCount());
    try testing.expectEqual(crystal.getSystem(), new_crystal.getSystem());
    try testing.expect(math.approxEqAbs(f64, 
        crystal.getStability(), 
        new_crystal.getStability(),
        0.0001));
}
