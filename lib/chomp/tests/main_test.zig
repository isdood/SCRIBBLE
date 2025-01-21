///! Main Test Runner
///! =============
///! Author: isdood
///! Created: 2025-01-21 03:08:36 UTC
///! License: MIT

const std = @import("std");
const testing = std.testing;

// Import all test files
comptime {
    _ = @import("basic_integration.zig");
    _ = @import("bridge_tests.zig");
    _ = @import("safety_tests.zig");
}

test {
    // Run all tests in referenced files
    std.testing.refAllDecls(@This());
}
