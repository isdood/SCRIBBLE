///! Safety System Tests
///! ================
///! Author: isdood
///! Created: 2025-01-21 03:05:35 UTC
///! License: MIT

const std = @import("std");
const testing = std.testing;
const chomp = @import("chomp");

test "ownership tracking" {
    const allocator = testing.allocator;

    var safety = try chomp.Safety.init(allocator, .strict);
    defer safety.deinit();

    const Resource = struct {
        data: []u8,
    };

    // Create owned resource
    var resource = Resource{
        .data = try allocator.dupe(u8, "test"),
    };
    defer allocator.free(resource.data);

    // Track ownership
    try safety.trackOwnership(&resource);

    // Attempt to transfer ownership
    try safety.transferOwnership(&resource);

    // This should fail - resource already transferred
    try testing.expectError(
        error.InvalidOwnership,
        safety.transferOwnership(&resource)
    );
}

test "lifetime verification" {
    const allocator = testing.allocator;

    var safety = try chomp.Safety.init(allocator, .strict);
    defer safety.deinit();

    const Borrower = struct {
        ref: *const i32,
    };

    var value: i32 = 42;

    // Create borrower
    var borrower = Borrower{
        .ref = &value,
    };

    // Track lifetime
    try safety.trackLifetime(&borrower, &value);

    // Verify lifetime
    try safety.verifyLifetime(&borrower);

    // Value goes out of scope
    try testing.expectError(
        error.InvalidLifetime,
        safety.verifyLifetime(&borrower)
    );
}
