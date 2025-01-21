///! Diagnostic Engine
///! ===============
///! Author: Caleb J.D. Terkovics <isdood>
///! Last Updated: 2025-01-21 02:43:11 UTC

const std = @import("std");
const source = @import("source.zig");

pub const DiagnosticLevel = enum {
    Error,
    Warning,
    Note,
    Help,
};

pub const Diagnostic = struct {
    level: DiagnosticLevel,
    message: []const u8,
    span: ?source.Span,
    notes: std.ArrayList([]const u8),

    pub fn init(allocator: std.mem.Allocator, level: DiagnosticLevel, message: []const u8) !Diagnostic {
        return Diagnostic{
            .level = level,
            .message = message,
            .span = null,
            .notes = std.ArrayList([]const u8).init(allocator),
        };
    }

    pub fn deinit(self: *Diagnostic) void {
        self.notes.deinit();
    }
};

pub const DiagnosticEngine = struct {
    allocator: std.mem.Allocator,
    diagnostics: std.ArrayList(Diagnostic),
    error_count: usize,
    warning_count: usize,

    pub fn init(allocator: std.mem.Allocator) !DiagnosticEngine {
        return DiagnosticEngine{
            .allocator = allocator,
            .diagnostics = std.ArrayList(Diagnostic).init(allocator),
            .error_count = 0,
            .warning_count = 0,
        };
    }

    pub fn deinit(self: *DiagnosticEngine) void {
        for (self.diagnostics.items) |*diagnostic| {
            diagnostic.deinit();
        }
        self.diagnostics.deinit();
    }

    pub fn emit(self: *DiagnosticEngine, level: DiagnosticLevel, message: []const u8) !void {
        var diagnostic = try Diagnostic.init(self.allocator, level, message);
        try self.diagnostics.append(diagnostic);

        switch (level) {
            .Error => self.error_count += 1,
            .Warning => self.warning_count += 1,
            else => {},
        }
    }
};
