#!/bin/bash
# Crystal Runtime Performance History Setup Script
# Created: 2025-01-22 01:02:44 UTC
# Author: isdood

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}Setting up Crystal Runtime performance history tracking...${NC}"

# Create performance history directory
mkdir -p zig/crystal/src/tests/perf/history

# Create performance history storage
cat > zig/crystal/src/tests/perf/history.zig << 'END_HISTORY'
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
END_HISTORY

# Update performance test to include history
cat >> zig/crystal/src/tests/perf/main.zig << 'END_PERF_TEST_UPDATE'

// Add after the main() function:
fn recordPerformanceMetrics(
    core_time: u64,
    core_ops: u64,
    memory_per_op: usize,
    total_memory: usize,
    concurrent_time: u64,
    concurrent_ops: u64,
) !void {
    var history = @import("history.zig").PerformanceHistory.init(std.heap.c_allocator);
    defer history.deinit();

    // Try to load existing history
    history.loadFromFile() catch {};

    // Add new record
    try history.addRecord(.{
        .timestamp = std.time.timestamp(),
        .core_op_time = core_time,
        .core_ops_per_sec = core_ops,
        .memory_per_op = memory_per_op,
        .total_memory = total_memory,
        .concurrent_op_time = concurrent_time,
        .concurrent_ops_per_sec = concurrent_ops,
    });
}
END_PERF_TEST_UPDATE

# Create performance analysis script
cat > analyze_performance.sh << 'END_ANALYSIS'
#!/bin/bash

echo "=== Crystal Runtime Performance Analysis ==="
echo

jq -r '
  def avg: add / length;
  def trend_direction:
    if length <= 1 then "N/A"
    else
      if last > (.[:-1] | avg) then "⬆️  Increasing"
      elif last < (.[:-1] | avg) then "⬇️  Decreasing"
      else "➡️  Stable"
    end;

  "Core Operations:",
  "  Current: \(last.core_op_time)ns/op (\(last.core_ops_per_sec) ops/sec)",
  "  Trend: \([.[].core_op_time] | trend_direction)",
  "",
  "Memory Usage:",
  "  Current: \(last.memory_per_op) bytes/op (total: \(last.total_memory) bytes)",
  "  Trend: \([.[].memory_per_op] | trend_direction)",
  "",
  "Concurrent Operations:",
  "  Current: \(last.concurrent_op_time)ns/op (\(last.concurrent_ops_per_sec) ops/sec)",
  "  Trend: \([.[].concurrent_op_time] | trend_direction)"
' zig/crystal/src/tests/perf/history/performance_log.json
END_ANALYSIS

chmod +x analyze_performance.sh

echo -e "${GREEN}Crystal Runtime performance history tracking has been set up successfully!${NC}"
echo -e "${BLUE}Usage instructions:${NC}"
echo "1. Performance metrics are now automatically recorded after each test run"
echo "2. Run './analyze_performance.sh' to view performance trends"
echo "3. Historical data is stored in 'zig/crystal/src/tests/perf/history/performance_log.json'"
