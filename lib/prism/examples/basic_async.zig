// basic_async.zig - Basic async example for Prism
// Created by: isdood
// Date: 2025-01-21 11:20:14 UTC

const std = @import("std");
const prism = @import("prism");
const Crystal = prism.crystal.Crystal;
const Pattern = prism.pattern.Pattern;
const Runtime = prism.core.Runtime;
const Task = prism.core.Task;

pub fn main() !void {
    // Initialize allocator
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Initialize runtime
    var runtime = try Runtime.init(allocator, .{
        .thread_count = 4,
        .stack_size = 1024 * 1024,
        .use_hardware_threads = true,
    });
    defer runtime.deinit();

    // Create crystal system
    var crystal = try Crystal.init(allocator, .Cubic);
    defer crystal.deinit();

    // Create pattern generator
    var pattern = try Pattern.init(allocator, .{
        .pattern_type = .Cubic,
        .spacing = 1.0,
        .scale = 1.0,
        .rotation = .{ 0.0, 0.0, 0.0 },
        .symmetry = 8,
    });
    defer pattern.deinit();

    // Example context for async tasks
    const Context = struct {
        crystal: *Crystal,
        pattern: *Pattern,
        step: usize,
        
        pub fn generatePattern(self: *@This()) !void {
            std.debug.print("Generating pattern (step {})\n", .{self.step});
            try self.pattern.generate(self.crystal);
            self.step += 1;
        }

        pub fn optimizeCrystal(self: *@This()) !void {
            std.debug.print("Optimizing crystal (step {})\n", .{self.step});
            try self.crystal.optimize();
            self.step += 1;
        }

        pub fn measureStability(self: *@This()) !void {
            const stability = self.crystal.getStability();
            std.debug.print("Crystal stability (step {}): {d:.4}\n", .{self.step, stability});
            self.step += 1;
        }
    };

    var context = Context{
        .crystal = &crystal,
        .pattern = &pattern,
        .step = 0,
    };

    // Create async tasks
    const tasks = [_]Task{
        try runtime.createTask(
            *Context,
            Context.generatePattern,
            &context,
            .{ .priority = .High },
        ),
        try runtime.createTask(
            *Context,
            Context.optimizeCrystal,
            &context,
            .{ .priority = .Normal },
        ),
        try runtime.createTask(
            *Context,
            Context.measureStability,
            &context,
            .{ .priority = .Low },
        ),
    };

    // Execute tasks with progress tracking
    const ProgressContext = struct {
        completed: usize = 0,
        total: usize,

        pub fn onProgress(self: *@This()) void {
            self.completed += 1;
            std.debug.print("Progress: {}/{}\n", .{self.completed, self.total});
        }
    };

    var progress = ProgressContext{ .total = tasks.len };

    // Execute tasks asynchronously
    for (tasks) |task| {
        try runtime.executeTask(task);
    }

    // Wait for completion with timeout
    const timeout_ns = 5 * std.time.ns_per_s;
    const start_time = std.time.nanoTimestamp();

    while (progress.completed < progress.total) {
        try runtime.update();
        
        // Check timeout
        if (std.time.nanoTimestamp() - start_time > timeout_ns) {
            std.debug.print("Execution timed out\n", .{});
            return error.Timeout;
        }

        // Yield to allow other tasks to run
        std.time.sleep(1 * std.time.ns_per_ms);
    }

    // Final stability check
    const final_stability = crystal.getStability();
    std.debug.print("\nFinal crystal stability: {d:.4}\n", .{final_stability});

    // Example of async event handling
    const EventContext = struct {
        crystal: *Crystal,
        event_count: usize = 0,

        pub fn handleEvent(self: *@This()) !void {
            self.event_count += 1;
            std.debug.print("Handling event {}\n", .{self.event_count});
            
            if (self.event_count % 2 == 0) {
                try self.crystal.optimize();
                std.debug.print("Optimized crystal after event\n", .{});
            }
        }
    };

    var event_context = EventContext{ .crystal = &crystal };
    const event_handler = try runtime.createTask(
        *EventContext,
        EventContext.handleEvent,
        &event_context,
        .{ .priority = .Normal },
    );

    // Simulate some events
    var i: usize = 0;
    while (i < 4) : (i += 1) {
        try runtime.executeTask(event_handler);
        std.time.sleep(100 * std.time.ns_per_ms);
    }

    // Final cleanup
    try runtime.waitForAll();
    std.debug.print("\nAll tasks completed successfully\n", .{});
}

test "basic async example" {
    // Add tests for the example functionality
    const allocator = std.testing.allocator;
    var runtime = try Runtime.init(allocator, .{});
    defer runtime.deinit();

    var crystal = try Crystal.init(allocator, .Cubic);
    defer crystal.deinit();

    // Test basic task execution
    const TestContext = struct {
        success: bool = false,

        pub fn run(self: *@This()) void {
            self.success = true;
        }
    };

    var context = TestContext{};
    const task = try runtime.createTask(*TestContext, TestContext.run, &context, .{});
    try runtime.executeTask(task);
    try runtime.waitForAll();

    try std.testing.expect(context.success);
}
