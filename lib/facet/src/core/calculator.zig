//! Facet Core Calculator
//! Author: @isdood
//! Created: 2025-01-21 16:11:06 UTC

const std = @import("std");
const types = @import("types.zig");
const resonance_mod = @import("../resonance/attunement.zig");
const crystal_mod = @import("../crystal/lattice.zig");

const Result = types.Result;
const CalcError = types.CalcError;
const Attunement = resonance_mod.Attunement;
const CrystalLattice = crystal_mod.CrystalLattice;

/// Calculator configuration
pub const CalcConfig = struct {
    /// Enable resonance checking
    check_resonance: bool = true,
    /// Maintain resonance during calculations
    maintain_resonance: bool = true,
    /// Minimum required clarity
    min_clarity: f64 = 0.8,
    /// Enable error correction
    error_correction: bool = true,
};

/// Core calculator implementation
pub const Calculator = struct {
    resonance_state: *Attunement,
    crystal_lattice: *CrystalLattice,
    config: CalcConfig,

    const Self = @This();

    /// Initialize new calculator
    pub fn init(options: struct {
        resonance_state: *Attunement,
        crystal_lattice: *CrystalLattice,
        config: ?CalcConfig = null,
    }) !*Self {
        const calculator = try std.heap.page_allocator.create(Self);

        calculator.* = .{
            .resonance_state = options.resonance_state,
            .crystal_lattice = options.crystal_lattice,
            .config = options.config orelse CalcConfig{},
        };

        return calculator;
    }

    /// Clean up calculator resources
    pub fn deinit(self: *Self) void {
        std.heap.page_allocator.destroy(self);
    }

    /// Compute result from expression
    pub fn compute(self: *Self, expression: []const u8, options: struct {
        check_resonance: bool = true,
        maintain_resonance: bool = true,
    }) !Result {
        // Parse and evaluate expression
        var result = try self.evaluate(expression);

        // Apply resonance checks if enabled
        if (options.check_resonance and self.config.check_resonance) {
            try self.resonance_state.optimize(&result);

            if (result.resonance < 0.5) {
                return CalcError.ResonanceLoss;
            }
        }

        // Maintain resonance if enabled
        if (options.maintain_resonance and self.config.maintain_resonance) {
            try self.crystal_lattice.attune(result.resonance);
        }

        // Check clarity requirements
        if (result.clarity < self.config.min_clarity) {
            return CalcError.ClarityTooLow;
        }

        return result;
    }

    /// Evaluate mathematical expression
    fn evaluate(self: *Self, expression: []const u8) !Result {
        _ = self;  // Autofix: self is unused
        var parser = ExpressionParser.init(expression);
        const value = try parser.parse();
        return Result.init(value);
    }

    /// Expression parser helper
    const ExpressionParser = struct {
        input: []const u8,
        index: usize,

        fn init(input: []const u8) ExpressionParser {
            return .{
                .input = input,
                .index = 0,
            };
        }

        fn parse(self: *ExpressionParser) CalcError!f64 {
            var result = try self.parseTerm();

            while (self.index < self.input.len) {
                const op = self.input[self.index];
                if (op != '+' and op != '-') break;
                self.index += 1;

                const term = try self.parseTerm();
                result = if (op == '+') result + term else result - term;
            }

            return result;
        }

        fn parseTerm(self: *ExpressionParser) CalcError!f64 {
            var result = try self.parseFactor();

            while (self.index < self.input.len) {
                const op = self.input[self.index];
                if (op != '*' and op != '/') break;
                self.index += 1;

                const factor = try self.parseFactor();
                if (op == '*') {
                    result *= factor;
                } else {
                    if (factor == 0.0) return CalcError.DivisionByZero;
                    result /= factor;
                }
            }

            return result;
        }

        fn parseFactor(self: *ExpressionParser) CalcError!f64 {
            // Skip whitespace
            while (self.index < self.input.len and self.input[self.index] == ' ') {
                self.index += 1;
            }

            if (self.index >= self.input.len) return CalcError.InvalidOperation;

            // Parse parentheses
            if (self.input[self.index] == '(') {
                self.index += 1;
                const result = try self.parse();
                if (self.index >= self.input.len or self.input[self.index] != ')') {
                    return CalcError.InvalidOperation;
                }
                self.index += 1;
                return result;
            }

            // Parse number
            const start = self.index;
            while (self.index < self.input.len) {
                const c = self.input[self.index];
                if (!std.ascii.isDigit(c) and c != '.') break;
                self.index += 1;
            }

            if (start == self.index) return CalcError.InvalidOperation;

            const number = std.fmt.parseFloat(f64, self.input[start..self.index]) catch {
                return CalcError.InvalidOperation;
            };
            return number;
        }
    };
};

test "calculator_basic" {
    var test_crystal = try CrystalLattice.init(.{
        .clarity = 0.95,
        .facets = 3,
    });
    defer test_crystal.deinit();

    var test_resonance = try Attunement.init(test_crystal, null);
    defer test_resonance.deinit();

    var calculator = try Calculator.init(.{
        .resonance_state = test_resonance,
        .crystal_lattice = test_crystal,
    });
    defer calculator.deinit();

    const result = try calculator.compute("2 + 2", .{});
    try std.testing.expectEqual(result.value, 4.0);
}

test "calculator_complex" {
    var test_crystal = try CrystalLattice.init(.{
        .clarity = 1.0,
        .facets = 4,
    });
    defer test_crystal.deinit();

    var test_resonance = try Attunement.init(test_crystal, .{
        .min_resonance = 0.9,
        .target_resonance = 0.95,
    });
    defer test_resonance.deinit();

    var calculator = try Calculator.init(.{
        .resonance_state = test_resonance,
        .crystal_lattice = test_crystal,
        .config = .{
            .min_clarity = 0.9,
        },
    });
    defer calculator.deinit();

    const result = try calculator.compute("(3 + 4) * 2", .{
        .check_resonance = true,
        .maintain_resonance = true,
    });

    try std.testing.expectEqual(result.value, 14.0);
    try std.testing.expect(result.clarity >= calculator.config.min_clarity);
}
