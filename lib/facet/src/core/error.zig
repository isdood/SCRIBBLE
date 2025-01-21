//! Facet Error Handling
//! Author: @isdood
//! Created: 2025-01-21 13:02:19 UTC

const std = @import("std");

/// Core error categories
pub const ErrorCategory = enum {
    /// Crystal-related errors
    Crystal,
    /// Resonance-related errors
    Resonance,
    /// Calculation-related errors
    Calculation,
    /// System-related errors
    System,
};

/// Crystal system errors
pub const CrystalError = error{
    /// Crystal clarity too low
    LowClarity,
    /// Crystal lattice misalignment
    LatticeMisalignment,
    /// Facet symmetry violation
    SymmetryViolation,
    /// Dispersion instability
    DispersionInstability,
    /// Crystal overload
    CrystalOverload,
    /// Crystal fatigue
    CrystalFatigue,
};

/// Resonance system errors
pub const ResonanceError = error{
    /// Resonance loss
    ResonanceLoss,
    /// Attunement failure
    AttunementFailure,
    /// Harmonic disruption
    HarmonicDisruption,
    /// Phase misalignment
    PhaseMisalignment,
    /// Pattern collapse
    PatternCollapse,
    /// Whimsy depletion
    WhimsyDepletion,
};

/// Calculation system errors
pub const CalculationError = error{
    /// Invalid expression
    InvalidExpression,
    /// Division by zero
    DivisionByZero,
    /// Overflow
    Overflow,
    /// Underflow
    Underflow,
    /// Invalid operation
    InvalidOperation,
    /// Precision loss
    PrecisionLoss,
};

/// System-level errors
pub const SystemError = error{
    /// Resource exhaustion
    ResourceExhaustion,
    /// Cache overflow
    CacheOverflow,
    /// Memory allocation failure
    AllocationFailure,
    /// State corruption
    StateCorruption,
    /// Configuration error
    ConfigurationError,
};

/// Combined error set
pub const FacetError = CrystalError || ResonanceError || CalculationError || SystemError;

/// Error context information
pub const ErrorContext = struct {
    /// Error category
    category: ErrorCategory,
    /// Error source location
    source: std.builtin.SourceLocation,
    /// Error timestamp
    timestamp: i64,
    /// Crystal clarity at error time
    clarity: f64,
    /// Resonance level at error time
    resonance: f64,
    /// Additional error details
    details: ?[]const u8,
};

/// Error handler for Facet system
pub const ErrorHandler = struct {
    /// Last error context
    last_error: ?ErrorContext,
    /// Error count by category
    error_counts: std.AutoHashMap(ErrorCategory, usize),
    allocator: std.mem.Allocator,

    const Self = @This();

    /// Initialize new error handler
    pub fn init(allocator: std.mem.Allocator) !*Self {
        const handler = try allocator.create(Self);

        handler.* = .{
            .last_error = null,
            .error_counts = std.AutoHashMap(ErrorCategory, usize).init(allocator),
            .allocator = allocator,
        };

        return handler;
    }

    /// Clean up error handler resources
    pub fn deinit(self: *Self) void {
        if (self.last_error) |err| {
            if (err.details) |details| {
                self.allocator.free(details);
            }
        }
        self.error_counts.deinit();
        self.allocator.destroy(self);
    }

    /// Handle error with context
    pub fn handleError(self: *Self, err: FacetError, context: ErrorContext) void {
        // Update error counts
        const count = self.error_counts.get(context.category) orelse 0;
        self.error_counts.put(context.category, count + 1) catch {};

        // Store error context
        if (self.last_error) |last| {
            if (last.details) |details| {
                self.allocator.free(details);
            }
        }
        self.last_error = context;

        // Log error
        std.log.err("{s} Error: {s} (Clarity: {d:.2}, Resonance: {d:.2})", .{
            @tagName(context.category),
                    @errorName(err),
                    context.clarity,
                    context.resonance,
        });
    }

    /// Check if error is recoverable
    pub fn isRecoverable(err: FacetError) bool {
        return switch (err) {
            CrystalError.LowClarity,
            CrystalError.CrystalFatigue,
            ResonanceError.ResonanceLoss,
            ResonanceError.WhimsyDepletion,
            CalculationError.PrecisionLoss,
            => true,
            else => false,
        };
    }

    /// Get error suggestion
    pub fn getSuggestion(err: FacetError) []const u8 {
        return switch (err) {
            CrystalError.LowClarity => "Try increasing crystal attunement",
            CrystalError.LatticeMisalignment => "Realign crystal lattice structure",
            ResonanceError.ResonanceLoss => "Stabilize resonance pattern",
            ResonanceError.WhimsyDepletion => "Restore whimsy through meditation",
            CalculationError.PrecisionLoss => "Consider using higher precision mode",
            else => "Consult crystal maintenance guide",
        };
    }

    /// Get error metrics
    pub fn getMetrics(self: *const Self) struct {
        total_errors: usize,
        errors_by_category: std.AutoHashMap(ErrorCategory, usize),
        last_error_timestamp: ?i64,
    } {
        var total: usize = 0;
        var iterator = self.error_counts.iterator();
        while (iterator.next()) |entry| {
            total += entry.value_ptr.*;
        }

        return .{
            .total_errors = total,
            .errors_by_category = self.error_counts,
            .last_error_timestamp = if (self.last_error) |err| err.timestamp else null,
        };
    }
};

test "error_handler_basic" {
    const allocator = std.testing.allocator;

    var handler = try ErrorHandler.init(allocator);
    defer handler.deinit();

    const context = ErrorContext{
        .category = .Crystal,
        .source = @src(),
        .timestamp = std.time.timestamp(),
        .clarity = 0.85,
        .resonance = 0.90,
        .details = null,
    };

    handler.handleError(CrystalError.LowClarity, context);

    const metrics = handler.getMetrics();
    try std.testing.expect(metrics.total_errors == 1);
}

test "error_recovery" {
    try std.testing.expect(ErrorHandler.isRecoverable(CrystalError.LowClarity));
    try std.testing.expect(!ErrorHandler.isRecoverable(CrystalError.LatticeMisalignment));
}
