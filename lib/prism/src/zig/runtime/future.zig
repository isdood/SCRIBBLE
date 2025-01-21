//! future.zig - Future implementation for Prism async runtime
//! Created by: isdood
//! Date: 2025-01-21 10:56:55 UTC

const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const AutoHashMap = std.AutoHashMap;

/// Future errors
pub const FutureError = error{
    AlreadyCompleted,
    NotCompleted,
    CancellationError,
    TimeoutError,
    ChainError,
};

/// Future state
pub const State = enum {
    Pending,
    Running,
    Completed,
    Cancelled,
    Failed,
};

/// Future interface - must be implemented by all future types
pub const Future = struct {
    /// Virtual table for future operations
    const VTable = struct {
        execute: fn (*Future) anyerror!void,
        cancel: fn (*Future) void,
        cleanup: fn (*Future) void,
    };

    /// Future state and metadata
    state: State,
    vtable: *const VTable,
    result: ?*anyopaque,
    error: ?anyerror,
    allocator: ?Allocator,
    children: ArrayList(*Future),
    parent: ?*Future,
    timeout_ns: ?u64,

    const Self = @This();

    /// Initialize a new future
    pub fn init(vtable: *const VTable, allocator: ?Allocator) Self {
        return .{
            .state = .Pending,
            .vtable = vtable,
            .result = null,
            .error = null,
            .allocator = allocator,
            .children = if (allocator) |alloc| 
                ArrayList(*Future).init(alloc) 
            else 
                ArrayList(*Future).init(std.heap.page_allocator),
            .parent = null,
            .timeout_ns = null,
        };
    }

    /// Clean up resources
    pub fn deinit(self: *Self) void {
        self.vtable.cleanup(self);
        for (self.children.items) |child| {
            child.deinit();
        }
        self.children.deinit();
        if (self.result) |result| {
            if (self.allocator) |allocator| {
                allocator.destroy(result);
            }
        }
    }

    /// Execute the future
    pub fn execute(self: *Self) !void {
        if (self.state == .Completed) {
            return FutureError.AlreadyCompleted;
        }

        self.state = .Running;

        // Handle timeout if set
        if (self.timeout_ns) |timeout| {
            const timer = try std.time.Timer.start();
            const result = self.vtable.execute(self);
            
            if (timer.read() > timeout) {
                self.state = .Failed;
                self.error = FutureError.TimeoutError;
                return FutureError.TimeoutError;
            }
            
            try result;
        } else {
            try self.vtable.execute(self);
        }

        if (self.state != .Cancelled) {
            self.state = .Completed;
        }

        // Execute children if any
        for (self.children.items) |child| {
            try child.execute();
        }
    }

    /// Cancel the future
    pub fn cancel(self: *Self) void {
        if (self.state != .Completed) {
            self.vtable.cancel(self);
            self.state = .Cancelled;
            
            // Cancel all children
            for (self.children.items) |child| {
                child.cancel();
            }
        }
    }

    /// Chain another future to execute after this one
    pub fn then(self: *Self, next: *Future) !void {
        if (self.state == .Cancelled) {
            return FutureError.CancellationError;
        }
        
        try self.children.append(next);
        next.parent = self;
    }

    /// Set a timeout for future execution
    pub fn setTimeout(self: *Self, nanoseconds: u64) void {
        self.timeout_ns = nanoseconds;
    }

    /// Get the current state
    pub fn getState(self: Self) State {
        return self.state;
    }

    /// Get the result if completed
    pub fn getResult(self: Self, comptime T: type) !T {
        if (self.state != .Completed) {
            return FutureError.NotCompleted;
        }
        if (self.error) |err| {
            return err;
        }
        if (self.result) |result| {
            return @ptrCast(*T, @alignCast(@alignOf(T), result)).*;
        }
        unreachable;
    }

    /// Set the result
    pub fn setResult(self: *Self, result: anytype) !void {
        if (self.state == .Completed) {
            return FutureError.AlreadyCompleted;
        }
        
        const T = @TypeOf(result);
        if (self.allocator) |allocator| {
            const ptr = try allocator.create(T);
            ptr.* = result;
            self.result = ptr;
        }
    }
};

/// A simple value future implementation
pub fn ValueFuture(comptime T: type) type {
    return struct {
        future: Future,
        value: T,

        const Self = @This();

        /// VTable for ValueFuture
        const vtable = Future.VTable{
            .execute = execute,
            .cancel = cancel,
            .cleanup = cleanup,
        };

        /// Initialize a new value future
        pub fn init(allocator: ?Allocator, value: T) Self {
            return .{
                .future = Future.init(&vtable, allocator),
                .value = value,
            };
        }

        /// Execute implementation
        fn execute(base: *Future) anyerror!void {
            const self = @fieldParentPtr(Self, "future", base);
            try self.future.setResult(self.value);
        }

        /// Cancel implementation
        fn cancel(base: *Future) void {
            const self = @fieldParentPtr(Self, "future", base);
            _ = self;
        }

        /// Cleanup implementation
        fn cleanup(base: *Future) void {
            const self = @fieldParentPtr(Self, "future", base);
            _ = self;
        }
    };
}

test "value future basic functionality" {
    const allocator = std.testing.allocator;
    
    var value_future = ValueFuture(i32).init(allocator, 42);
    defer value_future.future.deinit();

    try value_future.future.execute();
    try std.testing.expectEqual(@as(i32, 42), try value_future.future.getResult(i32));
}

test "future chaining" {
    const allocator = std.testing.allocator;
    
    var future1 = ValueFuture(i32).init(allocator, 42);
    defer future1.future.deinit();
    
    var future2 = ValueFuture(i32).init(allocator, 84);
    defer future2.future.deinit();

    try future1.future.then(&future2.future);
    try future1.future.execute();

    try std.testing.expectEqual(future2.future.getState(), .Completed);
}

test "future timeout" {
    const allocator = std.testing.allocator;
    
    // Create a future that sleeps
    const SleepFuture = struct {
        future: Future,
        duration_ns: u64,

        const Self = @This();

        const vtable = Future.VTable{
            .execute = execute,
            .cancel = cancel,
            .cleanup = cleanup,
        };

        pub fn init(allocator: ?Allocator, duration_ns: u64) Self {
            return .{
                .future = Future.init(&vtable, allocator),
                .duration_ns = duration_ns,
            };
        }

        fn execute(base: *Future) anyerror!void {
            const self = @fieldParentPtr(Self, "future", base);
            std.time.sleep(self.duration_ns);
        }

        fn cancel(base: *Future) void {
            const self = @fieldParentPtr(Self, "future", base);
            _ = self;
        }

        fn cleanup(base: *Future) void {
            const self = @fieldParentPtr(Self, "future", base);
            _ = self;
        }
    };

    var sleep_future = SleepFuture.init(allocator, std.time.ns_per_s);
    defer sleep_future.future.deinit();

    sleep_future.future.setTimeout(std.time.ns_per_ms * 10);
    try std.testing.expectError(FutureError.TimeoutError, sleep_future.future.execute());
}
