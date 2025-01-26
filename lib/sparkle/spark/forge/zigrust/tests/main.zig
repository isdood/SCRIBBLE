const std = @import("std");
const testing = std.testing;
const zigrust = @import("main.zig");

test "safety levels" {
    try zigrust.initSafetyBridge();

    const code = "safe_function()";
    const options = zigrust.CompilerOptions{
        .safety_level = .Calm,
        .enable_optimizations = true,
        .check_ownership = true,
    };

    try zigrust.checkSafety(code, options);
}
