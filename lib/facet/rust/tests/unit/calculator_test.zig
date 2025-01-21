// test/unit/calculator_test.zig
const std = @import("std");
const testing = std.testing;
const calculator = @import("calculator.zig");

test "basic arithmetic operations" {
    var calc = try calculator.Calculator.init(.{
        .crystal_lattice = try CrystalLattice.init(.{ .clarity = 0.95 }),
                                              .resonance_state = try ResonanceState.init(.{}),
                                              .check_resonance = true,
    });
    defer calc.deinit();

    // Test addition
    const add_result = try calc.compute("2 + 2", .{});
    try testing.expectEqual(add_result.value, 4.0);
    try testing.expect(add_result.clarity > 0.9);

    // Test multiplication with resonance
    const mul_result = try calc.compute("3 * 3", .{
        .check_resonance = true,
        .maintain_resonance = true,
    });
    try testing.expectEqual(mul_result.value, 9.0);
    try testing.expect(mul_result.resonance > 0.85);
}

test "crystal clarity degradation" {
    var calc = try calculator.Calculator.init(.{});
    defer calc.deinit();

    var initial_clarity = calc.config.crystal_lattice.clarity;

    // Perform multiple operations to test clarity degradation
    for (0..100) |_| {
        _ = try calc.compute("1 + 1", .{});
    }

    try testing.expect(calc.config.crystal_lattice.clarity < initial_clarity);
    try testing.expect(calc.config.crystal_lattice.clarity >= 0.85);
}
