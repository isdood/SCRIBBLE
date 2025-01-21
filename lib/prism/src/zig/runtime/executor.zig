//! executor.zig - Task executor for Prism async runtime
//! Created by: isdood
//! Date: 2025-01-21 10:55:46 UTC

const std = @import("std");
const Allocator = std.mem.Allocator;
const Thread = std.Thread;
const ArrayListUnmanaged = std.ArrayListUnmanaged;
const AutoHashMap = std.AutoHashMap;

const Future = @import("future.zig").Future;
const Timer = @import("timer.zig").Timer;
const crystal = @import("../crystal/lattice.zig");

/// Executor errors
pub const ExecutorError = error{
    TaskQueueFull,
    NoAvailableThreads,
    CrystalMisaligned,
    FutureError,
    ExecutorShutdown,
};

/// Task status in the executor
pub const TaskStatus = enum {
    Ready,
    Running,
    Waiting,
    Completed,
    Failed,
};

/// Represents a task in the executor
pub const Task = struct {
    id: u64,
    future: *Future,
    status: TaskStatus,
    dependencies: ArrayListUnmanaged(*Task),
    crystal_node: ?*crystal.LatticeNode,
    completion_callback: ?fn (*Task) void,

    pub fn init(allocator: Allocator, future: *Future) !*Task {
        const task = try allocator.create(Task);
        task.* = .{
            .id = undefined, // Set by executor
            .future = future,
            .status = .Ready,
            .dependencies = .{},
            .crystal_node = null,
            .completion_callback = null,
        };
        return task;
    }

    pub fn deinit(self: *Task, allocator: Allocator) void {
        self.dependencies.deinit(allocator);
    }
};

/// Thread pool worker
const Worker = struct {
    thread: Thread,
    executor: *Executor,
    id: usize,
    running: bool,

    fn spawn(executor: *Executor, id: usize) !Worker {
        return Worker{
            .thread = try Thread.spawn(.{}, workerMain, .{ executor, id }),
            .executor = executor,
            .id = id,
            .running = true,
        };
    }

    fn workerMain(executor: *Executor, id: usize) void {
        while (executor.workers.items[id].running) {
            if (executor.getNextTask()) |task| {
                executor.executeTask(task, id) catch |err| {
                    std.debug.print("Worker {d} task execution error: {}\n", .{ id, err });
                    task.status = .Failed;
                };
            } else {
                // No tasks available, sleep briefly
                std.time.sleep(10 * std.time.ns_per_ms);
            }
        }
    }
};

