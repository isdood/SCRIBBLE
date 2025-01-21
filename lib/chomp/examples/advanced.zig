///! Advanced Integration Example
///! ========================
///! Author: isdood
///! Created: 2025-01-21 03:05:35 UTC
///! License: MIT

const std = @import("std");
const chomp = @import("chomp");

const Vector = struct {
    data: []f32,
    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator, size: usize) !Vector {
        return Vector{
            .data = try allocator.alloc(f32, size),
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *Vector) void {
        self.allocator.free(self.data);
    }
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Initialize Chomp with safety checks
    var bridge = try chomp.Bridge.init(allocator);
    defer bridge.deinit();

    // Register Vector type
    try bridge.registerType(Vector, "RustVector");

    // Create vectors
    var vec1 = try Vector.init(allocator, 3);
    defer vec1.deinit();
    vec1.data[0] = 1.0;
    vec1.data[1] = 2.0;
    vec1.data[2] = 3.0;

    var vec2 = try Vector.init(allocator, 3);
    defer vec2.deinit();
    vec2.data[0] = 4.0;
    vec2.data[1] = 5.0;
    vec2.data[2] = 6.0;

    // Convert to Rust
    const rust_vec1 = try bridge.toRust(vec1);
    const rust_vec2 = try bridge.toRust(vec2);

    // Call Rust function for vector operations
    const dot_product = try bridge.call("vector_dot_product", .{rust_vec1, rust_vec2});
    std.debug.print("Dot product: {d}\n", .{dot_product});

    const sum_vec = try bridge.call("vector_add", .{rust_vec1, rust_vec2});
    const result_vec = try bridge.toZig(Vector, sum_vec);
    defer result_vec.deinit();

    std.debug.print("Sum vector: ", .{});
    for (result_vec.data) |value| {
        std.debug.print("{d} ", .{value});
    }
    std.debug.print("\n", .{});
}
