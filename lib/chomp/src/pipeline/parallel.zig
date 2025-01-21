///! Parallel Processing Manager
///! =======================
///! Author: Caleb J.D. Terkovics <isdood>
///! Created: 2025-01-21 02:54:58 UTC
///! License: MIT

const std = @import("std");
const safety = @import("../safety/mod.zig");

pub const ParallelProcessor = struct {
    allocator: std.mem.Allocator,
    thread_pool: ThreadPool,
    task_queue: TaskQueue,
    results: ResultCollector,

    pub const ThreadPool = struct {
        threads: []std.Thread,
        work_signal: std.Thread.ResetEvent,
        stop_signal: std.atomic.Bool,

        pub fn init(allocator: std.mem.Allocator, thread_count: usize) !ThreadPool {
            var threads = try allocator.alloc(std.Thread, thread_count);
            errdefer allocator.free(threads);

            var self = ThreadPool{
                .threads = threads,
                .work_signal = std.Thread.ResetEvent{},
                .stop_signal = std.atomic.Bool.init(false),
            };

            for (threads) |*thread, i| {
                thread.* = try std.Thread.spawn(.{}, workerFn, .{ &self, i });
            }

            return self;
        }

        fn workerFn(self: *ThreadPool, thread_id: usize) void {
            while (!self.stop_signal.load(.Acquire)) {
                self.work_signal.wait();
                if (self.stop_signal.load(.Acquire)) break;

                // Process tasks
                while (self.getNextTask()) |task| {
                    self.processTask(task, thread_id);
                }
            }
        }
    };

    pub const TaskQueue = struct {
        tasks: std.ArrayList(Task),
        mutex: std.Thread.Mutex,

        pub const Task = struct {
            id: u64,
            kind: TaskKind,
            data: *anyopaque,
            priority: u8,
        };

        pub const TaskKind = enum {
            parse,
            analyze,
            optimize,
            generate,
        };

        pub fn push(self: *TaskQueue, task: Task) !void {
            self.mutex.lock();
            defer self.mutex.unlock();

            try self.tasks.append(task);
            std.sort.sort(Task, self.tasks.items, {}, taskPriorityLessThan);
        }

        fn taskPriorityLessThan(context: void, a: Task, b: Task) bool {
            _ = context;
            return a.priority > b.priority;
        }
    };

    pub const ResultCollector = struct {
        results: std.AutoHashMap(u64, TaskResult),
        mutex: std.Thread.Mutex,

        pub const TaskResult = struct {
            task_id: u64,
            success: bool,
            data: ?*anyopaque,
            error: ?anyerror,
        };

        pub fn collect(self: *ResultCollector, result: TaskResult) !void {
            self.mutex.lock();
            defer self.mutex.unlock();

            try self.results.put(result.task_id, result);
        }
    };

    pub fn init(allocator: std.mem.Allocator) !*ParallelProcessor {
        var self = try allocator.create(ParallelProcessor);
        self.* = .{
            .allocator = allocator,
            .thread_pool = try ThreadPool.init(allocator, try std.Thread.getCpuCount()),
            .task_queue = .{
                .tasks = std.ArrayList(TaskQueue.Task).init(allocator),
                .mutex = std.Thread.Mutex{},
            },
            .results = .{
                .results = std.AutoHashMap(u64, ResultCollector.TaskResult).init(allocator),
                .mutex = std.Thread.Mutex{},
            },
        };
        return self;
    }

    pub fn scheduleTask(self: *ParallelProcessor, kind: TaskQueue.TaskKind, data: *anyopaque, priority: u8) !u64 {
        const task_id = @intCast(u64, std.time.milliTimestamp());
        const task = TaskQueue.Task{
            .id = task_id,
            .kind = kind,
            .data = data,
            .priority = priority,
        };

        try self.task_queue.push(task);
        self.thread_pool.work_signal.set();
        return task_id;
    }

    pub fn waitForResult(self: *ParallelProcessor, task_id: u64) !ResultCollector.TaskResult {
        while (true) {
            if (self.results.results.get(task_id)) |result| {
                return result;
            }
            std.time.sleep(1 * std.time.ns_per_ms);
        }
    }
};
