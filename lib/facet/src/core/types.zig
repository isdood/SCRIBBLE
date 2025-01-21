//! Facet Core Type Definitions
//! Author: @isdood
//! Created: 2025-01-21 16:16:41 UTC

const std = @import("std");

/// Calculation result type
pub const Result = struct {
    /// Calculated value
    value: f64,
    /// Resonance level
    resonance: f64,
    /// Crystal clarity
    clarity: f64,

    const Self = @This();

    /// Initialize new result
    pub fn init(value: f64) Result {
        return .{
            .value = value,
            .resonance = 1.0,
            .clarity = 1.0,
        };
    }

    /// Check if result has perfect resonance
    pub fn isPerfect(self: Self) bool {
        return self.resonance >= 0.99 and self.clarity >= 0.99;
    }

    /// Scale result by factor
    pub fn scale(self: *Self, factor: f64) void {
        self.value *= factor;
        self.resonance = @min(1.0, self.resonance * factor);
        self.clarity = @min(1.0, self.clarity * @fabs(factor));  // Changed from std.math.fabs to @fabs
    }

    /// Combine two results
    pub fn combine(self: Self, other: Self, operation: enum { Add, Multiply }) Result {
        var result = Result{
            .value = switch (operation) {
                .Add => self.value + other.value,
                .Multiply => self.value * other.value,
            },
            .resonance = @min(self.resonance, other.resonance),
            .clarity = @min(self.clarity, other.clarity),
        };

        // Adjust resonance and clarity based on operation
        switch (operation) {
            .Add => {
                result.resonance *= 0.95;  // Small reduction in resonance for addition
                result.clarity *= 0.98;    // Minor clarity loss
            },
            .Multiply => {
                result.resonance *= 0.9;   // Larger reduction for multiplication
                result.clarity *= 0.95;    // More significant clarity loss
            },
        }

        return result;
    }

    /// Format result as string
    pub fn format(
        self: Self,
        comptime fmt: []const u8,
        options: std.fmt.FormatOptions,
        writer: anytype,
    ) !void {
        _ = fmt;
        _ = options;

        try writer.print("{d:.4} (r:{d:.2}, c:{d:.2})", .{
            self.value,
            self.resonance,
            self.clarity,
        });
    }
};

/// Error set for calculation operations
pub const CalcError = error{
    DivisionByZero,
    InvalidOperation,
    ResonanceLoss,
    ClarityTooLow,
    Overflow,
    Underflow,
};

test "result_basic" {
    const result = Result.init(42.0);
    try std.testing.expectEqual(result.value, 42.0);
    try std.testing.expect(result.resonance == 1.0);
    try std.testing.expect(result.clarity == 1.0);
}

test "result_scale" {
    var result = Result.init(10.0);
    result.scale(2.0);
    try std.testing.expectEqual(result.value, 20.0);
    try std.testing.expect(result.resonance <= 1.0);
    try std.testing.expect(result.clarity <= 1.0);
}

test "result_combine" {
    const a = Result.init(5.0);
    const b = Result.init(3.0);

    const sum = a.combine(b, .Add);
    try std.testing.expectEqual(sum.value, 8.0);
    try std.testing.expect(sum.resonance < 1.0);

    const product = a.combine(b, .Multiply);
    try std.testing.expectEqual(product.value, 15.0);
    try std.testing.expect(product.resonance < sum.resonance);
}

test "result_perfect" {
    var result = Result{
        .value = 1.0,
        .resonance = 1.0,
        .clarity = 1.0,
    };
    try std.testing.expect(result.isPerfect());

    result.resonance = 0.98;
    try std.testing.expect(!result.isPerfect());
}
