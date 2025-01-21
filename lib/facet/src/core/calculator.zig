//! Facet Core Calculator
//! Author: @isdood
//! Created: 2025-01-21 15:42:04 UTC

const std = @import("std");
const crystal = @import("../crystal/lattice.zig");
const resonance = @import("../resonance/attunement.zig");
const types = @import("types.zig");

const CrystalLattice = crystal.CrystalLattice;
const Attunement = resonance.Attunement;  // Updated to match new name
const Result = types.Result;

/// Calculator configuration options
pub const CalculatorConfig = struct {
    /// Crystal lattice instance
    crystal_lattice: *CrystalLattice,
    /// Resonance state instance
    resonance_state: *Attunement,  // Updated to match new name
    /// Enable resonance checking
    check_resonance: bool = true,
    /// Enable result caching
    enable_cache: bool = true,
    /// Maximum cache size
    max_cache_size: usize = 1000,
};

// ... (Operation enum remains unchanged) ...

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

    // ... (checkCache and cacheResult methods remain unchanged) ...

    /// Evaluate mathematical expression
    fn evaluate(self: *Self, expression: []const u8) !Result {
        _ = self;  // Mark self as unused
        var parser = ExpressionParser.init(expression);
        const ast = try parser.parse();
        return ast;  // Return the Result directly since parse() now returns Result
    }

    /// Apply crystal resonance to result
    fn applyResonance(self: *Self, result: *Result) !void {
        // Update crystal lattice state
        try self.config.crystal_lattice.attune(result.resonance);

        // Apply resonance effects
        try self.config.resonance_state.optimize(result);  // Changed from apply to optimize to match new API

        // Update result clarity based on crystal state
        result.clarity = self.config.crystal_lattice.clarity;
    }

    /// Maintain stable resonance state
    fn maintainResonance(self: *Self, _: *Result) !void {  // Mark result as unused with _
        const metrics = self.config.resonance_state.getMetrics();

        // Adjust resonance if needed
        if (metrics.resonance < 0.85) {
            try self.config.crystal_lattice.applyDispersion();
            // Note: stabilize method is now part of the optimize workflow
            try self.config.resonance_state.optimize(&Result{
                .value = 0.0,
                .resonance = metrics.resonance,
                .clarity = self.config.crystal_lattice.clarity,
            });
        }
    }

    /// Get calculator metrics
    pub fn getMetrics(self: *const Self) struct {
        cache_size: usize,
        crystal_clarity: f64,
        resonance_level: f64,
    } {
        const resonance_metrics = self.config.resonance_state.getMetrics();
        return .{
            .cache_size = self.cache.items.len,
            .crystal_clarity = self.config.crystal_lattice.clarity,
            .resonance_level = resonance_metrics.resonance,  // Updated to use new metrics struct
        };
    }
};

// ... (ExpressionParser struct remains unchanged) ...

test "calculator_basic" {
    var lattice = try CrystalLattice.init(.{
        .clarity = 0.95,
        .facets = 3,
    });
    defer lattice.deinit();

    var resonance_state = try Attunement.init(lattice, null);  // Updated to match new API
    defer resonance_state.deinit();

    var calculator = try Calculator.init(.{
        .crystal_lattice = lattice,
        .resonance_state = resonance_state,
    });
    defer calculator.deinit();

    const result = try calculator.compute("2 + 2", .{});
    try std.testing.expectEqual(result.value, 4.0);
}

test "calculator_cache" {
    var lattice = try CrystalLattice.init(null);
    defer lattice.deinit();

    var resonance_state = try Attunement.init(lattice, null);  // Updated to match new API
    defer resonance_state.deinit();

    var calculator = try Calculator.init(.{
        .crystal_lattice = lattice,
        .resonance_state = resonance_state,
        .enable_cache = true,
    });
    defer calculator.deinit();

    _ = try calculator.compute("1 + 1", .{});
    const metrics = calculator.getMetrics();

    try std.testing.expect(metrics.cache_size > 0);
}
