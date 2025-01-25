cat > ../lazuline/src/lib.zig << 'EOF'
const std = @import("std");
const assert = std.debug.assert;
const print = std.debug.print;
const Thread = std.Thread;
const Atomic = std.atomic.Value;

pub const WorkFn = *const fn (*anyopaque) void;

pub const Job = struct {
    context: *anyopaque,
    work_fn: WorkFn,
};

pub const ThreadPool = struct {
    const Self = @This();
    
    thread: Thread,
    job_queue: std.ArrayList(Job),
    job_mutex: Thread.Mutex,
    job_available: Thread.Condition,
    allocator: std.mem.Allocator,
    running: bool,
    
    pub fn init(allocator: std.mem.Allocator) !Self {
        print("[POOL] Init\n", .{});
        
        var pool = Self{
            .thread = undefined,
            .job_queue = std.ArrayList(Job).init(allocator),
            .job_mutex = .{},
            .job_available = .{},
            .allocator = allocator,
            .running = true,
        };
        
        try pool.job_queue.ensureTotalCapacity(100);
        
        print("[POOL] Starting worker\n", .{});
        pool.thread = try Thread.spawn(.{}, workerFn, .{&pool});
        
        return pool;
    }
    
    fn workerFn(pool: *Self) void {
        print("[WORKER] Starting\n", .{});
        
        while (true) {
            pool.job_mutex.lock();
            
            while (pool.job_queue.items.len == 0 and pool.running) {
                pool.job_available.wait(&pool.job_mutex);
            }
            
            if (!pool.running and pool.job_queue.items.len == 0) {
                pool.job_mutex.unlock();
                break;
            }
            
            const job = pool.job_queue.orderedRemove(0);
            print("[WORKER] Got job\n", .{});
            
            pool.job_mutex.unlock();
            
            job.work_fn(job.context);
            print("[WORKER] Finished job\n", .{});
        }
        
        print("[WORKER] Exiting\n", .{});
    }
    
    pub fn deinit(self: *Self) void {
        print("[POOL] Shutdown\n", .{});
        
        self.job_mutex.lock();
        self.running = false;
        self.job_available.signal();
        self.job_mutex.unlock();
        
        self.thread.join();
        self.job_queue.deinit();
    }
    
    pub fn schedule(self: *Self, context: anytype, comptime work_fn: fn (*const std.meta.Child(@TypeOf(context))) void) !void {
        print("[POOL] Schedule job\n", .{});
        
        const PtrType = *const std.meta.Child(@TypeOf(context));
        
        const Wrapper = struct {
            fn call(ptr: *anyopaque) void {
                const typed_ptr = @as(PtrType, @ptrCast(@alignCast(ptr)));
                work_fn(typed_ptr);
            }
        };
        
        const job = Job{
            .context = @constCast(context),
            .work_fn = Wrapper.call,
        };
        
        self.job_mutex.lock();
        defer self.job_mutex.unlock();
        
        if (!self.running) return error.ThreadPoolShutDown;
        
        try self.job_queue.append(job);
        self.job_available.signal();
    }
    
    pub fn wait(self: *Self) void {
        print("[POOL] Wait\n", .{});
        
        while (true) {
            self.job_mutex.lock();
            const done = (self.job_queue.items.len == 0);
            self.job_mutex.unlock();
            
            if (done) break;
            
            std.time.sleep(10 * std.time.ns_per_ms);
        }
        
        print("[POOL] Done waiting\n", .{});
    }
};
EOF
