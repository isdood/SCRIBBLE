//! Rust Bridge Interface
//! Author: @isdood
//! Created: 2025-01-21 13:08:37 UTC

const std = @import("std");
const types = @import("../core/types.zig");
const error = @import("../core/error.zig");

const Result = types.Result;
const FacetError = error.FacetError;

/// Rust bridge configuration
pub const RustConfig = extern struct {
    /// Memory allocation strategy
    allocator_type: u8,
    /// Thread pool size
    thread_count: u32,
    /// Enable SIMD operations
    enable_simd: bool,
    /// Debug level
    debug_level: u8,
    /// Reserved for future use
    _padding: [4]u8 = [_]u8{0} ** 4,
};

/// Rust computation context
const RustContext = extern struct {
    /// Operation identifier
    op_id: u64,
    /// Crystal clarity
    clarity: f64,
    /// Resonance factor
    resonance: f64,
    /// Error code
    error_code: u32,
    /// Flags
    flags: u32,
};

/// Rust vector type for FFI
const RustVec = extern struct {
    /// Data pointer
    ptr: [*]const u8,
    /// Length
    len: usize,
    /// Capacity
    cap: usize,
};

/// External Rust functions
extern "rust" {
    fn rust_init(config: *const RustConfig) bool;
    fn rust_cleanup() void;
    fn rust_compute(ctx: *RustContext, input: *const RustVec) RustVec;
    fn rust_free_vec(vec: RustVec) void;
    fn rust_get_error_message(error_code: u32) [*:0]const u8;
    fn rust_release_string(ptr: [*:0]const u8) void;
}

/// Rust bridge interface
pub const RustBridge = struct {
    config: RustConfig,
    initialized: bool,
    context: RustContext,
    allocator: std.mem.Allocator,

    const Self = @This();

    /// Initialize Rust bridge
    pub fn init(allocator: std.mem.Allocator, thread_count: u32) !*Self {
        const bridge = try allocator.create(Self);

        const config = RustConfig{
            .allocator_type = 0, // Default allocator
            .thread_count = thread_count,
            .enable_simd = true,
            .debug_level = 0,
        };

        if (!rust_init(&config)) {
            return error.RustInitializationFailed;
        }

        bridge.* = .{
            .config = config,
            .initialized = true,
            .context = .{
                .op_id = 0,
                .clarity = 1.0,
                .resonance = 1.0,
                .error_code = 0,
                .flags = 0,
            },
            .allocator = allocator,
        };

        return bridge;
    }

    /// Clean up bridge resources
    pub fn deinit(self: *Self) void {
        if (self.initialized) {
            rust_cleanup();
        }
        self.allocator.destroy(self);
    }

    /// Perform computation through Rust
    pub fn compute(self: *Self, data: []const u8) ![]const u8 {
        if (!self.initialized) return error.BridgeNotInitialized;

        const input_vec = RustVec{
            .ptr = data.ptr,
            .len = data.len,
            .cap = data.len,
        };

        const result_vec = rust_compute(&self.context, &input_vec);
        defer rust_free_vec(result_vec);

        if (self.context.error_code != 0) {
            const err_msg = rust_get_error_message(self.context.error_code);
            defer rust_release_string(err_msg);
            std.log.err("Rust computation error: {s}", .{err_msg});
            return error.RustComputationFailed;
        }

        const result = try self.allocator.alloc(u8, result_vec.len);
        @memcpy(result.ptr, result_vec.ptr, result_vec.len);

        return result;
    }

    /// Update computation context
    pub fn updateContext(self: *Self, clarity: f64, resonance: f64) void {
        self.context.clarity = clarity;
        self.context.resonance = resonance;
        self.context.op_id += 1;
    }

    /// Get error message for code
    pub fn getErrorMessage(error_code: u32) []const u8 {
        const msg = rust_get_error_message(error_code);
        defer rust_release_string(msg);
        return std.mem.span(msg);
    }

    /// Check if SIMD is enabled
    pub fn hasSimd(self: Self) bool {
        return self.config.enable_simd;
    }

    /// Get current thread count
    pub fn getThreadCount(self: Self) u32 {
        return self.config.thread_count;
    }
};

/// Crystal computation request
pub const ComputeRequest = struct {
    /// Operation type
    op_type: u8,
    /// Input data
    data: []const u8,
    /// Crystal clarity requirement
    min_clarity: f64,
    /// Required resonance
    min_resonance: f64,
    /// Enable whimsy
    enable_whimsy: bool,
};

/// Crystal computation response
pub const ComputeResponse = struct {
    /// Result data
    data: []const u8,
    /// Achieved clarity
    clarity: f64,
    /// Achieved resonance
    resonance: f64,
    /// Operation success
    success: bool,
    /// Error message if any
    error_message: ?[]const u8,
};

test "rust_bridge_basic" {
    const allocator = std.testing.allocator;

    var bridge = try RustBridge.init(allocator, 4);
    defer bridge.deinit();

    try std.testing.expect(bridge.initialized);
    try std.testing.expect(bridge.hasSimd());
    try std.testing.expectEqual(bridge.getThreadCount(), 4);
}

test "rust_bridge_compute" {
    const allocator = std.testing.allocator;

    var bridge = try RustBridge.init(allocator, 1);
    defer bridge.deinit();

    const input = "test data";
    const result = try bridge.compute(input);
    defer allocator.free(result);

    try std.testing.expect(result.len > 0);
}

test "rust_bridge_context" {
    const allocator = std.testing.allocator;

    var bridge = try RustBridge.init(allocator, 1);
    defer bridge.deinit();

    bridge.updateContext(0.95, 0.98);
    try std.testing.expectEqual(bridge.context.clarity, 0.95);
    try std.testing.expectEqual(bridge.context.resonance, 0.98);
}
