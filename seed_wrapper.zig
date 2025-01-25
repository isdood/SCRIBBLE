const std = @import("std");

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    if (args.len < 2) {
        std.debug.print("Usage: {s} <command>\n", .{args[0]});
        std.process.exit(1);
    }

    // Create new args array
    var new_args = try allocator.alloc([]const u8, args.len);
    defer allocator.free(new_args);

    // First arg is always the original script
    new_args[0] = "./bin/seed.orig";

    // Copy remaining args
    var arg_idx: usize = 1;
    while (arg_idx < args.len) : (arg_idx += 1) {
        new_args[arg_idx] = args[arg_idx];
    }

    var proc = std.process.Child.init(new_args, allocator);
    proc.stdout_behavior = .Pipe;
    proc.stderr_behavior = .Inherit;
    proc.stdin_behavior = .Inherit;

    try proc.spawn();

    const stdout = proc.stdout.?;
    const writer = std.io.getStdOut().writer();

    var buffer: [4096]u8 = undefined;
    var out_buffer: [4096]u8 = undefined;
    var out_index: usize = 0;

    while (true) {
        const bytes_read = try stdout.read(&buffer);
        if (bytes_read == 0) break;

        var byte_idx: usize = 0;
        while (byte_idx < bytes_read) {
            if (byte_idx + 1 < bytes_read and buffer[byte_idx] == '_' and buffer[byte_idx + 1] == '_') {
                out_buffer[out_index] = '*';
                out_buffer[out_index + 1] = '*';
                out_index += 2;
                byte_idx += 2;
            } else {
                out_buffer[out_index] = buffer[byte_idx];
                out_index += 1;
                byte_idx += 1;
            }
        }

        if (out_index > 0) {
            try writer.writeAll(out_buffer[0..out_index]);
            out_index = 0;
        }
    }

    const term = try proc.wait();
    switch (term) {
        .Exited => |code| std.process.exit(code),
        else => std.process.exit(1),
    }
}
