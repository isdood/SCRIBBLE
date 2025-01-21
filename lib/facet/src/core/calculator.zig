//! Facet Core Calculator
//! Author: @isdood
//! Created: 2025-01-21 13:01:19 UTC

const std = @import("std");
const crystal = @import("../crystal/lattice.zig");
const resonance = @import("../resonance/attunement.zig");
const types = @import("types.zig");

const CrystalLattice = crystal.CrystalLattice;
const ResonanceState = resonance.ResonanceState;
const Result = types.Result;

/// Calculator configuration options
pub const CalculatorConfig = struct {
    /// Crystal lattice instance
    crystal_lattice: *CrystalLattice,
    /// Resonance state instance
    resonance_state: *ResonanceState,
    /// Enable resonance checking
    check_resonance: bool = true,
    /// Enable result caching
    enable_cache: bool = true,
    /// Maximum cache size
    max_cache_size: usize = 1000,
};

/// Operation type
const Operation = enum {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Root,

    /// Get operation symbol
    pub fn symbol(self: Operation) []const u8 {
        return switch (self) {
            .Add => "+",
            .Subtract => "-",
            .Multiply => "*",
            .Divide => "/",
            .Power => "^",
            .Root => "√",
        };
    }
};

/// Calculator result cache entry
const CacheEntry = struct {
    expression: []const u8,
    result: Result,
    timestamp: i64,
};

/// Core calculator implementation
pub const Calculator = struct {
    config: CalculatorConfig,
    cache: std.ArrayList(CacheEntry),
    allocator: std.mem.Allocator,

    const Self = @This();

    /// Initialize new calculator
    pub fn init(config: CalculatorConfig) !*Self {
        const calculator = try std.heap.page_allocator.create(Self);

        calculator.* = .{
            .config = config,
            .cache = std.ArrayList(CacheEntry).init(std.heap.page_allocator),
            .allocator = std.heap.page_allocator,
        };

        return calculator;
    }

    /// Clean up calculator resources
    pub fn deinit(self: *Self) void {
        for (self.cache.items) |entry| {
            self.allocator.free(entry.expression);
        }
        self.cache.deinit();
        std.heap.page_allocator.destroy(self);
    }

    /// Compute result for expression
    pub fn compute(self: *Self, expression: []const u8, options: struct {
        check_resonance: bool = true,
        maintain_resonance: bool = true,
    }) !Result {
        // Check cache first if enabled
        if (self.config.enable_cache) {
            if (self.checkCache(expression)) |cached| {
                return cached;
            }
        }

        // Parse and evaluate expression
        var result = try self.evaluate(expression);

        // Apply crystal resonance if enabled
        if (options.check_resonance) {
            try self.applyResonance(&result);
        }

        // Maintain resonance state if requested
        if (options.maintain_resonance) {
            try self.maintainResonance(&result);
        }

        // Cache result if enabled
        if (self.config.enable_cache) {
            try self.cacheResult(expression, result);
        }

        return result;
    }

    /// Check cache for existing result
    fn checkCache(self: *Self, expression: []const u8) ?Result {
        for (self.cache.items) |entry| {
            if (std.mem.eql(u8, entry.expression, expression)) {
                return entry.result;
            }
        }
        return null;
    }

    /// Cache calculation result
    fn cacheResult(self: *Self, expression: []const u8, result: Result) !void {
        // Remove oldest entry if cache is full
        if (self.cache.items.len >= self.config.max_cache_size) {
            _ = self.cache.orderedRemove(0);
        }

        // Create new cache entry
        const cached_expr = try self.allocator.dupe(u8, expression);
        const entry = CacheEntry{
            .expression = cached_expr,
            .result = result,
            .timestamp = std.time.timestamp(),
        };

        try self.cache.append(entry);
    }

    /// Evaluate mathematical expression
    fn evaluate(self: *Self, expression: []const u8) !Result {
        var parser = ExpressionParser.init(expression);
        const ast = try parser.parse();
        return self.evaluateAst(ast);
    }

    /// Apply crystal resonance to result
    fn applyResonance(self: *Self, result: *Result) !void {
        // Update crystal lattice state
        try self.config.crystal_lattice.attune(result.resonance);

        // Apply resonance effects
        try self.config.resonance_state.apply(result);

        // Update result clarity based on crystal state
        result.clarity = self.config.crystal_lattice.clarity;
    }

    /// Maintain stable resonance state
    fn maintainResonance(self: *Self, result: *Result) !void {
        const metrics = self.config.resonance_state.getMetrics();

        // Adjust resonance if needed
        if (metrics.resonance < 0.85) {
            try self.config.crystal_lattice.applyDispersion();
            try self.config.resonance_state.stabilize();
        }
    }

    /// Get calculator metrics
    pub fn getMetrics(self: *const Self) struct {
        cache_size: usize,
        crystal_clarity: f64,
        resonance_level: f64,
    } {
        return .{
            .cache_size = self.cache.items.len,
            .crystal_clarity = self.config.crystal_lattice.clarity,
            .resonance_level = self.config.resonance_state.getCurrentLevel(),
        };
    }
};

/// Basic expression parser
const ExpressionParser = struct {
    input: []const u8,
    pos: usize,

    const ParseError = error{
        InvalidExpression,
        UnexpectedToken,
        UnbalancedParentheses,
    };

    fn init(input: []const u8) ExpressionParser {
        return .{
            .input = input,
            .pos = 0,
        };
    }

    fn parse(self: *ExpressionParser) !Result {
        // Basic expression parsing implementation
        // This would be expanded with proper operator precedence and AST generation
        _ = self;
        return Result{
            .value = 0.0,
            .resonance = 1.0,
            .clarity = 1.0,
        };
    }
};

test "calculator_basic" {
    var lattice = try CrystalLattice.init(.{
        .clarity = 0.95,
        .facets = 3,
    });
    defer lattice.deinit();

    var res_state = try ResonanceState.init(.{});
    defer res_state.deinit();

    var calculator = try Calculator.init(.{
        .crystal_lattice = lattice,
        .resonance_state = res_state,
    });
    defer calculator.deinit();

    const result = try calculator.compute("2 + 2", .{});
    try std.testing.expectEqual(result.value, 4.0);
}

test "calculator_cache" {
    var lattice = try CrystalLattice.init(null);
    defer lattice.deinit();

    var res_state = try ResonanceState.init(null);
    defer res_state.deinit();

    var calculator = try Calculator.init(.{
        .crystal_lattice = lattice,
        .resonance_state = res_state,
        .enable_cache = true,
    });
    defer calculator.deinit();

    _ = try calculator.compute("1 + 1", .{});
    const metrics = calculator.getMetrics();

    try std.testing.expect(metrics.cache_size > 0);
}
