cat > ../lazuline/src/lib.zig << 'EOF'
const std = @import("std");

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

    const Self = @This();
    
    pub fn init(allocator: std.mem.Allocator) !Self {
        const thread_count = try std.Thread.getCpuCount();
        const threads = try allocator.alloc(std.Thread, thread_count);
        
        var pool = Self{
            .allocator = allocator,
            .threads = threads,
            .mutex = .{},
            .jobs = std.ArrayList(Job).init(allocator),
            .shutdown = std.atomic.Value(bool).init(false),
        };

        for (threads) |*thread| {
            thread.* = try std.Thread.spawn(.{}, worker, .{&pool});
        }

        return pool;
    }

    pub fn deinit(self: *Self) void {
        self.shutdown.store(true, .seq_cst);
        for (self.threads) |thread| {
            thread.join();
        }
        self.jobs.deinit();
        self.allocator.free(self.threads);
    }

    fn worker(pool: *Self) void {
        while (!pool.shutdown.load(.seq_cst)) {
            pool.mutex.lock();
            const job = if (pool.jobs.items.len > 0) pool.jobs.orderedRemove(0) else null;
            pool.mutex.unlock();

            if (job) |j| {
                j.work_fn(j.context);
            } else {
                std.time.sleep(1 * std.time.ns_per_ms);
            }
        }
    }

    pub fn schedule(self: *Self, context: anytype, comptime work_fn: fn (*const std.meta.Child(@TypeOf(context))) void) !void {
        const PtrType = *const std.meta.Child(@TypeOf(context));
        
        const CastedFn = struct {
            fn cast(raw_ptr: *anyopaque) void {
                const ptr = @as(PtrType, @ptrCast(@alignCast(raw_ptr)));
                work_fn(ptr);
            }
        };

        const job = Job{
            .context = @constCast(context),
            .work_fn = CastedFn.cast,
        };

        self.mutex.lock();
        defer self.mutex.unlock();
        try self.jobs.append(job);
    }

    pub fn wait(self: *Self) void {
        while (true) {
            self.mutex.lock();
            const is_empty = self.jobs.items.len == 0;
            self.mutex.unlock();
            
            if (is_empty) break;
            std.time.sleep(1 * std.time.ns_per_ms);
        }
    }
};
EOF
