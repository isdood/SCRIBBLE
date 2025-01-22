const std = @import("std");

pub fn build(b: *std.Build) *std.Build.Module {
    const module = b.addModule("crystal", .{
        .source_file = .{ .cwd_relative = "src/main.zig" },
    });
    return module;
}
