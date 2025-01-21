//! Facet State Manager
//! Author: @isdood
//! Created: 2025-01-21 13:03:18 UTC

const std = @import("std");
const crystal = @import("../crystal/lattice.zig");
const resonance = @import("../resonance/attunement.zig");
const error = @import("error.zig");

const CrystalLattice = crystal.CrystalLattice;
const ResonanceState = resonance.ResonanceState;
const ErrorHandler = error.ErrorHandler;

/// Global state configuration
pub const StateConfig = struct {
    /// Enable state persistence
    enable_persistence: bool = true,
    /// Auto-save interval (seconds)
    auto_save_interval: u32 = 300,
    /// Maximum history size
    max_history_size: usize = 1000,
    /// Debug mode
    debug_mode: bool = false,
    /// Performance monitoring
    enable_monitoring: bool = true,
};

/// State snapshot for persistence
const StateSnapshot = struct {
    /// Timestamp of snapshot
    timestamp: i64,
    /// Crystal clarity
    crystal_clarity: f64,
    /// Resonance level
    resonance_level: f64,
    /// Computation count
    computation_count: usize,
    /// Error count
    error_count: usize,
};

/// Performance metrics
const PerformanceMetrics = struct {
    /// Average computation time
    avg_compute_time: f64,
    /// Crystal efficiency
    crystal_efficiency: f64,
    /// Resonance stability
    resonance_stability: f64,
    /// Cache hit rate
    cache_hit_rate: f64,
    /// Error rate
    error_rate: f64,
};

/// Global state manager
pub const State = struct {
    config: StateConfig,
    crystal_lattice: *CrystalLattice,
    resonance_state: *ResonanceState,
    error_handler: *ErrorHandler,
    history: std.ArrayList(StateSnapshot),
    metrics: PerformanceMetrics,
    last_save: i64,
    computation_count: usize,
    allocator: std.mem.Allocator,

    const Self = @This();

    /// Initialize new state manager
    pub fn init(allocator: std.mem.Allocator, config: ?StateConfig) !*Self {
        const state = try allocator.create(Self);

        // Initialize components
        const crystal_lattice = try CrystalLattice.init(null);
        const resonance_state = try ResonanceState.init(null);
        const error_handler = try ErrorHandler.init(allocator);

        state.* = .{
            .config = config orelse StateConfig{},
            .crystal_lattice = crystal_lattice,
            .resonance_state = resonance_state,
            .error_handler = error_handler,
            .history = std.ArrayList(StateSnapshot).init(allocator),
            .metrics = .{
                .avg_compute_time = 0.0,
                .crystal_efficiency = 1.0,
                .resonance_stability = 1.0,
                .cache_hit_rate = 0.0,
                .error_rate = 0.0,
            },
            .last_save = std.time.timestamp(),
            .computation_count = 0,
            .allocator = allocator,
        };

        // Load persisted state if enabled
        if (state.config.enable_persistence) {
            try state.loadState();
        }

        return state;
    }

    /// Clean up state resources
    pub fn deinit(self: *Self) void {
        // Save state if persistence is enabled
        if (self.config.enable_persistence) {
            self.saveState() catch {};
        }

        self.crystal_lattice.deinit();
        self.resonance_state.deinit();
        self.error_handler.deinit();
        self.history.deinit();
        self.allocator.destroy(self);
    }

    /// Update state after computation
    pub fn update(self: *Self, compute_time: f64) !void {
        self.computation_count += 1;

        // Update performance metrics
        self.updateMetrics(compute_time);

        // Create state snapshot
        try self.takeSnapshot();

        // Auto-save if needed
        if (self.config.enable_persistence) {
            const current_time = std.time.timestamp();
            if (current_time - self.last_save >= self.config.auto_save_interval) {
                try self.saveState();
                self.last_save = current_time;
            }
        }

        // Prune history if needed
        if (self.history.items.len > self.config.max_history_size) {
            _ = self.history.orderedRemove(0);
        }
    }

    /// Take state snapshot
    fn takeSnapshot(self: *Self) !void {
        const snapshot = StateSnapshot{
            .timestamp = std.time.timestamp(),
            .crystal_clarity = self.crystal_lattice.clarity,
            .resonance_level = self.resonance_state.getCurrentLevel(),
            .computation_count = self.computation_count,
            .error_count = self.error_handler.getMetrics().total_errors,
        };

        try self.history.append(snapshot);
    }

    /// Update performance metrics
    fn updateMetrics(self: *Self, compute_time: f64) void {
        const alpha = 0.1; // Exponential moving average factor

        self.metrics.avg_compute_time = (1 - alpha) * self.metrics.avg_compute_time + alpha * compute_time;
        self.metrics.crystal_efficiency = self.crystal_lattice.clarity;
        self.metrics.resonance_stability = self.resonance_state.getStability();
        self.metrics.error_rate = @intToFloat(f64, self.error_handler.getMetrics().total_errors) /
        @intToFloat(f64, self.computation_count);
    }

    /// Save state to persistence
    fn saveState(self: *Self) !void {
        if (!self.config.enable_persistence) return;

        // Implementation would save state to disk/database
        if (self.config.debug_mode) {
            std.log.debug("Saving state... (Computations: {d}, Clarity: {d:.2})", .{
                self.computation_count,
                self.crystal_lattice.clarity,
            });
        }
    }

    /// Load persisted state
    fn loadState(self: *Self) !void {
        if (!self.config.enable_persistence) return;

        // Implementation would load state from disk/database
        if (self.config.debug_mode) {
            std.log.debug("Loading state...", .{});
        }
    }

    /// Get current state metrics
    pub fn getMetrics(self: *const Self) struct {
        performance: PerformanceMetrics,
        crystal_state: crystal.CrystalLattice.Metrics,
        resonance_state: resonance.ResonanceState.Metrics,
        history_size: usize,
    } {
        return .{
            .performance = self.metrics,
            .crystal_state = self.crystal_lattice.getMetrics(),
            .resonance_state = self.resonance_state.getMetrics(),
            .history_size = self.history.items.len,
        };
    }
};

test "state_basic" {
    const allocator = std.testing.allocator;

    var state = try State.init(allocator, null);
    defer state.deinit();

    try state.update(0.001);
    const metrics = state.getMetrics();

    try std.testing.expect(metrics.history_size > 0);
    try std.testing.expect(state.computation_count == 1);
}

test "state_persistence" {
    const allocator = std.testing.allocator;

    var state = try State.init(allocator, .{
        .enable_persistence = true,
        .auto_save_interval = 1,
    });
    defer state.deinit();

    try state.update(0.001);
    const current_time = std.time.timestamp();
    try std.testing.expect(current_time - state.last_save <= 1);
}
