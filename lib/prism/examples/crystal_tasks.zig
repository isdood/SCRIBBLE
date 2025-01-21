// crystal_tasks.zig - Crystal task management examples
// Created by: isdood
// Date: 2025-01-21 11:21:38 UTC

const std = @import("std");
const prism = @import("prism");
const Crystal = prism.crystal.Crystal;
const Pattern = prism.pattern.Pattern;
const Runtime = prism.core.Runtime;
const Task = prism.core.Task;
const Timer = prism.timer.Timer;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Initialize runtime with crystal-specific configuration
    var runtime = try Runtime.init(allocator, .{
        .thread_count = 8,
        .stack_size = 2 * 1024 * 1024,
        .use_hardware_threads = true,
        .crystal_optimization_enabled = true,
    });
    defer runtime.deinit();

    // Initialize crystal systems for different patterns
    const CrystalSet = struct {
        cubic: Crystal,
        hexagonal: Crystal,
        tetragonal: Crystal,
        active_index: usize = 0,

        pub fn init(alloc: std.mem.Allocator) !@This() {
            return .{
                .cubic = try Crystal.init(alloc, .Cubic),
                .hexagonal = try Crystal.init(alloc, .Hexagonal),
                .tetragonal = try Crystal.init(alloc, .Tetragonal),
            };
        }

        pub fn deinit(self: *@This()) void {
            self.cubic.deinit();
            self.hexagonal.deinit();
            self.tetragonal.deinit();
        }

        pub fn getActive(self: *@This()) *Crystal {
            return switch (self.active_index) {
                0 => &self.cubic,
                1 => &self.hexagonal,
                2 => &self.tetragonal,
                else => unreachable,
            };
        }

        pub fn rotate(self: *@This()) void {
            self.active_index = (self.active_index + 1) % 3;
        }
    };

    var crystal_set = try CrystalSet.init(allocator);
    defer crystal_set.deinit();

    // Pattern generation context
    const PatternContext = struct {
        crystal: *Crystal,
        pattern: Pattern,
        iteration: usize = 0,
        
        const max_iterations = 5;

        pub fn init(alloc: std.mem.Allocator, crystal: *Crystal, pattern_type: Pattern.Type) !@This() {
            return .{
                .crystal = crystal,
                .pattern = try Pattern.init(alloc, .{
                    .pattern_type = pattern_type,
                    .spacing = 1.0,
                    .scale = 1.0,
                    .rotation = .{ 0.0, 0.0, 0.0 },
                    .symmetry = 8,
                }),
            };
        }

        pub fn deinit(self: *@This()) void {
            self.pattern.deinit();
        }

        pub fn generate(self: *@This()) !void {
            std.debug.print("Generating pattern iteration {}/{}\n", .{ self.iteration + 1, max_iterations });
            try self.pattern.generate(self.crystal);
            self.iteration += 1;
        }

        pub fn isComplete(self: *@This()) bool {
            return self.iteration >= max_iterations;
        }
    };

    // Create pattern contexts for each crystal type
    var pattern_contexts = [_]PatternContext{
        try PatternContext.init(allocator, &crystal_set.cubic, .Cubic),
        try PatternContext.init(allocator, &crystal_set.hexagonal, .Hexagonal),
        try PatternContext.init(allocator, &crystal_set.tetragonal, .Grid),
    };
    defer for (&pattern_contexts) |*ctx| ctx.deinit();

    // Task scheduling context
    const SchedulerContext = struct {
        runtime: *Runtime,
        crystal_set: *CrystalSet,
        pattern_contexts: []PatternContext,
        current_task: ?Task = null,

        pub fn schedule(self: *@This()) !void {
            if (self.current_task) |task| {
                const status = try self.runtime.getTaskStatus(task);
                if (status == .Completed) {
                    self.current_task = null;
                    self.crystal_set.rotate();
                }
            }

            if (self.current_task == null) {
                const ctx = &self.pattern_contexts[self.crystal_set.active_index];
                if (!ctx.isComplete()) {
                    self.current_task = try self.runtime.createTask(
                        *PatternContext,
                        PatternContext.generate,
                        ctx,
                        .{ .priority = .Normal },
                    );
                    try self.runtime.executeTask(self.current_task.?);
                }
            }
        }
    };

    var scheduler = SchedulerContext{
        .runtime = &runtime,
        .crystal_set = &crystal_set,
        .pattern_contexts = &pattern_contexts,
    };

    // Monitoring context
    const MonitorContext = struct {
        crystal_set: *CrystalSet,
        last_stability: f64 = 0.0,

        pub fn monitor(self: *@This()) !void {
            const crystal = self.crystal_set.getActive();
            const stability = crystal.getStability();
            
            if (stability != self.last_stability) {
                std.debug.print("Crystal stability changed: {d:.4} -> {d:.4}\n", 
                    .{ self.last_stability, stability });
                self.last_stability = stability;
            }
        }
    };

    var monitor = MonitorContext{ .crystal_set = &crystal_set };
    var timer = try Timer.init();
    defer timer.deinit();

    // Set up periodic monitoring
    _ = try timer.setInterval(100 * std.time.ns_per_ms, MonitorContext.monitor, &monitor);

    // Main event loop
    const start_time = std.time.nanoTimestamp();
    const timeout_ns = 30 * std.time.ns_per_s;

    while (true) {
        // Check completion
        var all_complete = true;
        for (pattern_contexts) |ctx| {
            if (!ctx.isComplete()) {
                all_complete = false;
                break;
            }
        }
        if (all_complete) break;

        // Check timeout
        if (std.time.nanoTimestamp() - start_time > timeout_ns) {
            std.debug.print("Execution timed out\n", .{});
            return error.Timeout;
        }

        // Update components
        try scheduler.schedule();
        try timer.update();
        try runtime.update();

        // Small delay to prevent busy-waiting
        std.time.sleep(1 * std.time.ns_per_ms);
    }

    // Final stability report
    std.debug.print("\nFinal stability report:\n", .{});
    std.debug.print("Cubic: {d:.4}\n", .{crystal_set.cubic.getStability()});
    std.debug.print("Hexagonal: {d:.4}\n", .{crystal_set.hexagonal.getStability()});
    std.debug.print("Tetragonal: {d:.4}\n", .{crystal_set.tetragonal.getStability()});
}

test "crystal tasks" {
    // Basic functionality test
    const allocator = std.testing.allocator;
    
    var runtime = try Runtime.init(allocator, .{});
    defer runtime.deinit();

    var crystal = try Crystal.init(allocator, .Cubic);
    defer crystal.deinit();

    var pattern = try Pattern.init(allocator, .{
        .pattern_type = .Cubic,
        .spacing = 1.0,
        .scale = 1.0,
        .rotation = .{ 0.0, 0.0, 0.0 },
        .symmetry = 4,
    });
    defer pattern.deinit();

    const TestContext = struct {
        crystal: *Crystal,
        pattern: *Pattern,
        completed: bool = false,

        pub fn run(self: *@This()) !void {
            try self.pattern.generate(self.crystal);
            self.completed = true;
        }
    };

    var context = TestContext{
        .crystal = &crystal,
        .pattern = &pattern,
    };

    const task = try runtime.createTask(*TestContext, TestContext.run, &context, .{});
    try runtime.executeTask(task);
    try runtime.waitForAll();

    try std.testing.expect(context.completed);
    try std.testing.expect(crystal.getNodeCount() > 0);
    try std.testing.expect(crystal.getStability() > 0.0);
}
