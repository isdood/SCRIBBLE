//! Facet - Crystal-Based Calculator
//! Author: @isdood
//! Created: 2025-01-21 15:55:16 UTC

const std = @import("std");
const core = @import("core/calculator.zig");
const resonance = @import("resonance/attunement.zig");
const crystal = @import("crystal/lattice.zig");
const ui = @import("ui/cli.zig");

const Calculator = core.Calculator;
const Attunement = resonance.Attunement;  // Updated from ResonanceState
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

/// Global state type definition
const GlobalState = struct {
    calculator: ?Calculator = null,
    resonance_state: ?Attunement = null,  // Updated from ResonanceState
    crystal_lattice: ?CrystalLattice = null,
    cli: ?CLI = null,

    /// Initialize all components
    pub fn init(self: *GlobalState) MainError!void {
        if (self.calculator != null) return;

        // Initialize crystal lattice
        self.crystal_lattice = try CrystalLattice.init(.{
            .clarity = Config.crystal_clarity,
            .facets = 3,
            .sparkle_factor = 0.7,
        });

        // Initialize resonance state with crystal lattice
        self.resonance_state = try Attunement.init(self.crystal_lattice.?, .{
            .min_resonance = Config.resonance_threshold,
            .attunement_factor = Config.attunement_strength,
            .adaptive_resonance = true,
        });

        // Initialize calculator with resonance state and crystal lattice
        self.calculator = try Calculator.init(.{
            .resonance_state = self.resonance_state.?,
            .crystal_lattice = self.crystal_lattice.?,
        });

        // Initialize CLI
        self.cli = try CLI.init(.{
            .calculator = self.calculator.?,
        });
    }

    /// Deinitialize all components
    pub fn deinit(self: *GlobalState) void {
        if (self.cli) |cli| cli.deinit();
        if (self.calculator) |calc| calc.deinit();
        if (self.resonance_state) |res| res.deinit();
        if (self.crystal_lattice) |lattice| lattice.deinit();
    }
};

/// Global state instance
var global_state = GlobalState{};

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
            .maintain_resonance = true,  // Updated from maintain_attunement
        });

        try global_state.cli.?.displayResult(result);
    } else {
        // Interactive mode
        try global_state.cli.?.runInteractive();
    }
}

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
