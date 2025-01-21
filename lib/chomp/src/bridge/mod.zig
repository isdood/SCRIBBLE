///! Bridge System for Zig-Rust Integration
///! ====================================
///! Author: Caleb J.D. Terkovics <isdood>
///! Created: 2025-01-21 02:52:27 UTC
///! License: MIT

const std = @import("std");
const safety = @import("../safety/mod.zig");
const types = @import("types.zig");

pub const Bridge = struct {
    allocator: std.mem.Allocator,
    safety_context: safety.Context,
    type_registry: TypeRegistry,
    lifetime_tracker: LifetimeTracker,

    const Self = @This();

    pub fn init(allocator: std.mem.Allocator, safety_level: safety.Level) !*Self {
        return Self{
            .allocator = allocator,
            .safety_context = try safety.Context.init(safety_level),
            .type_registry = try TypeRegistry.init(allocator),
            .lifetime_tracker = try LifetimeTracker.init(allocator),
        };
    }

    pub fn registerType(self: *Self, zig_type: type, rust_type: []const u8) !void {
        try self.type_registry.register(zig_type, rust_type);
    }

    pub fn trackLifetime(self: *Self, ptr: *anyopaque, lifetime: Lifetime) !void {
        try self.lifetime_tracker.track(ptr, lifetime);
    }

    pub fn verifyCall(self: *Self, func_name: []const u8) !void {
        try self.safety_context.verifyFFICall(func_name);
    }
};

pub const TypeRegistry = struct {
    types: std.AutoHashMap(TypeId, TypeMapping),
    allocator: std.mem.Allocator,

    pub fn register(self: *Self, zig_type: type, rust_type: []const u8) !void {
        const type_id = try TypeId.init(zig_type);
        const mapping = TypeMapping{
            .zig_type = zig_type,
            .rust_type = try self.allocator.dupe(u8, rust_type),
        };
        try self.types.put(type_id, mapping);
    }
};

pub const LifetimeTracker = struct {
    lifetimes: std.AutoHashMap(*anyopaque, Lifetime),
    allocator: std.mem.Allocator,

    pub fn track(self: *Self, ptr: *anyopaque, lifetime: Lifetime) !void {
        try self.lifetimes.put(ptr, lifetime);
    }

    pub fn verify(self: *Self, ptr: *anyopaque) !void {
        const lifetime = self.lifetimes.get(ptr) orelse
        return error.NoLifetimeTracking;
        try lifetime.verify();
    }
};

pub const Lifetime = struct {
    start: i64,
    end: ?i64,
    context: []const u8,

    pub fn verify(self: Lifetime) !void {
        const now = std.time.milliTimestamp();
        if (self.end) |end| {
            if (now > end) {
                return error.ExpiredLifetime;
            }
        }
    }
};
