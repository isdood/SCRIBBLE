const std = @import("std");

pub const CrystalChannel = struct {
    const Self = @This();

    pub const Config = struct {
        buffer_size: usize = 64,
        resonance_frequency: f64 = 440.0, // A440 resonance
        damping_factor: f64 = 0.01,
    };

    const Node = struct {
        data: []u8,
        energy: f64,
        next: ?*Node,
    };

    mutex: std.Thread.Mutex,
    not_empty: std.Thread.Condition,
    not_full: std.Thread.Condition,
    head: ?*Node,
    tail: ?*Node,
    len: usize,
    capacity: usize,
    allocator: std.mem.Allocator,
    resonance: f64,
    damping: f64,

    pub fn init(allocator: std.mem.Allocator, config: Config) Self {
        return .{
            .mutex = std.Thread.Mutex{},
            .not_empty = std.Thread.Condition{},
            .not_full = std.Thread.Condition{},
            .head = null,
            .tail = null,
            .len = 0,
            .capacity = config.buffer_size,
            .allocator = allocator,
            .resonance = config.resonance_frequency,
            .damping = config.damping_factor,
        };
    }

    pub fn deinit(self: *Self) void {
        var current = self.head;
        while (current) |node| {
            const next = node.next;
            self.allocator.free(node.data);
            self.allocator.destroy(node);
            current = next;
        }
    }

    pub fn send(self: *Self, data: []const u8) !void {
        const node = try self.allocator.create(Node);
        node.data = try self.allocator.alloc(u8, data.len);
        @memcpy(node.data, data);
        node.energy = self.resonance;
        node.next = null;

        self.mutex.lock();
        defer self.mutex.unlock();

        while (self.len >= self.capacity) {
            self.not_full.wait(&self.mutex);
        }

        if (self.tail) |tail| {
            tail.next = node;
            self.tail = node;
        } else {
            self.head = node;
            self.tail = node;
        }

        self.len += 1;
        self.not_empty.signal();
    }

    pub fn receive(self: *Self) ![]u8 {
        self.mutex.lock();
        defer self.mutex.unlock();

        while (self.len == 0) {
            self.not_empty.wait(&self.mutex);
        }

        const node = self.head.?;
        const data = node.data;

        self.head = node.next;
        if (self.head == null) {
            self.tail = null;
        }

        self.len -= 1;
        self.allocator.destroy(node);
        self.not_full.signal();

        return data;
    }
};
