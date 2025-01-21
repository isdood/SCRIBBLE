///! Safety System Core
///! ================
///! Author: isdood
///! Created: 2025-01-21 02:58:39 UTC
///! License: MIT

const std = @import("std");

pub const Safety = struct {
    context: Context,
    allocator: std.mem.Allocator,
    verifier: Verifier,
    reporter: Reporter,

    pub const Level = enum {
        strict,    // Full Rust-level safety guarantees
        standard,  // Basic memory and thread safety
        minimal,   // Performance-critical sections
    };

    pub fn init(allocator: std.mem.Allocator, level: Level) !*Safety {
        var self = try allocator.create(Safety);
        self.* = .{
            .context = try Context.init(allocator, level),
            .allocator = allocator,
            .verifier = try Verifier.init(allocator),
            .reporter = try Reporter.init(allocator),
        };
        return self;
    }

    pub fn verify(self: *Safety, operation: anytype) !void {
        const op_info = @typeInfo(@TypeOf(operation));

        try self.context.beginVerification();
        defer self.context.endVerification();

        switch (self.context.level) {
            .strict => try self.verifyStrict(operation),
            .standard => try self.verifyStandard(operation),
            .minimal => try self.verifyMinimal(operation),
        }
    }

    fn verifyStrict(self: *Safety, operation: anytype) !void {
        // Full Rust-style safety checks
        try self.verifier.checkOwnership(operation);
        try self.verifier.checkLifetimes(operation);
        try self.verifier.checkThreadSafety(operation);
        try self.verifier.checkBoundsChecks(operation);
        try self.verifier.checkNullChecks(operation);
    }

    fn verifyStandard(self: *Safety, operation: anytype) !void {
        // Basic safety checks
        try self.verifier.checkBoundsChecks(operation);
        try self.verifier.checkNullChecks(operation);
        try self.verifier.checkBasicThreadSafety(operation);
    }

    fn verifyMinimal(self: *Safety, operation: anytype) !void {
        // Only critical safety checks
        try self.verifier.checkCriticalBounds(operation);
        try self.verifier.checkCriticalNull(operation);
    }
};
