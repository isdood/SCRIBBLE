///! Source Code Management
///! ====================
///! Author: Caleb J.D. Terkovics <isdood>
///! Last Updated: 2025-01-21 02:43:11 UTC

const std = @import("std");

pub const Span = struct {
    start: usize,
    end: usize,
    file_id: u32,
};

pub const SourceFile = struct {
    path: []const u8,
    content: []const u8,
    line_starts: std.ArrayList(usize),

    pub fn init(allocator: std.mem.Allocator, path: []const u8, content: []const u8) !SourceFile {
        var self = SourceFile{
            .path = path,
            .content = content,
            .line_starts = std.ArrayList(usize).init(allocator),
        };
        try self.computeLineStarts();
        return self;
    }

    fn computeLineStarts(self: *SourceFile) !void {
        try self.line_starts.append(0);
        for (self.content, 0..) |c, i| {
            if (c == '\n') {
                try self.line_starts.append(i + 1);
            }
        }
    }

    pub fn deinit(self: *SourceFile) void {
        self.line_starts.deinit();
    }
};

pub const SourceMap = struct {
    allocator: std.mem.Allocator,
    files: std.ArrayList(SourceFile),

    pub fn init(allocator: std.mem.Allocator) !SourceMap {
        return SourceMap{
            .allocator = allocator,
            .files = std.ArrayList(SourceFile).init(allocator),
        };
    }

    pub fn deinit(self: *SourceMap) void {
        for (self.files.items) |*file| {
            file.deinit();
        }
        self.files.deinit();
    }

    pub fn addFile(self: *SourceMap, path: []const u8, content: []const u8) !u32 {
        const file = try SourceFile.init(self.allocator, path, content);
        try self.files.append(file);
        return @intCast(self.files.items.len - 1);
    }
};
