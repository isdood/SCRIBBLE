//! Magic Math Bridge Interface
//! Author: @isdood
//! Created: 2025-01-21 13:07:09 UTC

const std = @import("std");
const types = @import("../core/types.zig");
const crystal = @import("../crystal/lattice.zig");

const Result = types.Result;
const CrystalLattice = crystal.CrystalLattice;

/// Magic math operation types
const MagicOp = enum(u8) {
    /// Enhanced addition
    Add = 0,
    /// Enhanced subtraction
    Sub = 1,
    /// Enhanced multiplication
    Mul = 2,
    /// Enhanced division
    Div = 3,
    /// Crystal power operation
    Pow = 4,
    /// Crystal root operation
    Root = 5,
    /// Whimsical modulo
    Mod = 6,
    /// Harmonic blend
    Blend = 7,
};

/// Magic math configuration for FFI
const MagicConfig = extern struct {
    /// Crystal clarity threshold
    clarity_threshold: f64,
    /// Resonance factor
    resonance_factor: f64,
    /// Whimsy enable flag
    enable_whimsy: bool,
    /// Debug mode flag
    debug_mode: bool,
};

/// Result structure for FFI
const MagicResult = extern struct {
    /// Computed value
    value: f64,
    /// Operation success flag
    success: bool,
    /// Error code if any
    error_code: u32,
    /// Crystal clarity achieved
    clarity: f64,
    /// Resonance level
    resonance: f64,
};

/// External Rust functions
extern "rust" fn rust_magic_compute(
    op: u8,
    a: f64,
    b: f64,
    config: *const MagicConfig,
) MagicResult;

extern "rust" fn rust_magic_init() void;
extern "rust" fn rust_magic_cleanup() void;

/// Magic math bridge
pub const MagicMath = struct {
    config: MagicConfig,
    crystal_lattice: *CrystalLattice,
    initialized: bool,

    const Self = @This();

    /// Initialize magic math bridge
    pub fn init(crystal_lattice: *CrystalLattice) !*Self {
        const bridge = try std.heap.page_allocator.create(Self);

        bridge.* = .{
            .config = .{
                .clarity_threshold = 0.87,
                .resonance_factor = 1.0,
                .enable_whimsy = true,
                .debug_mode = false,
            },
            .crystal_lattice = crystal_lattice,
            .initialized = false,
        };

        rust_magic_init();
        bridge.initialized = true;

        return bridge;
    }

    /// Clean up bridge resources
    pub fn deinit(self: *Self) void {
        if (self.initialized) {
            rust_magic_cleanup();
        }
        std.heap.page_allocator.destroy(self);
    }

    /// Perform magic computation
    pub fn compute(self: *Self, op: MagicOp, a: f64, b: f64) !Result {
        // Update config with current crystal state
        self.updateConfig();

        // Perform computation through Rust
        const magic_result = rust_magic_compute(
            @enumToInt(op),
                                                a,
                                                b,
                                                &self.config
        );

        // Handle computation result
        if (!magic_result.success) {
            return error.MagicComputationFailed;
        }

        // Create result with crystal metrics
        return Result{
            .value = magic_result.value,
            .clarity = magic_result.clarity,
            .resonance = magic_result.resonance,
            .sparkle = magic_result.clarity >= 0.95,
            .timestamp = std.time.timestamp(),
            .whimsy = if (self.config.enable_whimsy) 1.1 else 1.0,
        };
    }

    /// Update configuration with crystal state
    fn updateConfig(self: *Self) void {
        const crystal_metrics = self.crystal_lattice.getMetrics();

        self.config.clarity_threshold = @max(0.85, crystal_metrics.clarity);
        self.config.resonance_factor = crystal_metrics.resonance_factor;
    }

    /// Enhanced addition
    pub fn add(self: *Self, a: f64, b: f64) !Result {
        return self.compute(.Add, a, b);
    }

    /// Enhanced subtraction
    pub fn sub(self: *Self, a: f64, b: f64) !Result {
        return self.compute(.Sub, a, b);
    }

    /// Enhanced multiplication
    pub fn mul(self: *Self, a: f64, b: f64) !Result {
        return self.compute(.Mul, a, b);
    }

    /// Enhanced division
    pub fn div(self: *Self, a: f64, b: f64) !Result {
        if (b == 0.0) return error.DivisionByZero;
        return self.compute(.Div, a, b);
    }

    /// Crystal power operation
    pub fn pow(self: *Self, base: f64, exp: f64) !Result {
        return self.compute(.Pow, base, exp);
    }

    /// Crystal root operation
    pub fn root(self: *Self, value: f64, n: f64) !Result {
        if (value < 0.0 and @mod(n, 2.0) == 0.0) {
            return error.InvalidRoot;
        }
        return self.compute(.Root, value, n);
    }

    /// Whimsical modulo
    pub fn mod(self: *Self, a: f64, b: f64) !Result {
        if (b == 0.0) return error.ModuloByZero;
        return self.compute(.Mod, a, b);
    }

    /// Harmonic blend
    pub fn blend(self: *Self, a: f64, b: f64) !Result {
        return self.compute(.Blend, a, b);
    }
};

test "magic_math_basic" {
    var lattice = try CrystalLattice.init(null);
    defer lattice.deinit();

    var magic = try MagicMath.init(lattice);
    defer magic.deinit();

    const result = try magic.add(2.0, 2.0);
    try std.testing.expectEqual(result.value, 4.0);
    try std.testing.expect(result.clarity > 0.0);
}

test "magic_math_division" {
    var lattice = try CrystalLattice.init(null);
    defer lattice.deinit();

    var magic = try MagicMath.init(lattice);
    defer magic.deinit();

    const result = try magic.div(10.0, 2.0);
    try std.testing.expectEqual(result.value, 5.0);

    try std.testing.expectError(error.DivisionByZero, magic.div(1.0, 0.0));
}

test "magic_math_whimsy" {
    var lattice = try CrystalLattice.init(.{ .clarity = 1.0 });
    defer lattice.deinit();

    var magic = try MagicMath.init(lattice);
    defer magic.deinit();

    const result = try magic.blend(3.0, 7.0);
    try std.testing.expect(result.whimsy > 1.0);
}
