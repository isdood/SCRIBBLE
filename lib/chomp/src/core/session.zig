///! Compiler Session Management
///! =========================
///! Author: Caleb J.D. Terkovics <isdood>
///! Last Updated: 2025-01-21 02:43:11 UTC

const std = @import("std");
const Options = @import("options.zig").Options;
const Config = @import("config.zig").Config;
const source = @import("source.zig");

pub const Session = struct {
    allocator: std.mem.Allocator,
    config: Config,
    source_map: source.SourceMap,
    stats: Statistics,

    pub const Statistics = struct {
        parse_time_ns: u64,
        compile_time_ns: u64,
        memory_used: usize,
    };

    pub fn init(allocator: std.mem.Allocator, opts: Options) !Session {
        return Session{
            .allocator = allocator,
            .config = try Config.fromOptions(opts),
            .source_map = try source.SourceMap.init(allocator),
            .stats = .{
                .parse_time_ns = 0,
                .compile_time_ns = 0,
                .memory_used = 0,
            },
        };
    }

    pub fn deinit(self: *Session) void {
        self.source_map.deinit();
    }
};
