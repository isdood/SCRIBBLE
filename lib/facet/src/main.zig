//! Facet - Crystal-Based Calculator
//! Author: @isdood
//! Created: 2025-01-21 12:53:30 UTC

const std = @import("std");
const core = @import("core/calculator.zig");
const resonance = @import("resonance/attunement.zig");
const crystal = @import("crystal/lattice.zig");
const ui = @import("ui/cli.zig");

const Calculator = core.Calculator;
const ResonanceState = resonance.ResonanceState;
const CrystalLattice = crystal.CrystalLattice;
const CLI = ui.CLI;

/// Default configuration values
const Config = struct {
    /// Minimum resonance threshold
    resonance_threshold: f64 = 0.87,
    /// Crystal attunement strength
    attunement_strength: f64 = 0.93,
    /// Crystal clarity requirement
    crystal_clarity: f64 = 0.95,
};

/// Error set for main operations
const MainError = error{
    ResonanceLoss,
    AttunementFailure,
    CrystalMisalignment,
    InvalidInput,
    InitializationFailed,
    WhimsyDepletion,
};

/// Global state management
var global_state = struct {
    calculator: ?Calculator = null,
    resonance_state: ?ResonanceState = null,
    crystal_lattice: ?CrystalLattice = null,
    cli: ?CLI = null,

    /// Initialize all components
    pub fn init() MainError!void {
        if (global_state.calculator != null) return;

        // Initialize crystal lattice
        global_state.crystal_lattice = try CrystalLattice.init(.{
            .clarity = Config.crystal_clarity,
            .facets = 3,
            .sparkle_factor = 0.7,
        });

        // Initialize resonance state
        global_state.resonance_state = try ResonanceState.init(.{
            .resonance = Config.resonance_threshold,
            .attunement = Config.attunement_strength,
            .whimsy = 1.0,
        });

        // Initialize calculator with resonance state and crystal lattice
        global_state.calculator = try Calculator.init(.{
            .resonance_state = global_state.resonance_state.?,
            .crystal_lattice = global_state.crystal_lattice.?,
        });

        // Initialize CLI
        global_state.cli = try CLI.init(.{
            .calculator = global_state.calculator.?,
        });
    }

    /// Deinitialize all components
    pub fn deinit() void {
        if (global_state.cli) |cli| cli.deinit();
        if (global_state.calculator) |calc| calc.deinit();
        if (global_state.resonance_state) |res| res.deinit();
        if (global_state.crystal_lattice) |lattice| lattice.deinit();
    }
};

/// Main entry point
pub fn main() !void {
    // Initialize logger
    try std.log.init(.{
        .level = .info,
        .prefix = "[Facet] ",
    });
    defer std.log.deinit();

    const log = std.log.scoped(.main);
    log.info("Initializing Facet Calculator...", .{});

    // Initialize global state
    try global_state.init();
    defer global_state.deinit();

    // Process command line arguments
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    // Handle CLI operations
    if (args.len > 1) {
        // Expression provided as argument
        const result = try global_state.calculator.?.compute(args[1], .{
            .check_resonance = true,
            .maintain_attunement = true,
        });

        try global_state.cli.?.displayResult(result);
    } else {
        // Interactive mode
        try global_state.cli.?.runInteractive();
    }
}

// Test main functionality
test "basic_initialization" {
    try global_state.init();
    defer global_state.deinit();

    try std.testing.expect(global_state.calculator != null);
    try std.testing.expect(global_state.resonance_state != null);
    try std.testing.expect(global_state.crystal_lattice != null);
    try std.testing.expect(global_state.cli != null);
}

test "crystal_resonance" {
    try global_state.init();
    defer global_state.deinit();

    const result = try global_state.calculator.?.compute("2 + 2", .{
        .check_resonance = true,
    });

    try std.testing.expectEqual(result.value, 4.0);
    try std.testing.expect(result.resonance >= Config.resonance_threshold);
    try std.testing.expect(result.clarity >= Config.crystal_clarity);
}
