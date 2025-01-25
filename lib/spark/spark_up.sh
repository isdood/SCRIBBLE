#!/bin/bash

# spark_up.sh - Spark Language Setup Script
# Created: 2025-01-25 11:00:58
# Author: isdood

mkdir -p tools/seed
cat > "tools/seed/seed.zig" << 'ZIGCODE'
const std = @import("std");

pub fn main() !void {
    // Get arguments
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    if (args.len < 3) return;

    // Create shell wrapper command that uses eval to preserve arguments exactly
    var process_args = std.ArrayList([]const u8).init(allocator);
    defer process_args.deinit();

    // Build a command that uses eval to preserve the arguments exactly
    const cmd = try std.fmt.allocPrint(
        allocator,
        "eval 'tools/seed/zig-out/bin/seed plant \"{s}\"'",
        .{args[2]}
    );
    defer allocator.free(cmd);

    try process_args.appendSlice(&[_][]const u8{
        "/bin/sh",
        "-c",
        cmd,
    });

    // Execute through shell
    var child = std.process.Child.init(process_args.items, allocator);
    _ = try child.spawnAndWait();
}
ZIGCODE

# Compile the Zig wrapper
cd tools/seed && zig build-exe seed.zig -O ReleaseSafe -target native-linux-gnu && cd ../../

# Make it executable
chmod +x tools/seed/zig-out/bin/seed

# Only create symlink if compilation succeeded
if [ -f "tools/seed/zig-out/bin/seed" ]; then
    ln -sf zig-out/bin/seed tools/seed/seed
    chmod +x tools/seed/seed
    echo "✨ Seed manager compiled successfully!"
else
    echo "⚠️  Seed manager compilation failed"
fi
