const std = @import("std");
const os = std.os;
const fs = std.fs;

pub fn main() !void {
    // Get current executable path
    var buf: [fs.MAX_PATH_BYTES]u8 = undefined;
    const self_path = try fs.selfExePath(&buf);
    const dir = fs.path.dirname(self_path) orelse return error.NoPath;
    
    // Build path to real binary
    var path_buf: [fs.MAX_PATH_BYTES]u8 = undefined;
    const real_path = try std.fmt.bufPrint(&path_buf, "{s}/zig-out/bin/seed", .{dir});
    
    // Execute the real binary, bypassing shell interpretation
    const err = os.linux.syscall3(
        .execve,
        @intFromPtr(real_path.ptr),
        @intFromPtr(os.argv.ptr),
        @intFromPtr(@as(?[*]const ?[*:0]u8, null))
    );
    
    switch (err) {
        0 => {},
        else => return error.ExecveFailed,
    }
}
