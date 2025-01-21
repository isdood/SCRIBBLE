///! Safety Context Management
///! ======================
///! Author: isdood
///! Created: 2025-01-21 02:58:39 UTC
///! License: MIT

const std = @import("std");
const Safety = @import("mod.zig").Safety;

pub const Context = struct {
    allocator: std.mem.Allocator,
    level: Safety.Level,
    current_scope: ?*Scope,
    ownership_map: std.AutoHashMap(*anyopaque, Ownership),
    lifetime_tracker: LifetimeTracker,

    pub const Scope = struct {
        parent: ?*Scope,
        variables: std.StringHashMap(Variable),
        borrows: std.ArrayList(Borrow),

        pub const Variable = struct {
            name: []const u8,
            type_info: TypeInfo,
            ownership: Ownership,
            location: Location,
        };

        pub const Borrow = struct {
            variable: *Variable,
            kind: BorrowKind,
            lifetime: Lifetime,
        };
    };

    pub const Ownership = enum {
        owned,
        shared,
        borrowed,
        none,
    };

    pub const BorrowKind = enum {
        shared,
        mutable,
    };

    pub fn init(allocator: std.mem.Allocator, level: Safety.Level) !Context {
        return Context{
            .allocator = allocator,
            .level = level,
            .current_scope = null,
            .ownership_map = std.AutoHashMap(*anyopaque, Ownership).init(allocator),
            .lifetime_tracker = try LifetimeTracker.init(allocator),
        };
    }

    pub fn beginScope(self: *Context) !void {
        const new_scope = try self.allocator.create(Scope);
        new_scope.* = .{
            .parent = self.current_scope,
            .variables = std.StringHashMap(Scope.Variable).init(self.allocator),
            .borrows = std.ArrayList(Scope.Borrow).init(self.allocator),
        };
        self.current_scope = new_scope;
    }

    pub fn endScope(self: *Context) !void {
        if (self.current_scope) |scope| {
            // Verify all borrows are returned
            try self.verifyBorrowsReturned(scope);

            // Clean up scope resources
            var it = scope.variables.iterator();
            while (it.next()) |entry| {
                self.allocator.free(entry.key_ptr.*);
                self.deallocateVariable(entry.value_ptr);
            }

            scope.variables.deinit();
            scope.borrows.deinit();

            self.current_scope = scope.parent;
            self.allocator.destroy(scope);
        }
    }

    pub fn trackOwnership(self: *Context, ptr: *anyopaque, ownership: Ownership) !void {
        try self.ownership_map.put(ptr, ownership);
    }

    pub fn verifyOwnership(self: Context, ptr: *anyopaque) !void {
        const ownership = self.ownership_map.get(ptr) orelse return error.NoOwnershipTracking;

        switch (ownership) {
            .owned => return,
            .shared => if (self.level == .strict) return error.SharedAccessViolation,
            .borrowed => return error.BorrowedAccessViolation,
            .none => return error.InvalidOwnership,
        }
    }

    fn verifyBorrowsReturned(self: *Context, scope: *Scope) !void {
        for (scope.borrows.items) |borrow| {
            if (self.lifetime_tracker.isActive(borrow.lifetime)) {
                return error.BorrowNotReturned;
            }
        }
    }

    fn deallocateVariable(self: *Context, variable: *Scope.Variable) void {
        self.allocator.free(variable.name);
        // Additional cleanup based on type_info...
    }
};
