///! Simple Integration Example
///! ======================
///! Author: isdood
///! Created: 2025-01-21 03:05:35 UTC
///! License: MIT

const std = @import("std");
const chomp = @import("chomp");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Initialize Chomp
    var bridge = try chomp.Bridge.init(allocator);
    defer bridge.deinit();

    // Define Zig structure
    const Person = struct {
        name: []const u8,
        age: u32,
    };

    // Register with Rust
    try bridge.registerType(Person, "RustPerson");

    // Create instance
    const person = Person{
        .name = "Alice",
        .age = 30,
    };

    // Convert to Rust
    const rust_person = try bridge.toRust(person);
    std.debug.print("Converted to Rust: {any}\n", .{rust_person});

    // Call Rust function
    const result = try bridge.call("process_person", .{rust_person});
    std.debug.print("Rust processing result: {any}\n", .{result});
}
