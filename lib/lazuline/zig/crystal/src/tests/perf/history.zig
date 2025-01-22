const std = @import("std");

pub const PerformanceRecord = struct {
    timestamp: i64,
    core_op_time: u64,
    core_ops_per_sec: u64,
    memory_per_op: usize,
    total_memory: usize,
    concurrent_op_time: u64,
    concurrent_ops_per_sec: u64,
};

pub const PerformanceHistory = struct {
    const Self = @This();

    records: std.ArrayList(PerformanceRecord),
    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator) Self {
        return Self{
            .records = std.ArrayList(PerformanceRecord).init(allocator),
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *Self) void {
        self.records.deinit();
    }

    pub fn addRecord(self: *Self, record: PerformanceRecord) !void {
        try self.records.append(record);
        try self.saveToFile();
    }

    pub fn saveToFile(self: *Self) !void {
        const file = try std.fs.cwd().createFile(
            "zig/crystal/src/tests/perf/history/performance_log.json",
            .{ .truncate = true },
        );
        defer file.close();

        const writer = file.writer();
        try writer.writeAll("[\n");

        for (self.records.items, 0..) |record, i| {
            try writer.print(
                "  {{\"timestamp\": {d}, \"core_op_time\": {d}, \"core_ops_per_sec\": {d}, " ++
                "\"memory_per_op\": {d}, \"total_memory\": {d}, " ++
                "\"concurrent_op_time\": {d}, \"concurrent_ops_per_sec\": {d}}}{s}\n",
                .{
                    record.timestamp,
                    record.core_op_time,
                    record.core_ops_per_sec,
                    record.memory_per_op,
                    record.total_memory,
                    record.concurrent_op_time,
                    record.concurrent_ops_per_sec,
                    if (i < self.records.items.len - 1) "," else "",
                },
            );
        }

        try writer.writeAll("]\n");
    }

    pub fn loadFromFile(self: *Self) !void {
        const file = try std.fs.cwd().openFile(
            "zig/crystal/src/tests/perf/history/performance_log.json",
            .{},
        );
        defer file.close();

        const content = try file.readToEndAlloc(self.allocator, 1024 * 1024);
        defer self.allocator.free(content);

        var parser = std.json.Parser.init(self.allocator, false);
        defer parser.deinit();

        var tree = try parser.parse(content);
        defer tree.deinit();

        const records = tree.root.Array;
        for (records.items) |record_value| {
            const record = PerformanceRecord{
                .timestamp = record_value.Object.get("timestamp").?.Integer,
                .core_op_time = @intCast(record_value.Object.get("core_op_time").?.Integer),
                .core_ops_per_sec = @intCast(record_value.Object.get("core_ops_per_sec").?.Integer),
                .memory_per_op = @intCast(record_value.Object.get("memory_per_op").?.Integer),
                .total_memory = @intCast(record_value.Object.get("total_memory").?.Integer),
                .concurrent_op_time = @intCast(record_value.Object.get("concurrent_op_time").?.Integer),
                .concurrent_ops_per_sec = @intCast(record_value.Object.get("concurrent_ops_per_sec").?.Integer),
            };
            try self.records.append(record);
        }
    }
};
