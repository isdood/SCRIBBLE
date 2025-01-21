///! Compiler Options
///! ===============
///! Author: Caleb J.D. Terkovics <isdood>
///! Last Updated: 2025-01-21 02:43:11 UTC

const std = @import("std");
const Config = @import("config.zig").Config;

pub const Options = struct {
    input_files: std.ArrayList([]const u8),
    output_file: ?[]const u8,
    opt_level: Config.OptLevel,
    target: ?Config.Target,

    pub fn init(allocator: std.mem.Allocator) Options {
        return .{
            .input_files = std.ArrayList([]const u8).init(allocator),
            .output_file = null,
            .opt_level = .Debug,
            .target = null,
        };
    }

    pub fn deinit(self: *Options) void {
        self.input_files.deinit();
    }
};
