//! scheduler.zig - Crystal-based task scheduler for Prism async runtime
//! Created by: isdood
//! Date: 2025-01-21 10:44:21 UTC

const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const AutoHashMap = std.AutoHashMap;

const harmony = @import("harmony.zig");
const wave = @import("wave.zig");
const crystal = @import("../crystal/lattice.zig");

/// Core scheduler error set
pub const SchedulerError = error{
    TaskCreationFailed,
    HarmonyDisrupted,
    LatticeOverflow,
    ResonanceLost,
    WaveInterference,
};

/// Task priority based on crystal face alignment
pub const Priority = enum {
    Perfect,    // Perfectly aligned crystal faces
    Aligned,    // Well-aligned but not perfect
    Partial,    // Partial alignment
    Amorphous,  // No clear alignment
};

/// Represents a schedulable task in the crystal lattice
pub const Task = struct {
    id: u64,
    priority: Priority,
    resonance: f64,
    wave_pattern: wave.Pattern,
    harmony_state: harmony.HarmonyState,
    completion_flag: std.atomic.Value(bool),

    pub fn init(id: u64, priority: Priority) !Task {
        return Task{
            .id = id,
            .priority = priority,
            .resonance = 1.0,
            .wave_pattern = try wave.Pattern.init(),
            .harmony_state = harmony.HarmonyState.init(1.0),
            .completion_flag = std.atomic.Value(bool).init(false),
        };
    }

    pub fn complete(self: *Task) void {
        self.completion_flag.store(true, .Release);
    }
};

/// The main crystal-lattice based scheduler
pub const Scheduler = struct {
    allocator: Allocator,
    crystal_lattice: crystal.Lattice,
    task_pool: ArrayList(Task),
    active_tasks: AutoHashMap(u64, *Task),
    harmony_threshold: f64,
    next_task_id: std.atomic.Value(u64),

    const Self = @This();

    /// Initialize a new scheduler with the given allocator
    pub fn init(allocator: Allocator) !*Self {
        const self = try allocator.create(Self);
        self.* = Self{
            .allocator = allocator,
            .crystal_lattice = try crystal.Lattice.init(allocator),
            .task_pool = ArrayList(Task).init(allocator),
            .active_tasks = AutoHashMap(u64, *Task).init(allocator),
            .harmony_threshold = 0.95,
            .next_task_id = std.atomic.Value(u64).init(0),
        };
        return self;
    }

    /// Clean up resources
    pub fn deinit(self: *Self) void {
        self.task_pool.deinit();
        self.active_tasks.deinit();
        self.crystal_lattice.deinit();
        self.allocator.destroy(self);
    }

    /// Schedule a new task with given priority
    pub fn schedule(self: *Self, priority: Priority) !*Task {
        const task_id = self.next_task_id.fetchAdd(1, .Monotonic);
        
        var task = try Task.init(task_id, priority);
        try self.task_pool.append(task);
        
        // Get pointer to the task we just added
        const task_ptr = &self.task_pool.items[self.task_pool.items.len - 1];
        try self.active_tasks.put(task_id, task_ptr);
        
        // Integrate task into crystal lattice
        try self.crystal_lattice.integrateTask(task_ptr) catch |err| {
            _ = self.active_tasks.remove(task_id);
            return err;
        };

        return task_ptr;
    }

    /// Execute the next task that maintains highest harmony
    pub fn executeNext(self: *Self) !?*Task {
        var best_task: ?*Task = null;
        var max_resonance: f64 = 0;

        // Find task with highest resonance that maintains harmony
        var iterator = self.active_tasks.valueIterator();
        while (iterator.next()) |task| {
            const combined_resonance = self.calculateResonance(task);
            if (combined_resonance > max_resonance and 
                self.maintainsHarmony(task)) {
                max_resonance = combined_resonance;
                best_task = task;
            }
        }

        if (best_task) |task| {
            try self.prepareExecution(task);
        }

        return best_task;
    }

    /// Calculate the resonance value for a task
    fn calculateResonance(self: *Self, task: *Task) f64 {
        const priority_factor = switch (task.priority) {
            .Perfect => 1.0,
            .Aligned => 0.8,
            .Partial => 0.6,
            .Amorphous => 0.4,
        };

        return task.resonance * 
               task.harmony_state.resonance * 
               priority_factor * 
               self.crystal_lattice.getStability();
    }

    /// Check if executing a task maintains system harmony
    fn maintainsHarmony(self: *Self, task: *Task) bool {
        return task.harmony_state.resonance >= self.harmony_threshold and
               self.crystal_lattice.checkAlignment(task);
    }

    /// Prepare a task for execution
    fn prepareExecution(self: *Self, task: *Task) !void {
        try self.crystal_lattice.alignTask(task);
        task.harmony_state.maintainHarmony();
        try self.validateHarmony(task);
    }

    /// Validate overall system harmony
    fn validateHarmony(self: *Self, task: *Task) !void {
        if (task.harmony_state.resonance < self.harmony_threshold) {
            return error.HarmonyDisrupted;
        }
        if (!self.crystal_lattice.isStable()) {
            return error.LatticeOverflow;
        }
    }
};

test "scheduler basic functionality" {
    const allocator = std.testing.allocator;
    
    const scheduler = try Scheduler.init(allocator);
    defer scheduler.deinit();

    // Schedule a perfect priority task
    const task = try scheduler.schedule(.Perfect);
    try std.testing.expect(task.priority == .Perfect);
    try std.testing.expect(task.resonance == 1.0);
    
    // Verify task execution maintains harmony
    const next_task = try scheduler.executeNext();
    try std.testing.expect(next_task != null);
    try std.testing.expect(next_task.?.id == task.id);
}
