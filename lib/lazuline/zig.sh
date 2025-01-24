cat > ../lazuline/src/lib.zig << 'EOF'
const std = @import("std");
const print = std.debug.print;
const builtin = @import("builtin");
const Mutex = std.Thread.Mutex;

pub const WorkFn = *const fn (*anyopaque) void;

pub const Job = struct {
    context: *anyopaque,
    work_fn: WorkFn,
};

pub const ThreadPool = struct {
    const Self = @This();
    
    allocator: std.mem.Allocator,
    thread: std.Thread,
    job_mutex: Mutex,
    current_job: ?*align(16) Job,
    job_ready: std.atomic.Value(bool),
    running: std.atomic.Value(bool),
    completed: std.atomic.Value(usize),
    
    pub fn init(allocator: std.mem.Allocator) !Self {
        print("\n[{s}] Creating thread pool (Target: {s})\n", .{
            @typeName(Self),
            @tagName(builtin.cpu.arch)
        });
        
        var pool = Self{
            .allocator = allocator,
            .thread = undefined,
            .job_mutex = .{},
            .current_job = null,
            .job_ready = std.atomic.Value(bool).init(false),
            .running = std.atomic.Value(bool).init(true),
            .completed = std.atomic.Value(usize).init(0),
        };
        
        print("[MAIN] Starting worker thread\n", .{});
        pool.thread = try std.Thread.spawn(.{}, workerFn, .{&pool});
        std.time.sleep(10 * std.time.ns_per_ms);
        print("[MAIN] Worker thread started\n", .{});
        
        return pool;
    }
    
    fn workerFn(pool: *Self) void {
        print("[WORKER] Thread starting (ID: {})\n", .{std.Thread.getCurrentId()});
        var jobs_seen: usize = 0;
        
        while (pool.running.load(.acquire)) {
            if (pool.job_ready.load(.acquire)) {
                pool.job_mutex.lock();
                if (pool.current_job) |job| {
                    jobs_seen += 1;
                    print("[WORKER] Processing job {} at {*}\n", .{jobs_seen, job});
                    
                    // Take ownership of the job
                    pool.current_job = null;
                    pool.job_ready.store(false, .release);
                    pool.job_mutex.unlock();
                    
                    // Process job
                    job.work_fn(job.context);
                    
                    // Clean up
                    const total = pool.completed.fetchAdd(1, .release) + 1;
                    print("[WORKER] Completed job {} (total: {})\n", .{jobs_seen, total});
                } else {
                    pool.job_ready.store(false, .release);
                    pool.job_mutex.unlock();
                }
            } else {
                std.time.sleep(1 * std.time.ns_per_ms);
            }
        }
        
        print("[WORKER] Thread shutting down (processed {} jobs)\n", .{jobs_seen});
    }
    
    pub fn deinit(self: *Self) void {
        print("[MAIN] Shutting down thread pool\n", .{});
        self.running.store(false, .release);
        self.thread.join();
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
        
        // Wait for any previous job to be taken
        print("[MAIN] Waiting for worker to be ready...\n", .{});
        while (self.job_ready.load(.acquire)) {
            std.time.sleep(1 * std.time.ns_per_ms);
        }
        
        // Create aligned job
        const job = try self.allocator.alignedAlloc(Job, 16, 1);
        job[0] = Job{
            .context = @constCast(context),
            .work_fn = Wrapper.call,
        };
        
        // Schedule job
        self.job_mutex.lock();
        print("[MAIN] Scheduling job at {*}\n", .{job});
        self.current_job = &job[0];
        self.job_ready.store(true, .release);
        self.job_mutex.unlock();
        
        // Wait for job to be taken
        print("[MAIN] Waiting for worker to take job...\n", .{});
        while (self.job_ready.load(.acquire)) {
            std.time.sleep(1 * std.time.ns_per_ms);
        }
        
        // Clean up
        self.allocator.free(job);
        print("[MAIN] Job taken by worker\n", .{});
    }
    
    pub fn wait(self: *Self) void {
        const total = self.completed.load(.acquire);
        print("[MAIN] Waiting for {} jobs to complete\n", .{total});
        
        while (true) {
            const completed = self.completed.load(.acquire);
            if (completed >= total) break;
            
            print("[MAIN] {}/{} jobs completed\n", .{completed, total});
            std.time.sleep(10 * std.time.ns_per_ms);
        }
        
        print("[MAIN] All {} jobs completed\n", .{self.completed.load(.acquire)});
    }
};

test "ThreadPool basic operation" {
    print("\n=== Starting thread pool test ===\n", .{});
    var pool = try ThreadPool.init(std.testing.allocator);
    defer pool.deinit();
    
    const Context = struct {
        value: std.atomic.Value(i32),
        
        fn work(self: *const @This()) void {
            const old = self.value.fetchAdd(1, .acq_rel);
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
    
    const final_value = ctx.value.load(.acquire);
    print("[TEST] Final value: {}\n", .{final_value});
    try std.testing.expectEqual(@as(i32, 10), final_value);
    print("[TEST] Test completed successfully\n", .{});
}
EOF