/// Main executor structure
pub const Executor = struct {
    allocator: Allocator,
    tasks: AutoHashMap(u64, *Task),
    workers: ArrayListUnmanaged(Worker),
    crystal_lattice: *crystal.Lattice,
    timer: Timer,
    next_task_id: std.atomic.Value(u64),
    shutdown_flag: std.atomic.Bool,

    const Self = @This();

    /// Initialize a new executor
    pub fn init(allocator: Allocator, thread_count: usize) !*Self {
        const self = try allocator.create(Self);
        self.* = .{
            .allocator = allocator,
            .tasks = AutoHashMap(u64, *Task).init(allocator),
            .workers = .{},
            .crystal_lattice = try crystal.Lattice.init(allocator),
            .timer = Timer.init(),
            .next_task_id = std.atomic.Value(u64).init(0),
            .shutdown_flag = std.atomic.Bool.init(false),
        };

        // Initialize worker threads
        try self.workers.ensureTotalCapacity(allocator, thread_count);
        var i: usize = 0;
        while (i < thread_count) : (i += 1) {
            const worker = try Worker.spawn(self, i);
            self.workers.appendAssumeCapacity(worker);
        }

        return self;
    }

    /// Clean up resources
    pub fn deinit(self: *Self) void {
        self.shutdown_flag.store(true, .Release);

        // Wait for workers to finish
        for (self.workers.items) |*worker| {
            worker.running = false;
            worker.thread.join();
        }
        self.workers.deinit(self.allocator);

        // Clean up tasks
        var task_iterator = self.tasks.valueIterator();
        while (task_iterator.next()) |task| {
            task.deinit(self.allocator);
            self.allocator.destroy(task);
        }
        self.tasks.deinit();

        self.crystal_lattice.deinit();
        self.allocator.destroy(self);
    }

    /// Submit a new task to the executor
    pub fn submit(self: *Self, future: *Future) !*Task {
        if (self.shutdown_flag.load(.Acquire)) {
            return ExecutorError.ExecutorShutdown;
        }

        const task = try Task.init(self.allocator, future);
        task.id = self.next_task_id.fetchAdd(1, .Monotonic);

        // Integrate with crystal lattice
        task.crystal_node = try self.crystal_lattice.addNode(.{
            @intToFloat(f64, task.id % 100),
            @intToFloat(f64, (task.id / 100) % 100),
            0,
        });

        try self.tasks.put(task.id, task);
        return task;
    }

    /// Add a dependency between tasks
    pub fn addDependency(self: *Self, task: *Task, dependency: *Task) !void {
        _ = self;
        try task.dependencies.append(self.allocator, dependency);
    }

    /// Get the next available task
    fn getNextTask(self: *Self) ?*Task {
        var iterator = self.tasks.valueIterator();
        while (iterator.next()) |task| {
            if (task.status == .Ready and self.canExecuteTask(task)) {
                return task;
            }
        }
        return null;
    }

    /// Check if a task can be executed
    fn canExecuteTask(self: *Self, task: *Task) bool {
        _ = self;
        // Check dependencies
        for (task.dependencies.items) |dep| {
            if (dep.status != .Completed) {
                return false;
            }
        }
        return true;
    }

    /// Execute a task on a worker thread
    fn executeTask(self: *Self, task: *Task, worker_id: usize) !void {
        _ = worker_id;
        if (!self.crystal_lattice.checkAlignment(task)) {
            return ExecutorError.CrystalMisaligned;
        }

        task.status = .Running;
        try task.future.execute();
        task.status = .Completed;

        if (task.completion_callback) |callback| {
            callback(task);
        }
    }

    /// Wait for a specific task to complete
    pub fn waitForTask(self: *Self, task: *Task) !void {
        while (task.status != .Completed and task.status != .Failed) {
            std.time.sleep(1 * std.time.ns_per_ms);
            if (self.shutdown_flag.load(.Acquire)) {
                return ExecutorError.ExecutorShutdown;
            }
        }

        if (task.status == .Failed) {
            return ExecutorError.FutureError;
        }
    }

    /// Set a callback for task completion
    pub fn setCompletionCallback(self: *Self, task: *Task, callback: fn (*Task) void) !void {
        _ = self;
        task.completion_callback = callback;
    }

    /// Get the current status of a task
    pub fn getTaskStatus(self: *Self, task_id: u64) ?TaskStatus {
        if (self.tasks.get(task_id)) |task| {
            return task.status;
        }
        return null;
    }
};

test "executor basic functionality" {
    const allocator = std.testing.allocator;
    
    const executor = try Executor.init(allocator, 4);
    defer executor.deinit();

    // Create a test future
    const TestFuture = struct {
        value: i32,
        const Self = @This();
        pub fn init() Self {
            return .{ .value = 42 };
        }
        pub fn execute(self: *Self) !void {
            _ = self;
        }
    };

    var future = TestFuture.init();
    const task = try executor.submit(@ptrCast(*Future, &future));
    
    try executor.waitForTask(task);
    try std.testing.expect(task.status == .Completed);
}

test "task dependencies" {
    const allocator = std.testing.allocator;
    
    const executor = try Executor.init(allocator, 4);
    defer executor.deinit();

    // Create test futures
    const TestFuture = struct {
        value: i32,
        const Self = @This();
        pub fn init(v: i32) Self {
            return .{ .value = v };
        }
        pub fn execute(self: *Self) !void {
            _ = self;
        }
    };

    var future1 = TestFuture.init(1);
    var future2 = TestFuture.init(2);

    const task1 = try executor.submit(@ptrCast(*Future, &future1));
    const task2 = try executor.submit(@ptrCast(*Future, &future2));

    try executor.addDependency(task2, task1);
    try executor.waitForTask(task2);

    try std.testing.expect(task1.status == .Completed);
    try std.testing.expect(task2.status == .Completed);
}
