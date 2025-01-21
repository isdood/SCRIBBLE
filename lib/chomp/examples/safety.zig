///! Safety Features Example
///! ===================
///! Author: isdood
///! Created: 2025-01-21 03:05:35 UTC
///! License: MIT

const std = @import("std");
const chomp = @import("chomp");

const Resource = struct {
    data: []u8,
    owner: bool,

    pub fn init(allocator: std.mem.Allocator, data: []const u8) !Resource {
        return Resource{
            .data = try allocator.dupe(u8, data),
            .owner = true,
        };
    }

    pub fn deinit(self: *Resource, allocator: std.mem.Allocator) void {
        if (self.owner) {
            allocator.free(self.data);
        }
    }
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Initialize Chomp with strict safety
    var bridge = try chomp.Bridge.init(allocator);
    defer bridge.deinit();

    var safety = try chomp.Safety.init(allocator, .strict);
    defer safety.deinit();

    // Register resource type
    try bridge.registerType(Resource, "RustResource");

    // Create and track resource
    var resource = try Resource.init(allocator, "Hello, Rust!");
    defer resource.deinit(allocator);

    // Track ownership
    try safety.trackOwnership(&resource);

    // Demonstrate ownership transfer
    const rust_resource = try bridge.toRust(resource);
    _ = try bridge.call("process_resource", .{rust_resource});

    // This would fail - ownership transferred
    // try safety.verifyOwnership(&resource);

    std.debug.print("Resource safely processed by Rust\n", .{});
}
