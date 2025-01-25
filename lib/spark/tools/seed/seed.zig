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
