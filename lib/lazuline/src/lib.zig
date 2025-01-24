const std = @import("std");
const print = std.debug.print;
const builtin = @import("builtin");
const time = std.time;

pub const WorkFn = *const fn (*anyopaque) void;

pub const Job = struct {
    context: *anyopaque,
    work_fn: WorkFn,
    id: u32,
};

pub const ThreadPool = struct {
    const Self = @This();
    
    allocator: std.mem.Allocator,
    thread: std.Thread,
    mutex: std.Thread.Mutex,
    job_queue: std.ArrayList(*Job),
    completed_jobs: std.atomic.Value(u32),
    running: std.atomic.Value(bool),
    
    pub fn init(allocator: std.mem.Allocator) !Self {
        print("\n[{s}] Creating thread pool (Target: {s})\n", .{
            @typeName(Self),
            @tagName(builtin.cpu.arch)
        });
        
        var pool = Self{
            .allocator = allocator,
            .thread = undefined,
            .mutex = .{},
            .job_queue = std.ArrayList(*Job).init(allocator),
            .completed_jobs = std.atomic.Value(u32).init(0),
            .running = std.atomic.Value(bool).init(true),
        };
        
        print("[MAIN] Starting worker thread\n", .{});
        pool.thread = try std.Thread.spawn(.{}, workerFn, .{&pool});
        std.time.sleep(10 * std.time.ns_per_ms);
        print("[MAIN] Worker thread started\n", .{});
        
        return pool;
    }
    
    fn workerFn(pool: *Self) void {
        print("[WORKER] Thread starting (ID: {})\n", .{std.Thread.getCurrentId()});
        var jobs_completed: u32 = 0;
        
        while (pool.running.load(.seq_cst)) {
            pool.mutex.lock();
            const has_job = pool.job_queue.items.len > 0;
            const job = if (has_job) pool.job_queue.orderedRemove(0) else null;
            pool.mutex.unlock();
            
            if (job) |j| {
                print("[WORKER] Processing job {} at {*}\n", .{j.id, j});
                j.work_fn(j.context);
                
                jobs_completed += 1;
                _ = pool.completed_jobs.fetchAdd(1, .seq_cst);
                print("[WORKER] Completed job {} (total: {})\n", .{j.id, jobs_completed});
                
                pool.allocator.destroy(j);
            } else {
                std.time.sleep(1 * std.time.ns_per_ms);
            }
        }
        
        // Process remaining jobs
        print("[WORKER] Processing remaining jobs\n", .{});
        pool.mutex.lock();
        while (pool.job_queue.items.len > 0) {
            const j = pool.job_queue.orderedRemove(0);
            pool.mutex.unlock();
            
            print("[WORKER] Processing final job {} at {*}\n", .{j.id, j});
            j.work_fn(j.context);
            
            jobs_completed += 1;
            _ = pool.completed_jobs.fetchAdd(1, .seq_cst);
            print("[WORKER] Completed final job {} (total: {})\n", .{j.id, jobs_completed});
            
            pool.allocator.destroy(j);
            pool.mutex.lock();
        }
        pool.mutex.unlock();
        
        print("[WORKER] Thread shutting down (processed {} jobs)\n", .{jobs_completed});
    }
    
    pub fn deinit(self: *Self) void {
        print("[MAIN] Shutting down thread pool\n", .{});
        self.running.store(false, .seq_cst);
        self.thread.join();
        
        self.mutex.lock();
        defer self.mutex.unlock();
        
        // Clean up any remaining jobs
        while (self.job_queue.items.len > 0) {
            const job = self.job_queue.orderedRemove(0);
            self.allocator.destroy(job);
        }
        self.job_queue.deinit();
        
        print("[MAIN] Worker finished\n", .{});
    }
    
    pub fn schedule(self: *Self, context: anytype, comptime work_fn: fn (*const std.meta.Child(@TypeOf(context))) void) !void {
        const PtrType = *const std.meta.Child(@TypeOf(context));
        
        const Wrapper = struct {
            fn call(ptr: *anyopaque) void {
                const typed_ptr = @as(PtrType, @ptrCast(@alignCast(ptr)));
                work_fn(typed_ptr);
            }
        };
        
        const job_id = self.completed_jobs.load(.seq_cst) + 1;
        
        // Create new job
        const job = try self.allocator.create(Job);
        errdefer self.allocator.destroy(job);
        
        job.* = Job{
            .context = @constCast(context),
            .work_fn = Wrapper.call,
            .id = job_id,
        };
        
        // Add job to queue
        print("[MAIN] Scheduling job {} at {*}\n", .{job_id, job});
        self.mutex.lock();
        try self.job_queue.append(job);
        self.mutex.unlock();
        
        // Wait for completion
        const start_time = time.milliTimestamp();
        const timeout_ms = 5000; // 5 second timeout
        
        while (self.completed_jobs.load(.seq_cst) < job_id) {
            if (time.milliTimestamp() - start_time > timeout_ms) {
                print("[MAIN] Job {} timed out\n", .{job_id});
                return error.JobTimeout;
            }
            std.time.sleep(1 * std.time.ns_per_ms);
        }
        
        print("[MAIN] Job {} completed\n", .{job_id});
    }
    
    pub fn wait(self: *Self) void {
        print("[MAIN] Waiting for all jobs to complete\n", .{});
        self.mutex.lock();
        const pending = self.job_queue.items.len;
        self.mutex.unlock();
        const completed = self.completed_jobs.load(.seq_cst);
        print("[MAIN] All jobs completed ({} done, {} pending)\n", .{completed, pending});
    }
};

test "ThreadPool basic operation" {
    print("\n=== Starting thread pool test ===\n", .{});
    var pool = try ThreadPool.init(std.testing.allocator);
    defer pool.deinit();
    
    const Context = struct {
        value: std.atomic.Value(i32),
        
        fn work(self: *const @This()) void {
            const old = self.value.fetchAdd(1, .seq_cst);
            print("[JOB] Incrementing value from {} to {}\n", .{old, old + 1});
            std.time.sleep(1 * std.time.ns_per_ms);
        }
    };
    
    var ctx = Context{ .value = std.atomic.Value(i32).init(0) };
    print("[TEST] Created context with initial value 0\n", .{});
    
    print("[TEST] Scheduling 10 jobs\n", .{});
    var i: usize = 0;
    while (i < 10) : (i += 1) {
        try pool.schedule(&ctx, Context.work);
        print("[TEST] Scheduled job {}/10\n", .{i + 1});
    }
    
    print("[TEST] All jobs scheduled, waiting for completion\n", .{});
    pool.wait();
    
    const final_value = ctx.value.load(.seq_cst);
    print("[TEST] Final value: {}\n", .{final_value});
    try std.testing.expectEqual(@as(i32, 10), final_value);
    print("[TEST] Test completed successfully\n", .{});
}
