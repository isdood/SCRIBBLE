//! Facet Resonance Attunement
//! Author: @isdood
//! Created: 2025-01-21 12:54:29 UTC

const std = @import("std");
const crystal = @import("../crystal/lattice.zig");
const types = @import("../core/types.zig");

const CrystalLattice = crystal.CrystalLattice;
const Result = types.Result;

/// Attunement configuration
pub const AttunementConfig = struct {
    /// Minimum acceptable resonance level
    min_resonance: f64 = 0.87,
    /// Target resonance level for optimal performance
    target_resonance: f64 = 0.95,
    /// Maximum number of attunement attempts
    max_attempts: u8 = 5,
    /// Resonance decay rate
    decay_rate: f64 = 0.01,
    /// Crystal attunement factor
    attunement_factor: f64 = 1.2,
    /// Enable adaptive resonance
    adaptive_resonance: bool = true,
};

/// Resonance attunement states
const AttunementState = enum {
    Initializing,
    Attuning,
    Stabilizing,
    Resonating,
    Perfect,
    Failed,
};

/// Resonance attunement for crystal calculations
pub const Attunement = struct {
    config: AttunementConfig,
    current_resonance: f64,
    attunement_level: f64,
    crystal_lattice: *CrystalLattice,
    state: AttunementState,

    const Self = @This();

    /// Initialize new attunement
    pub fn init(crystal_lattice: *CrystalLattice, config: ?AttunementConfig) !*Self {
        const attunement = try std.heap.page_allocator.create(Self);
        attunement.* = .{
            .config = config orelse AttunementConfig{},
            .current_resonance = 0.0,
            .attunement_level = 1.0,
            .crystal_lattice = crystal_lattice,
            .state = .Initializing,
        };
        return attunement;
    }

    /// Clean up attunement resources
    pub fn deinit(self: *Self) void {
        std.heap.page_allocator.destroy(self);
    }

    /// Optimize resonance for calculation
    pub fn optimize(self: *Self, result: *Result) !void {
        self.state = .Attuning;
        var attempts: u8 = 0;

        while (attempts < self.config.max_attempts) : (attempts += 1) {
            // Check current resonance level
            if (self.current_resonance >= self.config.target_resonance) {
                self.state = .Perfect;
                break;
            }

            // Attempt resonance attunement
            try self.attuneCrystals(result);
            try self.stabilizeResonance();

            // Update state based on resonance level
            if (self.current_resonance >= self.config.min_resonance) {
                self.state = .Resonating;
            } else {
                // Apply adaptive resonance if enabled
                if (self.config.adaptive_resonance) {
                    try self.adaptResonance();
                }
            }

            // Check if we've achieved target resonance
            if (self.current_resonance >= self.config.target_resonance) {
                self.state = .Perfect;
                break;
            }
        }

        // Update result with final resonance values
        result.resonance = self.current_resonance;
        if (self.state == .Failed) {
            return error.ResonanceAttunementFailed;
        }
    }

    /// Attune crystals to improve resonance
    fn attuneCrystals(self: *Self, result: *Result) !void {
        // Calculate base attunement
        const base_attunement = self.attunement_level * self.config.attunement_factor;

        // Apply crystal-specific attunement
        try self.crystal_lattice.attune(base_attunement);

        // Update resonance based on crystal response
        self.current_resonance = @min(
            1.0,
            self.current_resonance + (base_attunement * self.crystal_lattice.clarity)
        );

        // Apply natural decay
        self.current_resonance *= (1.0 - self.config.decay_rate);
    }

    /// Stabilize current resonance level
    fn stabilizeResonance(self: *Self) !void {
        // Calculate stability factor based on crystal clarity
        const stability = self.crystal_lattice.clarity * self.attunement_level;

        // Apply stability adjustments
        self.current_resonance = @min(
            1.0,
            self.current_resonance * stability
        );

        // Update attunement level based on stability
        self.attunement_level *= @max(0.95, stability);
    }

    /// Adapt resonance parameters for optimization
    fn adaptResonance(self: *Self) !void {
        // Increase attunement factor based on current resonance
        if (self.current_resonance < self.config.min_resonance) {
            self.attunement_level *= 1.1;
        }

        // Adjust decay rate based on crystal clarity
        const clarity_factor = self.crystal_lattice.clarity;
        self.config.decay_rate *= @max(0.9, clarity_factor);
    }

    /// Get current attunement state
    pub fn getState(self: *const Self) AttunementState {
        return self.state;
    }

    /// Check if attunement is perfect
    pub fn isPerfect(self: *const Self) bool {
        return self.state == .Perfect;
    }

    /// Get current resonance metrics
    pub fn getMetrics(self: *const Self) struct {
        resonance: f64,
        attunement: f64,
        state: AttunementState,
    } {
        return .{
            .resonance = self.current_resonance,
            .attunement = self.attunement_level,
            .state = self.state,
        };
    }
};

test "attunement_basic" {
    // Create test crystal lattice
    var lattice = try CrystalLattice.init(.{
        .clarity = 0.95,
        .facets = 3,
    });
    defer lattice.deinit();

    // Create attunement
    var attunement = try Attunement.init(&lattice, null);
    defer attunement.deinit();

    // Test attunement
    var result = Result{
        .value = 42.0,
        .resonance = 0.0,
        .clarity = 0.95,
    };

    try attunement.optimize(&result);
    try std.testing.expect(result.resonance >= attunement.config.min_resonance);
}

test "attunement_perfect_resonance" {
    var lattice = try CrystalLattice.init(.{
        .clarity = 1.0,
        .facets = 4,
    });
    defer lattice.deinit();

    var attunement = try Attunement.init(&lattice, .{
        .min_resonance = 0.9,
        .target_resonance = 0.95,
    });
    defer attunement.deinit();

    var result = Result{
        .value = 42.0,
        .resonance = 0.0,
        .clarity = 1.0,
    };

    try attunement.optimize(&result);
    try std.testing.expect(attunement.isPerfect());
}
