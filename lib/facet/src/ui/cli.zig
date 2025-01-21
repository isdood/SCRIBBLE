//! Facet CLI Interface
//! Author: @isdood
//! Created: 2025-01-21 15:45:02 UTC

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

/// CLI configuration options
const Config = struct {
    /// Show crystal clarity metrics
    show_clarity: bool = true,
    /// Display harmony levels
    show_harmony: bool = true,
    /// Enable sparkle effects
    enable_sparkle: bool = true,
    /// History size
    history_size: usize = 100,
};

/// CLI state and functionality
pub const CLI = struct {
    calculator: Calculator,
    config: Config,
    history: std.ArrayList([]const u8),
    allocator: std.mem.Allocator,

    const Self = @This();  // Added this line to define Self

    /// Initialize new CLI instance
    pub fn init(options: struct {
        calculator: Calculator,
        config: ?Config = null,
        allocator: ?std.mem.Allocator = null,
    }) !*Self {
        const alloc = options.allocator orelse std.heap.page_allocator;
        const cli = try alloc.create(Self);

        cli.* = .{
            .calculator = options.calculator,
            .config = options.config orelse Config{},
            .history = std.ArrayList([]const u8).init(alloc),
            .allocator = alloc,
        };

        return cli;
    }

    /// Clean up CLI resources
    pub fn deinit(self: *Self) void {
        for (self.history.items) |item| {
            self.allocator.free(item);
        }
        self.history.deinit();
        self.allocator.destroy(self);
    }

    /// Run interactive CLI mode
    pub fn runInteractive(self: *Self) !void {
        const stdout = std.io.getStdOut().writer();
        const stdin = std.io.getStdIn().reader();

        try self.displayWelcome();

        var buf: [1024]u8 = undefined;
        while (true) {
            // Display prompt
            try stdout.print("{s}crystal>{s} ", .{ Color.crystal, Color.reset });

            // Read input
            const input = (try stdin.readUntilDelimiter(&buf, '\n')) orelse break;
            if (input.len == 0) continue;

            // Handle special commands
            if (std.mem.eql(u8, input, "exit") or std.mem.eql(u8, input, "quit")) {
                try self.displayFarewell();
                break;
            }

            if (std.mem.eql(u8, input, "help")) {
                try self.displayHelp();
                continue;
            }

            // Process calculation
            const result = self.calculator.compute(input, .{
                .check_resonance = true,
                .maintain_resonance = true,
            }) catch |err| {
                try self.displayError(err);
                continue;
            };

            // Add to history
            const saved_input = try self.allocator.dupe(u8, input);
            try self.history.append(saved_input);
            if (self.history.items.len > self.config.history_size) {
                const removed = self.history.orderedRemove(0);
                self.allocator.free(removed);
            }

            // Display result
            try self.displayResult(result);
        }
    }

    /// Display calculation result
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

    /// Display error message
    fn displayError(self: *Self, err: anyerror) !void {
        const stdout = std.io.getStdOut().writer();
        try stdout.print("{s}Error:{s} {s}\n", .{
            Color.error_color,  // Updated from Color.error
            Color.reset,
            @errorName(err),
        });
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
};

test "cli_basic" {
    const calculator = try Calculator.init(.{});
    defer calculator.deinit();

    const cli = try CLI.init(.{ .calculator = calculator });
    defer cli.deinit();

    try std.testing.expect(cli.config.show_clarity);
    try std.testing.expect(cli.config.show_harmony);
    try std.testing.expect(cli.config.enable_sparkle);
}

test "cli_history" {
    const calculator = try Calculator.init(.{});
    defer calculator.deinit();

    const cli = try CLI.init(.{ .calculator = calculator });
    defer cli.deinit();

    try cli.history.append(try cli.allocator.dupe(u8, "2 + 2"));
    try std.testing.expectEqual(cli.history.items.len, 1);
}
