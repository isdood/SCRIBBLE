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
