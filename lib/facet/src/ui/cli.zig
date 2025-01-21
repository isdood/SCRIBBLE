//! Facet CLI Interface
//! Author: @isdood
//! Created: 2025-01-21 15:30:29 UTC

const std = @import("std");
const Calculator = @import("../core/calculator.zig").Calculator;
const Result = @import("../core/types.zig").Result;

/// CLI color configuration
const Color = struct {
    const reset = "\x1b[0m";
    const bright = "\x1b[1m";
    const crystal = "\x1b[38;5;159m";
    const harmony = "\x1b[38;5;183m";
    const sparkle = "\x1b[38;5;219m";
    const error_color = "\x1b[38;5;203m";  // Changed from 'error' to 'error_color'
};

// Rest of the code remains the same, but update references to Color.error:
/// Display error message
fn displayError(self: *Self, err: anyerror) !void {
    const stdout = std.io.getStdOut().writer();
    try stdout.print("{s}Error:{s} {s}\n", .{
        Color.error_color,  // Updated here
        Color.reset,
        @errorName(err),
    });
}

/// Display result
pub fn displayResult(self: *Self, result: Result) !void {
    const stdout = std.io.getStdOut().writer();

    // Display main result
    try stdout.print("{s}Result:{s} {d}\n", .{
        Color.bright,
        Color.reset,
        result.value,
    });

    // Display harmony metrics if enabled
    if (self.config.show_harmony) {
        try stdout.print("{s}Harmony:{s} {d:.2}\n", .{
            Color.harmony,
            Color.reset,
            result.harmony,
        });
    }

    // Display crystal clarity if enabled
    if (self.config.show_clarity) {
        try stdout.print("{s}Clarity:{s} {d:.2}\n", .{
            Color.crystal,
            Color.reset,
            result.clarity,
        });
    }

    // Add sparkle effect if enabled
    if (self.config.enable_sparkle and result.harmony >= 0.95) {
        try stdout.print("{s}✨ Perfect Harmony! ✨{s}\n", .{
            Color.sparkle,
            Color.reset,
        });
    }
}

/// Display welcome message
fn displayWelcome(self: *Self) !void {
    const stdout = std.io.getStdOut().writer();
    try stdout.print(
        \\{s}Welcome to Facet - Crystal Calculator{s}
        \\{s}✨ Version 0.1.0 ✨{s}
        \\Type 'help' for commands, 'exit' to quit
        \\
        \\
        , .{
            Color.crystal,
            Color.reset,
            Color.sparkle,
            Color.reset,
        });
}

/// Display farewell message
fn displayFarewell(self: *Self) !void {
    const stdout = std.io.getStdOut().writer();
    try stdout.print(
        \\{s}Thank you for using Facet!{s}
        \\{s}✨ May your calculations stay harmonious ✨{s}
        \\
        , .{
            Color.crystal,
            Color.reset,
            Color.sparkle,
            Color.reset,
        });
}

/// Display help message
fn displayHelp(self: *Self) !void {
    const stdout = std.io.getStdOut().writer();
    try stdout.print(
        \\{s}Facet Commands:{s}
        \\  help  - Display this help message
        \\  exit  - Exit the calculator
        \\  quit  - Same as exit
        \\
        \\{s}Examples:{s}
        \\  2 + 2
        \\  5 * (3 + 2)
    \\  10 / 2
    \\
    \\{s}Crystal Tips:{s}
    \\  - Higher harmony leads to more accurate results
    \\  - Maintain crystal clarity for best performance
    \\  - Watch for the sparkle effect on perfect calculations!
    \\
    , .{
        Color.bright,
        Color.reset,
        Color.crystal,
        Color.reset,
        Color.harmony,
        Color.reset,
    });
}
