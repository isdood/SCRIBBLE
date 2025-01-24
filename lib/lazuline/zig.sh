cat > ../lazuline/src/lib.zig << 'EOF'
const std = @import("std");
const debug = std.debug;

pub const WorkFn = *const fn (*anyopaque) void;

pub const Job = struct {
    context: *anyopaque,
    work_fn: WorkFn,
};

pub const ThreadPool = struct {
    allocator: std.mem.Allocator,
    threads: []std.Thread,
    mutex: std.Thread.Mutex,
    jobs: std.ArrayList(Job),
    shutdown: std.atomic.Value(bool),
    active_jobs: std.atomic.Value(usize),
    total_jobs_processed: std.atomic.Value(usize),
    condition: std.Thread.Condition,
    thread_count: usize,
    initialized_threads: std.atomic.Value(usize),

    const Self = @This();
    
    pub fn init(allocator: std.mem.Allocator) !Self {
        const thread_count = try std.Thread.getCpuCount();
        debug.print("\nThread Pool: Initializing with {d} threads\n", .{thread_count});
        
        const threads = try allocator.alloc(std.Thread, thread_count);
        errdefer allocator.free(threads);
        
        var pool = Self{
            .allocator = allocator,
            .threads = threads,
            .mutex = .{},
            .jobs = std.ArrayList(Job).init(allocator),
            .shutdown = std.atomic.Value(bool).init(false),
            .active_jobs = std.atomic.Value(usize).init(0),
            .total_jobs_processed = std.atomic.Value(usize).init(0),
            .condition = .{},
            .thread_count = thread_count,
            .initialized_threads = std.atomic.Value(usize).init(0),
        };

        // Create all threads before returning
        var i: usize = 0;
        while (i < thread_count) : (i += 1) {
            threads[i] = try std.Thread.spawn(.{}, worker, .{&pool});
            _ = pool.initialized_threads.fetchAdd(1, .release);
            debug.print("Thread Pool: Started worker thread {d}\n", .{i});
        }

        // Wait for all threads to start
        var retry: usize = 0;
        while (pool.initialized_threads.load(.acquire) < thread_count) {
            std.time.sleep(1 * std.time.ns_per_ms);
            retry += 1;
            if (retry > 1000) {
                return error.ThreadInitTimeout;
            }
        }

        debug.print("Thread Pool: All threads initialized\n", .{});
        return pool;
    }

    pub fn deinit(self: *Self) void {
        debug.print("\nThread Pool: Initiating shutdown\n", .{});
        
        self.shutdown.store(true, .release);
        self.mutex.lock();
        self.condition.broadcast();
        self.mutex.unlock();
        
        const init_threads = self.initialized_threads.load(.acquire);
        for (self.threads[0..init_threads], 0..) |thread, i| {
            debug.print("Thread Pool: Joining thread {d}\n", .{i});
            thread.join();
        }
        
        const remaining = self.jobs.items.len;
        const completed = self.total_jobs_processed.load(.acquire);
        debug.print("Thread Pool: Shutdown complete. Processed {d} jobs, {d} remaining\n", 
            .{completed, remaining});
        
        self.jobs.deinit();
        self.allocator.free(self.threads);
    }

    fn worker(pool: *Self) void {
        const thread_num = pool.initialized_threads.load(.monotonic);
        debug.print("Thread Pool[{d}]: Worker ready\n", .{thread_num});
        
        while (!pool.shutdown.load(.acquire)) {
            // Try to get a job
            pool.mutex.lock();
            
            // Wait for work while holding the mutex
            while (!pool.shutdown.load(.acquire) and pool.jobs.items.len == 0) {
                debug.print("Thread Pool[{d}]: Waiting for work\n", .{thread_num});
                pool.condition.wait(&pool.mutex);
            }
            
            // Check for shutdown after wakeup
            if (pool.shutdown.load(.acquire)) {
                pool.mutex.unlock();
                break;
            }
            
            // Get a job if available
            const job: ?Job = if (pool.jobs.items.len > 0) blk: {
                _ = pool.active_jobs.fetchAdd(1, .release);
                debug.print("Thread Pool[{d}]: Got job ({d} in queue)\n", 
                    .{thread_num, pool.jobs.items.len - 1});
                break :blk pool.jobs.orderedRemove(0);
            } else null;
            
            pool.mutex.unlock();
            
            // Process job if we got one
            if (job) |j| {
                debug.print("Thread Pool[{d}]: Executing job\n", .{thread_num});
                j.work_fn(j.context);
                _ = pool.total_jobs_processed.fetchAdd(1, .release);
                _ = pool.active_jobs.fetchSub(1, .release);
                debug.print("Thread Pool[{d}]: Job completed\n", .{thread_num});
            }
        }
        
        debug.print("Thread Pool[{d}]: Shutting down\n", .{thread_num});
    }

    pub fn schedule(self: *Self, context: anytype, comptime work_fn: fn (*const std.meta.Child(@TypeOf(context))) void) !void {
        const PtrType = *const std.meta.Child(@TypeOf(context));
        
        // Create work function wrapper
        const WorkerFn = struct {
            fn call(ptr: *anyopaque) void {
                const typed_ptr = @as(PtrType, @ptrCast(@alignCast(ptr)));
                work_fn(typed_ptr);
            }
        };

        self.mutex.lock();
        defer self.mutex.unlock();
        
        if (self.shutdown.load(.acquire)) {
            return error.ThreadPoolShutdown;
        }

        try self.jobs.append(Job{
            .context = @constCast(context),
            .work_fn = WorkerFn.call,
        });
        
        debug.print("Thread Pool: Scheduled job (queue: {d})\n", .{self.jobs.items.len});
        self.condition.signal();
    }

    pub fn wait(self: *Self) void {
        debug.print("\nThread Pool: Waiting for completion\n", .{});
        var stall_count: usize = 0;
        
        while (true) {
            const jobs_len = blk: {
                self.mutex.lock();
                defer self.mutex.unlock();
                break :blk self.jobs.items.len;
            };
            
            const active = self.active_jobs.load(.acquire);
            const completed = self.total_jobs_processed.load(.acquire);
            
            if (jobs_len == 0 and active == 0) {
                debug.print("Thread Pool: All jobs completed ({d} total)\n", .{completed});
                break;
            }
            
            stall_count += 1;
            if (stall_count > 50) {
                debug.print("Thread Pool: Possible stall, waking workers (jobs: {d}, active: {d})\n", 
                    .{jobs_len, active});
                self.mutex.lock();
                self.condition.broadcast();
                self.mutex.unlock();
                stall_count = 0;
            }
            
            debug.print("Thread Pool: Queue: {d}, Active: {d}, Completed: {d}, Stall: {d}\n", 
                .{jobs_len, active, completed, stall_count});
            std.time.sleep(10 * std.time.ns_per_ms);
        }
    }
};
EOF
