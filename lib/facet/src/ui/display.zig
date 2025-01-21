//! Facet Display Manager
//! Author: @isdood
//! Created: 2025-01-21 12:51:58 UTC

const std = @import("std");
const Result = @import("../core/types.zig").Result;

/// ANSI color and style configuration
pub const Style = struct {
    // Basic colors
    const reset = "\x1b[0m";
    const bright = "\x1b[1m";
    const dim = "\x1b[2m";
    const italic = "\x1b[3m";

    // Crystal theme colors
    const crystal_clear = "\x1b[38;5;159m";
    const crystal_dim = "\x1b[38;5;152m";
    const harmony_high = "\x1b[38;5;183m";
    const harmony_low = "\x1b[38;5;181m";
    const sparkle = "\x1b[38;5;219m";
    const warning = "\x1b[38;5;214m";
    const error = "\x1b[38;5;203m";

    // Special effects
    const rainbow = [_][]const u8{
        "\x1b[38;5;159m",
        "\x1b[38;5;183m",
        "\x1b[38;5;219m",
        "\x1b[38;5;213m",
        "\x1b[38;5;177m",
    };
};

/// Crystal display configuration
pub const DisplayConfig = struct {
    show_harmony: bool = true,
    show_clarity: bool = true,
    enable_sparkle: bool = true,
    rainbow_threshold: f64 = 0.98,
    minimal_mode: bool = false,
    unicode_support: bool = true,
};

/// Display formatting and effects manager
pub const Display = struct {
    config: DisplayConfig,
    allocator: std.mem.Allocator,
    frame_count: usize,

    const Self = @This();

    /// Initialize new display manager
    pub fn init(allocator: std.mem.Allocator, config: ?DisplayConfig) !*Self {
        const display = try allocator.create(Self);
        display.* = .{
            .config = config orelse DisplayConfig{},
            .allocator = allocator,
            .frame_count = 0,
        };
        return display;
    }

    /// Clean up display resources
    pub fn deinit(self: *Self) void {
        self.allocator.destroy(self);
    }

    /// Format calculation result with crystal effects
    pub fn formatResult(self: *Self, result: Result, writer: anytype) !void {
        // Basic result display
        if (self.config.minimal_mode) {
            try writer.print("{d}\n", .{result.value});
            return;
        }

        // Standard result display with crystal formatting
        try writer.print("{s}Result:{s} {d}\n", .{
            Style.bright,
            Style.reset,
            result.value,
        });

        // Show harmony level if enabled
        if (self.config.show_harmony) {
            const harmony_color = if (result.harmony >= 0.9)
            Style.harmony_high else Style.harmony_low;

            try writer.print("{s}Harmony:{s} {d:.2} ", .{
                harmony_color,
                Style.reset,
                result.harmony,
            });

            try self.drawHarmonyBar(result.harmony, writer);
            try writer.writeByte('\n');
        }

        // Show crystal clarity if enabled
        if (self.config.show_clarity) {
            const clarity_color = if (result.clarity >= 0.9)
            Style.crystal_clear else Style.crystal_dim;

            try writer.print("{s}Clarity:{s} {d:.2} ", .{
                clarity_color,
                Style.reset,
                result.clarity,
            });

            try self.drawClaritySparkles(result.clarity, writer);
            try writer.writeByte('\n');
        }

        // Add special effects for exceptional results
        if (self.config.enable_sparkle) {
            if (result.harmony >= self.config.rainbow_threshold) {
                try self.drawRainbowSparkles(writer);
            } else if (result.harmony >= 0.95) {
                try self.drawSparkles(writer);
            }
        }
    }

    /// Draw harmony level bar
    fn drawHarmonyBar(self: *Self, harmony: f64, writer: anytype) !void {
        if (!self.config.unicode_support) return;

        const bar_length = 20;
        const filled = @floatToInt(usize, harmony * @intToFloat(f64, bar_length));

        try writer.writeAll("[");
        var i: usize = 0;
        while (i < bar_length) : (i += 1) {
            if (i < filled) {
                try writer.print("{s}█{s}", .{ Style.harmony_high, Style.reset });
            } else {
                try writer.print("{s}░{s}", .{ Style.dim, Style.reset });
            }
        }
        try writer.writeAll("]");
    }

    /// Draw crystal clarity sparkles
    fn drawClaritySparkles(self: *Self, clarity: f64, writer: anytype) !void {
        if (!self.config.unicode_support) return;

        const sparkle_count = @floatToInt(usize, clarity * 5);
        var i: usize = 0;
        while (i < sparkle_count) : (i += 1) {
            try writer.print("{s}✦{s}", .{ Style.crystal_clear, Style.reset });
        }
    }

    /// Draw standard sparkle effect
    fn drawSparkles(self: *Self, writer: anytype) !void {
        if (!self.config.unicode_support) return;

        const sparkles = [_]u8{ '✦', '✧', '✴', '✷' };
        for (sparkles) |sparkle| {
            try writer.print("{s}{c}{s} ", .{
                Style.sparkle,
                sparkle,
                Style.reset,
            });
        }
        try writer.writeByte('\n');
    }

    /// Draw rainbow sparkle effect
    fn drawRainbowSparkles(self: *Self, writer: anytype) !void {
        if (!self.config.unicode_support) return;

        const message = "★ Perfect Harmony Achieved! ★";
        const colors = Style.rainbow;

        var i: usize = 0;
        while (i < message.len) : (i += 1) {
            const color_index = (i + self.frame_count) % colors.len;
            try writer.print("{s}{c}{s}", .{
                colors[color_index],
                message[i],
                Style.reset,
            });
        }
        try writer.writeByte('\n');

        self.frame_count +%= 1;
    }

    /// Format error message with crystal theme
    pub fn formatError(self: *Self, err: anyerror, writer: anytype) !void {
        try writer.print("{s}Crystal Error:{s} {s}\n", .{
            Style.error,
            Style.reset,
            @errorName(err),
        });
    }
};

test "display_basic" {
    const allocator = std.testing.allocator;

    const display = try Display.init(allocator, null);
    defer display.deinit();

    var buf: [1024]u8 = undefined;
    var fbs = std.io.fixedBufferStream(&buf);
    const writer = fbs.writer();

    const result = Result{
        .value = 42.0,
        .harmony = 0.95,
        .clarity = 0.97,
    };

    try display.formatResult(result, writer);

    // Basic verification
    const output = fbs.getWritten();
    try std.testing.expect(output.len > 0);
}

test "display_minimal" {
    const allocator = std.testing.allocator;

    const display = try Display.init(allocator, .{
        .minimal_mode = true,
        .show_harmony = false,
        .show_clarity = false,
    });
    defer display.deinit();

    var buf: [1024]u8 = undefined;
    var fbs = std.io.fixedBufferStream(&buf);
    const writer = fbs.writer();

    const result = Result{
        .value = 42.0,
        .harmony = 0.95,
        .clarity = 0.97,
    };

    try display.formatResult(result, writer);

    // Verify minimal output
    const output = fbs.getWritten();
    try std.testing.expectEqualStrings("42\n", output);
}
