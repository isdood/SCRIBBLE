//! Facet - Crystal-Based Calculator
//! Author: @isdood
//! Created: 2025-01-21 16:23:15 UTC

const std = @import("std");
const ui = @import("ui/mod.zig");
const resonance = @import("resonance/attunement.zig");
const crystal = @import("crystal/lattice.zig");
const core_calc = @import("core/calculator.zig");  // Renamed from calc to core_calc
const types = @import("core/types.zig");

const CLI = ui.cli.CLI;
const Result = types.Result;
const Calculator = core_calc.Calculator;  // Updated reference
const Attunement = resonance.Attunement;
const CrystalLattice = crystal.CrystalLattice;

/// Default configuration values
const Config = struct {
    /// Minimum resonance threshold
    resonance_threshold: f64 = 0.87,
    /// Crystal attunement strength
    attunement_strength: f64 = 0.93,
    /// Crystal clarity
    clarity: f64 = 0.95,
};

/// Application entry point
pub fn main() !void {
    // Initialize standard output
    const stdout = std.io.getStdOut().writer();

    // Initialize components
    var global_state = try initComponents();
    defer global_state.deinit();

    // Print welcome message
    try stdout.writeAll("\nFacet Crystal Calculator v1.0.0\n");
    try stdout.writeAll("Type 'help' for commands, 'exit' to quit\n\n");

    // Start CLI interface
    try global_state.cli.?.run();
}

/// Initialize all components
fn initComponents() !GlobalState {
    // Initialize crystal lattice
    const crystal_lattice = try CrystalLattice.init(.{  // Changed var to const
        .clarity = Config.clarity,
        .facets = 6,
        .symmetry = 1.0,
        .enable_dispersion = true,
    });

    // Initialize resonance state
    const resonance_state = try Attunement.init(crystal_lattice, .{  // Changed var to const
        .min_resonance = Config.resonance_threshold,
        .target_resonance = Config.attunement_strength,
    });

    // Initialize calculator
    const calculator = try Calculator.init(.{  // Changed var to const
        .resonance_state = resonance_state,
        .crystal_lattice = crystal_lattice,
    });

    // Initialize CLI interface
    const cli = try CLI.init(.{  // Changed var to const
        .calculator = calculator,
    });

    return GlobalState{
        .calculator = calculator,
        .resonance_state = resonance_state,
        .crystal_lattice = crystal_lattice,
        .cli = cli,
    };
}

/// Global state type definition
const GlobalState = struct {
    calculator: ?Calculator = null,
    resonance_state: ?Attunement = null,
    crystal_lattice: ?CrystalLattice = null,
    cli: ?*CLI = null,

    /// Deinitialize all components
    pub fn deinit(self: *GlobalState) void {
        if (self.cli) |cli_inst| {
            const mutable_cli: *CLI = @constCast(cli_inst);
            mutable_cli.deinit();
        }
        if (self.calculator) |calculator_inst| calculator_inst.deinit();
        if (self.resonance_state) |res_inst| res_inst.deinit();
        if (self.crystal_lattice) |lattice_inst| lattice_inst.deinit();
    }
};

test "init_components" {
    var state = try initComponents();
    defer state.deinit();

    try std.testing.expect(state.calculator != null);
    try std.testing.expect(state.resonance_state != null);
    try std.testing.expect(state.crystal_lattice != null);
    try std.testing.expect(state.cli != null);
}

test "config_values" {
    try std.testing.expect(Config.clarity > 0.0);
    try std.testing.expect(Config.clarity <= 1.0);
    try std.testing.expect(Config.resonance_threshold > 0.0);
    try std.testing.expect(Config.attunement_strength > Config.resonance_threshold);
}
