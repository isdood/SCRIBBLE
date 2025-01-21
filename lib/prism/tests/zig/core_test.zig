// core_test.zig - Core functionality tests for Prism
// Created by: isdood
// Date: 2025-01-21 11:17:06 UTC

const std = @import("std");
const testing = std.testing;
const core = @import("core");
const Timer = @import("timer").Timer;
const Pattern = @import("pattern").Pattern;
const Crystal = @import("crystal").Crystal;

test "core initialization" {
    const allocator = testing.allocator;
    var runtime = try core.Runtime.init(allocator, .{
        .thread_count = 4,
        .stack_size = 1024 * 1024,
        .use_hardware_threads = true,
    });
    defer runtime.deinit();

    try testing.expect(runtime.isInitialized());
    try testing.expectEqual(runtime.getThreadCount(), 4);
}

test "task creation and execution" {
    const allocator = testing.allocator;
    var runtime = try core.Runtime.init(allocator, .{});
    defer runtime.deinit();

    const TaskContext = struct {
        value: i32,
        completed: bool,

        pub fn run(self: *@This()) void {
            self.value += 1;
            self.completed = true;
        }
    };

    var context = TaskContext{ .value = 0, .completed = false };
    const task = try runtime.createTask(*TaskContext, TaskContext.run, &context, .{});
    
    try runtime.executeTask(task);
    try testing.expectEqual(context.value, 1);
    try testing.expect(context.completed);
}

test "timer functionality" {
    const allocator = testing.allocator;
    var timer = try Timer.init();
    defer timer.deinit();

    var callback_called = false;
    const TestContext = struct {
        called: *bool,

        pub fn onTimer(self: *@This()) void {
            self.called.* = true;
        }
    };

    var context = TestContext{ .called = &callback_called };
    const timer_id = try timer.setTimeout(10 * std.time.ns_per_ms, TestContext.onTimer, &context);
    
    // Wait for timer
    std.time.sleep(20 * std.time.ns_per_ms);
    try timer.update();
    
    try testing.expect(callback_called);
    try testing.expect(timer.getActiveCount() == 0);
}

test "crystal pattern generation" {
    const allocator = testing.allocator;
    var crystal = try Crystal.init(allocator, .Cubic);
    defer crystal.deinit();

    var pattern = try Pattern.init(allocator, .{
        .pattern_type = .Cubic,
        .spacing = 1.0,
        .scale = 1.0,
        .rotation = .{ 0.0, 0.0, 0.0 },
        .symmetry = 8,
    });
    defer pattern.deinit();

    try pattern.generate(&crystal);
    try testing.expect(crystal.getNodeCount() > 0);
    try testing.expect(crystal.getStability() > 0.0);
}

test "concurrent task execution" {
    const allocator = testing.allocator;
    var runtime = try core.Runtime.init(allocator, .{
        .thread_count = 4,
        .use_hardware_threads = true,
    });
    defer runtime.deinit();

    const TaskContext = struct {
        counter: *std.atomic.Atomic(usize),

        pub fn run(self: *@This()) void {
            _ = self.counter.fetchAdd(1, .SeqCst);
            std.time.sleep(10 * std.time.ns_per_ms);
        }
    };

    var counter = std.atomic.Atomic(usize).init(0);
    var contexts: [10]TaskContext = undefined;
    var tasks: [10]core.Task = undefined;

    // Create multiple tasks
    for (&contexts, 0..) |*ctx, i| {
        ctx.* = .{ .counter = &counter };
        tasks[i] = try runtime.createTask(*TaskContext, TaskContext.run, ctx, .{});
    }

    // Execute all tasks
    for (tasks) |task| {
        try runtime.executeTask(task);
    }

    try runtime.waitForAll();
    try testing.expectEqual(counter.load(.SeqCst), 10);
}

test "error handling" {
    const allocator = testing.allocator;
    var runtime = try core.Runtime.init(allocator, .{});
    defer runtime.deinit();

    const ErrorContext = struct {
        pub fn run(self: *@This()) core.Error!void {
            return error.TestError;
        }
    };

    var context = ErrorContext{};
    const task = try runtime.createTask(*ErrorContext, ErrorContext.run, &context, .{});
    
    try testing.expectError(error.TestError, runtime.executeTask(task));
}

test "task priorities" {
    const allocator = testing.allocator;
    var runtime = try core.Runtime.init(allocator, .{});
    defer runtime.deinit();

    const PriorityContext = struct {
        order: *std.ArrayList(core.Priority),
        priority: core.Priority,

        pub fn run(self: *@This()) void {
            self.order.append(self.priority) catch unreachable;
        }
    };

    var order = std.ArrayList(core.Priority).init(allocator);
    defer order.deinit();

    // Create tasks with different priorities
    const priorities = [_]core.Priority{ .Low, .High, .Normal };
    var contexts: [3]PriorityContext = undefined;
    var tasks: [3]core.Task = undefined;

    for (&contexts, &tasks, priorities) |*ctx, *task, priority| {
        ctx.* = .{ .order = &order, .priority = priority };
        task.* = try runtime.createTask(
            *PriorityContext,
            PriorityContext.run,
            ctx,
            .{ .priority = priority },
        );
    }

    // Execute all tasks
    for (tasks) |task| {
        try runtime.executeTask(task);
    }

    try runtime.waitForAll();
    try testing.expectEqual(order.items[0], .High);
}

test "crystal optimization" {
    const allocator = testing.allocator;
    var crystal = try Crystal.init(allocator, .Cubic);
    defer crystal.deinit();

    // Add some nodes
    var positions = [_][3]f64{
        .{ 0.0, 0.0, 0.0 },
        .{ 1.0, 0.0, 0.0 },
        .{ 0.0, 1.0, 0.0 },
        .{ 1.0, 1.0, 0.0 },
    };

    for (positions) |pos| {
        try crystal.addNode(pos);
    }

    const initial_stability = crystal.getStability();
    try crystal.optimize();
    const final_stability = crystal.getStability();

    try testing.expect(final_stability >= initial_stability);
}

test "runtime stress test" {
    const allocator = testing.allocator;
    var runtime = try core.Runtime.init(allocator, .{
        .thread_count = 8,
        .use_hardware_threads = true,
    });
    defer runtime.deinit();

    const StressContext = struct {
        counter: *std.atomic.Atomic(usize),

        pub fn run(self: *@This()) void {
            _ = self.counter.fetchAdd(1, .SeqCst);
            std.time.sleep(1 * std.time.ns_per_ms);
        }
    };

    var counter = std.atomic.Atomic(usize).init(0);
    const task_count = 1000;
    var contexts = try allocator.alloc(StressContext, task_count);
    defer allocator.free(contexts);

    for (contexts) |*ctx| {
        ctx.* = .{ .counter = &counter };
        const task = try runtime.createTask(*StressContext, StressContext.run, ctx, .{});
        try runtime.executeTask(task);
    }

    try runtime.waitForAll();
    try testing.expectEqual(counter.load(.SeqCst), task_count);
}
