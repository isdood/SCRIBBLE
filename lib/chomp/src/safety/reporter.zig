///! Safety Violation Reporter
///! =====================
///! Author: isdood
///! Created: 2025-01-21 02:58:39 UTC
///! License: MIT

const std = @import("std");
const Safety = @import("mod.zig").Safety;

pub const Reporter = struct {
    allocator: std.mem.Allocator,
    violations: std.ArrayList(Violation),

    pub const Violation = struct {
        kind: ViolationKind,
        message: []const u8,
        location: Location,
        stack_trace: ?[]const u8,

        pub const ViolationKind = enum {
            ownership_violation,
            lifetime_violation,
            bounds_violation,
            null_violation,
            thread_safety_violation,
        };
    };

    pub const Location = struct {
        file: []const u8,
        line: usize,
        column: usize,
    };

    pub fn init(allocator: std.mem.Allocator) !Reporter {
        return Reporter{
            .allocator = allocator,
            .violations = std.ArrayList(Violation).init(allocator),
        };
    }

    pub fn reportViolation(self: *Reporter, violation: Violation) !void {
        try self.violations.append(violation);
        try self.emitViolation(violation);
    }

    fn emitViolation(self: *Reporter, violation: Violation) !void {
        const stderr = std.io.getStdErr().writer();

        // Print violation header in red
        try stderr.print("\x1b[31mSafety Violation: {s}\x1b[0m\n", .{violation.message});

        // Print location
        try stderr.print("  --> {s}:{}:{}\n", .{
            violation.location.file,
            violation.location.line,
            violation.location.column,
        });

        // Print stack trace if available
        if (violation.stack_trace) |trace| {
            try stderr.print("\nStack trace:\n{s}\n", .{trace});
        }

        // Print additional context based on violation kind
        switch (violation.kind) {
            .ownership_violation => try stderr.writeAll(
                "\nHelp: Ensure proper ownership transfer or use shared references\n"
            ),
            .lifetime_violation => try stderr.writeAll(
                "\nHelp: Verify that referenced data outlives all its references\n"
            ),
            .bounds_violation => try stderr.writeAll(
                "\nHelp: Add bounds checking or use safe indexing methods\n"
            ),
            .null_violation => try stderr.writeAll(
                "\nHelp: Use optional types or ensure proper initialization\n"
            ),
            .thread_safety_violation => try stderr.writeAll(
                "\nHelp: Use appropriate synchronization primitives\n"
            ),
        }
    }

    pub fn getSummary(self: Reporter) ViolationSummary {
        var summary = ViolationSummary{
            .ownership_violations = 0,
            .lifetime_violations = 0,
            .bounds_violations = 0,
            .null_violations = 0,
            .thread_safety_violations = 0,
        };

        for (self.violations.items) |violation| {
            switch (violation.kind) {
                .ownership_violation => summary.ownership_violations += 1,
                .lifetime_violation => summary.lifetime_violations += 1,
                .bounds_violation => summary.bounds_violations += 1,
                .null_violation => summary.null_violations += 1,
                .thread_safety_violation => summary.thread_safety_violations += 1,
            }
        }

        return summary;
    }
};

pub const ViolationSummary = struct {
    ownership_violations: usize,
    lifetime_violations: usize,
    bounds_violations: usize,
    null_violations: usize,
    thread_safety_violations: usize,

    pub fn totalViolations(self: ViolationSummary) usize {
        return self.ownership_violations +
        self.lifetime_violations +
        self.bounds_violations +
        self.null_violations +
        self.thread_safety_violations;
    }
};
