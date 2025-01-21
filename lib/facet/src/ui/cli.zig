//! Facet CLI Interface
//! Author: @isdood
//! Created: 2025-01-21 16:19:43 UTC

const std = @import("std");
const calc = @import("../core/calculator.zig");
const types = @import("../core/types.zig");

const Calculator = calc.Calculator;
const Result = types.Result;
const CalcError = types.CalcError;

/// CLI configuration
pub const CLIConfig = struct {
    /// Show calculation clarity
    show_clarity: bool = true,
    /// Show resonance values
    show_resonance: bool = true,
    /// Enable command history
    enable_history: bool = true,
    /// Maximum history size
    max_history: usize = 100,
};

/// CLI interface
pub const CLI = struct {
    calculator: *Calculator,
    config: CLIConfig,
    allocator: std.mem.Allocator,
    history: std.ArrayList([]const u8),
    stdin: std.fs.File.Reader,
    stdout: std.fs.File.Writer,

    const Self = @This();

    /// Initialize new CLI interface
    pub fn init(options: struct {
        calculator: *Calculator,
        config: ?CLIConfig = null,
    }) !*Self {
        const cli = try std.heap.page_allocator.create(Self);

        cli.* = .{
            .calculator = options.calculator,
            .config = options.config orelse CLIConfig{},
            .allocator = std.heap.page_allocator,
            .history = std.ArrayList([]const u8).init(std.heap.page_allocator),
            .stdin = std.io.getStdIn().reader(),
            .stdout = std.io.getStdOut().writer(),
        };

        return cli;
    }

    /// Clean up CLI resources
    pub fn deinit(self: *Self) void {
        for (self.history.items) |item| {
            self.allocator.free(item);
        }
        self.history.deinit();
        std.heap.page_allocator.destroy(self);
    }

    /// Run CLI interface
    pub fn run(self: *Self) !void {
        var buffer: [1024]u8 = undefined;

        while (true) {
            // Print prompt
            try self.stdout.writeAll("> ");

            // Read input
            const input = (try self.stdin.readUntilDelimiterOrEof(buffer[0..], '\n')) orelse break;
            const trimmed = std.mem.trim(u8, input, " \t\r\n");

            if (trimmed.len == 0) continue;

            // Handle commands
            if (std.mem.eql(u8, trimmed, "exit")) break;
            if (std.mem.eql(u8, trimmed, "help")) {
                try self.printHelp();
                continue;
            }
            if (std.mem.eql(u8, trimmed, "history")) {
                try self.printHistory();
                continue;
            }
            if (std.mem.eql(u8, trimmed, "clear")) {
                try self.clearHistory();
                continue;
            }

            // Add to history
            if (self.config.enable_history) {
                try self.addToHistory(trimmed);
            }

            // Calculate result
            self.calculate(trimmed) catch |err| {
                try self.printError(err);
                continue;
            };
        }
    }

    /// Add expression to history
    fn addToHistory(self: *Self, expression: []const u8) !void {
        if (self.history.items.len >= self.config.max_history) {
            const removed = self.history.orderedRemove(0);
            self.allocator.free(removed);
        }

        const duped = try self.allocator.dupe(u8, expression);
        try self.history.append(duped);
    }

    /// Print command history
    fn printHistory(self: *Self) !void {
        for (self.history.items, 0..) |item, i| {
            try self.stdout.print("{d}: {s}\n", .{ i + 1, item });
        }
    }

    /// Clear command history
    fn clearHistory(self: *Self) !void {
        for (self.history.items) |item| {
            self.allocator.free(item);
        }
        try self.history.resize(0);
    }

    /// Print help message
    fn printHelp(self: *Self) !void {
        try self.stdout.writeAll(
            \\Commands:
            \\  help    - Show this help message
            \\  history - Show calculation history
            \\  clear   - Clear calculation history
            \\  exit    - Exit calculator
            \\
            \\Enter mathematical expressions to calculate
            \\Example: 2 + 2 * (3 + 4)
        \\
        );
    }

    /// Calculate and print result
    fn calculate(self: *Self, expression: []const u8) !void {
        const result = try self.calculator.compute(expression, .{
            .check_resonance = true,
            .maintain_resonance = true,
        });

        if (self.config.show_clarity or self.config.show_resonance) {
            try self.stdout.print("= {}\n", .{result});
        } else {
            try self.stdout.print("= {d:.4}\n", .{result.value});
        }
    }

    /// Print error message
    fn printError(self: *Self, err: anyerror) !void {
        const msg = switch (err) {
            CalcError.DivisionByZero => "Error: Division by zero",
            CalcError.InvalidOperation => "Error: Invalid operation",
            CalcError.ResonanceLoss => "Error: Resonance loss exceeded threshold",
            CalcError.ClarityTooLow => "Error: Crystal clarity too low",
            CalcError.Overflow => "Error: Numeric overflow",
            CalcError.Underflow => "Error: Numeric underflow",
            else => "Error: Unknown error occurred",
        };
            try self.stdout.print("{s}\n", .{msg});
    }
};

test "cli_basic" {
    var test_crystal = try crystal.CrystalLattice.init(.{
        .clarity = 0.95,
        .facets = 3,
    });
    defer test_crystal.deinit();

    var test_resonance = try resonance.Attunement.init(test_crystal, null);
    defer test_resonance.deinit();

    var calculator = try Calculator.init(.{
        .resonance_state = test_resonance,
        .crystal_lattice = test_crystal,
    });
    defer calculator.deinit();

    const cli = try CLI.init(.{ .calculator = calculator });
    defer cli.deinit();

    try std.testing.expect(cli.config.show_clarity);
}

test "cli_history" {
    var test_crystal = try crystal.CrystalLattice.init(.{
        .clarity = 0.95,
        .facets = 3,
    });
    defer test_crystal.deinit();

    var test_resonance = try resonance.Attunement.init(test_crystal, null);
    defer test_resonance.deinit();

    var calculator = try Calculator.init(.{
        .resonance_state = test_resonance,
        .crystal_lattice = test_crystal,
    });
    defer calculator.deinit();

    const cli = try CLI.init(.{ .calculator = calculator });
    defer cli.deinit();

    try cli.history.append(try cli.allocator.dupe(u8, "2 + 2"));
    try std.testing.expectEqual(cli.history.items.len, 1);
}
