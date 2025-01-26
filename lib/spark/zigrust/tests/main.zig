const std = @import("std");
const testing = std.testing;
const main = @import("../src/zig/main.zig");

test "safety levels" {
    try main.initSafetyBridge();

    const code = "safe_function()";
    const options = main.CompilerOptions{
        .safety_level = .Calm,
        .enable_optimizations = true,
        .check_ownership = true,
    };

    try main.checkSafety(code, options);
}
