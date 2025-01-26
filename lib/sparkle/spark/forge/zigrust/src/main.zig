const std = @import("std");
const c = @cImport({
    @cInclude("safety_bridge.h");
});

pub const SafetyLevel = enum {
    Calm,
    Balanced,
    Wild,
};

pub const CompilerOptions = struct {
    safety_level: SafetyLevel,
    enable_optimizations: bool,
    check_ownership: bool,
};

pub fn initSafetyBridge() !void {
    if (c.init_safety_bridge() != 0) {
        return error.SafetyBridgeInitFailed;
    }
}

pub fn checkSafety(code: []const u8, options: CompilerOptions) !void {
    const result = c.check_safety(
        code.ptr,
        code.len,
        @enumToInt(options.safety_level),
        options.enable_optimizations,
        options.check_ownership,
    );

    if (result != 0) {
        return error.SafetyCheckFailed;
    }
}

pub fn main() !void {
    try initSafetyBridge();
    std.debug.print("ZigRust Safety Bridge initialized\n", .{});
}
