///! Type Conversion System
///! ===================
///! Author: Caleb J.D. Terkovics <isdood>
///! Created: 2025-01-21 02:52:27 UTC
///! License: MIT

const std = @import("std");
const safety = @import("../safety/mod.zig");

pub const TypeId = struct {
    hash: u64,
    name: []const u8,

    pub fn init(comptime T: type) !TypeId {
        const name = @typeName(T);
        const hash = std.hash.Wyhash.hash(0, name);
        return TypeId{
            .hash = hash,
            .name = name,
        };
    }
};

pub const TypeMapping = struct {
    zig_type: type,
    rust_type: []const u8,
    conversion: ?*const fn(*anyopaque) callconv(.C) *anyopaque = null,

    pub fn convert(self: TypeMapping, value: *anyopaque) !*anyopaque {
        if (self.conversion) |conv| {
            return conv(value);
        }
        return value;
    }
};

pub const TypeConverter = struct {
    pub fn zigToRust(comptime T: type, value: T) !RustEquivalent(T) {
        return switch (@typeInfo(T)) {
            .Int => value,
            .Float => value,
            .Bool => value,
            .Pointer => |ptr_info| switch (ptr_info.size) {
                .Slice => try sliceToRustSlice(T, value),
                .One => try ptrToRustPtr(T, value),
                else => @compileError("Unsupported pointer type"),
            },
            .Struct => try structToRustStruct(T, value),
            .Enum => @enumToInt(value),
            else => @compileError("Unsupported type"),
        };
    }

    pub fn rustToZig(comptime T: type, value: RustEquivalent(T)) !T {
        return switch (@typeInfo(T)) {
            .Int => value,
            .Float => value,
            .Bool => value,
            .Pointer => |ptr_info| switch (ptr_info.size) {
                .Slice => try rustSliceToSlice(T, value),
                .One => try rustPtrToPtr(T, value),
                else => @compileError("Unsupported pointer type"),
            },
            .Struct => try rustStructToStruct(T, value),
            .Enum => @intToEnum(T, value),
            else => @compileError("Unsupported type"),
        };
    }
};

pub fn RustEquivalent(comptime T: type) type {
    return switch (@typeInfo(T)) {
        .Int => T,
        .Float => T,
        .Bool => bool,
        .Pointer => |ptr_info| switch (ptr_info.size) {
            .Slice => RustSlice,
            .One => *allowzero anyopaque,
            else => @compileError("Unsupported pointer type"),
        },
        .Struct => RustStruct(T),
        .Enum => i32,
        else => @compileError("Unsupported type"),
    };
}

pub const RustSlice = extern struct {
    ptr: [*]const u8,
    len: usize,
};
