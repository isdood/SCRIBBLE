//! timer.zig - Time management for Prism async runtime
//! Created by: isdood
//! Date: 2025-01-21 10:57:47 UTC

const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const AutoHashMap = std.AutoHashMap;
const Time = std.time.Timer;

/// Timer errors
pub const TimerError = error{
    TimerCreationFailed,
    InvalidDuration,
    TimerOverflow,
    AlreadyRunning,
    NotRunning,
};

/// Timer callback type
pub const TimerCallback = fn (*Timer) void;

/// Timer precision modes
pub const Precision = enum {
    High,    // Nanosecond precision
    Medium,  // Microsecond precision
    Low,     // Millisecond precision
};

/// Timer state
pub const State = enum {
    Idle,
    Running,
    Paused,
    Expired,
};

/// Represents a single timer event
const TimerEvent = struct {
    id: u64,
    deadline: u64,
    interval: ?u64,
    callback: ?TimerCallback,
    precision: Precision,
    state: State,

    pub fn init(id: u64, deadline: u64, interval: ?u64, callback: ?TimerCallback, precision: Precision) TimerEvent {
        return .{
            .id = id,
            .deadline = deadline,
            .interval = interval,
            .callback = callback,
            .precision = precision,
            .state = .Idle,
        };
    }
};

/// Main timer management structure
pub const Timer = struct {
    events: ArrayList(TimerEvent),
    system_timer: Time,
    next_id: std.atomic.Value(u64),
    start_time: u64,
    precision: Precision,

    const Self = @This();

    /// Initialize a new timer
    pub fn init() Timer {
        return Timer{
            .events = ArrayList(TimerEvent).init(std.heap.page_allocator),
            .system_timer = Time.start() catch unreachable,
            .next_id = std.atomic.Value(u64).init(0),
            .start_time = 0,
            .precision = .High,
        };
    }

    /// Clean up resources
    pub fn deinit(self: *Self) void {
        self.events.deinit();
    }

    /// Create a one-shot timer
    pub fn setTimeout(self: *Self, duration_ns: u64, callback: ?TimerCallback) !u64 {
        if (duration_ns == 0) {
            return TimerError.InvalidDuration;
        }

        const id = self.next_id.fetchAdd(1, .Monotonic);
        const now = self.system_timer.read();
        const deadline = now + duration_ns;

        const event = TimerEvent.init(
            id,
            deadline,
            null,
            callback,
            self.precision,
        );

        try self.events.append(event);
        return id;
    }

    /// Create a recurring timer
    pub fn setInterval(self: *Self, interval_ns: u64, callback: ?TimerCallback) !u64 {
        if (interval_ns == 0) {
            return TimerError.InvalidDuration;
        }

        const id = self.next_id.fetchAdd(1, .Monotonic);
        const now = self.system_timer.read();
        const deadline = now + interval_ns;

        const event = TimerEvent.init(
            id,
            deadline,
            interval_ns,
            callback,
            self.precision,
        );

        try self.events.append(event);
        return id;
    }

    /// Cancel a timer by ID
    pub fn clearTimer(self: *Self, timer_id: u64) bool {
        for (self.events.items) |event, index| {
            if (event.id == timer_id) {
                _ = self.events.orderedRemove(index);
                return true;
            }
        }
        return false;
    }

    /// Process all due timer events
    pub fn update(self: *Self) !void {
        const now = self.system_timer.read();
        var i: usize = 0;

        while (i < self.events.items.len) {
            var event = &self.events.items[i];

            if (now >= event.deadline) {
                if (event.callback) |callback| {
                    callback(self);
                }

                if (event.interval) |interval| {
                    // Recurring timer - update deadline
                    event.deadline += interval;
                    i += 1;
                } else {
                    // One-shot timer - remove it
                    _ = self.events.orderedRemove(i);
                }
            } else {
                i += 1;
            }
        }
    }

    /// Get remaining time for a timer
    pub fn getRemaining(self: Self, timer_id: u64) ?u64 {
        const now = self.system_timer.read();
        
        for (self.events.items) |event| {
            if (event.id == timer_id) {
                if (event.deadline > now) {
                    return event.deadline - now;
                }
                return 0;
            }
        }
        return null;
    }

    /// Set the timer precision mode
    pub fn setPrecision(self: *Self, precision: Precision) void {
        self.precision = precision;
    }

    /// Get active timer count
    pub fn getActiveCount(self: Self) usize {
        return self.events.items.len;
    }

    /// Reset the timer system
    pub fn reset(self: *Self) void {
        self.events.clearAndFree();
        self.start_time = self.system_timer.read();
    }

    /// Convert time based on precision mode
    fn adjustTime(time: u64, precision: Precision) u64 {
        return switch (precision) {
            .High => time,
            .Medium => time / 1000 * 1000,
            .Low => time / 1000000 * 1000000,
        };
    }

    /// Sleep for specified duration
    pub fn sleep(self: *Self, duration_ns: u64) !void {
        const adjusted_duration = adjustTime(duration_ns, self.precision);
        std.time.sleep(adjusted_duration);
        try self.update();
    }
};

test "timer basic functionality" {
    var timer = Timer.init();
    defer timer.deinit();

    var callback_called = false;
    const TestContext = struct {
        called: *bool,

        fn callback(timer: *Timer) void {
            _ = timer;
            called.* = true;
        }
    };

    const id = try timer.setTimeout(10 * std.time.ns_per_ms, TestContext.callback);
    try std.testing.expect(timer.getActiveCount() == 1);
    
    // Wait for timer to expire
    try timer.sleep(20 * std.time.ns_per_ms);
    
    try std.testing.expect(callback_called);
    try std.testing.expect(timer.getActiveCount() == 0);
    try std.testing.expect(timer.getRemaining(id) == null);
}

test "interval timer" {
    var timer = Timer.init();
    defer timer.deinit();

    var count: usize = 0;
    const TestContext = struct {
        count: *usize,

        fn callback(timer: *Timer) void {
            _ = timer;
            count.* += 1;
        }
    };

    const interval = 10 * std.time.ns_per_ms;
    const id = try timer.setInterval(interval, TestContext.callback);
    
    // Wait for multiple intervals
    try timer.sleep(25 * std.time.ns_per_ms);
    
    try std.testing.expect(count >= 2);
    try std.testing.expect(timer.getRemaining(id) != null);
    
    // Clear the interval
    try std.testing.expect(timer.clearTimer(id));
    try std.testing.expect(timer.getActiveCount() == 0);
}

test "timer precision modes" {
    var timer = Timer.init();
    defer timer.deinit();

    // Test different precision modes
    timer.setPrecision(.Low);
    const time_low = adjustTime(1234567, .Low);
    try std.testing.expect(time_low == 1000000);

    timer.setPrecision(.Medium);
    const time_medium = adjustTime(1234567, .Medium);
    try std.testing.expect(time_medium == 1234000);

    timer.setPrecision(.High);
    const time_high = adjustTime(1234567, .High);
    try std.testing.expect(time_high == 1234567);
}
