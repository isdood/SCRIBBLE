///! Core Module - Chomp Compiler
///! ===========================
///! Author: Caleb J.D. Terkovics <isdood>
///! Current User: isdood
///! Created: 2025-01-21 02:43:11 UTC
///! Version: 0.1.0
///! License: MIT

const std = @import("std");
pub const config = @import("config.zig");
pub const diagnostics = @import("diagnostics.zig");
pub const session = @import("session.zig");
pub const source = @import("source.zig");
pub const options = @import("options.zig");

pub const Core = struct {
    allocator: std.mem.Allocator,
    session: session.Session,
    diagnostics: diagnostics.DiagnosticEngine,

    pub fn init(allocator: std.mem.Allocator, opts: options.Options) !Core {
        return Core{
            .allocator = allocator,
            .session = try session.Session.init(allocator, opts),
            .diagnostics = try diagnostics.DiagnosticEngine.init(allocator),
        };
    }

    pub fn deinit(self: *Core) void {
        self.diagnostics.deinit();
        self.session.deinit();
    }
};
