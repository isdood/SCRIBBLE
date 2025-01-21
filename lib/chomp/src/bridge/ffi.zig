///! FFI Bridge Utilities
///! ==================
///! Author: Caleb J.D. Terkovics <isdood>
///! Created: 2025-01-21 02:52:27 UTC
///! License: MIT

const std = @import("std");
const safety = @import("../safety/mod.zig");
const types = @import("types.zig");

pub const FFIBridge = struct {
    safety_context: *safety.Context,
    type_registry: *types.TypeRegistry,

    pub fn init(safety_ctx: *safety.Context, type_reg: *types.TypeRegistry) FFIBridge {
        return .{
            .safety_context = safety_ctx,
            .type_registry = type_reg,
        };
    }

    pub fn wrap(self: FFIBridge, comptime Fn: type, func: Fn) WrapperFn(Fn) {
        const info = @typeInfo(Fn).Fn;
        return struct {
            fn wrapped(args: ...) callconv(.C) info.return_type.? {
                // Verify FFI call safety
                self.safety_context.verifyFFICall(@src()) catch |err| {
                    std.debug.panic("FFI safety violation: {}", .{err});
                };

                // Convert arguments
                var converted_args: std.BoundedArray(std.meta.ArgType(Fn, 0), info.args.len) =
                std.BoundedArray(std.meta.ArgType(Fn, 0), info.args.len).init(0) catch unreachable;

                inline for (info.args) |arg, i| {
                    converted_args.append(
                        types.TypeConverter.rustToZig(arg.type, args[i]) catch |err| {
                            std.debug.panic("Argument conversion failed: {}", .{err});
                        }
                    ) catch unreachable;
                }

                // Call function
                const result = @call(.auto, func, converted_args.slice());

                // Convert result
                return types.TypeConverter.zigToRust(info.return_type.?, result) catch |err| {
                    std.debug.panic("Return value conversion failed: {}", .{err});
                };
            }
        }.wrapped;
    }
};

fn WrapperFn(comptime Fn: type) type {
    const info = @typeInfo(Fn).Fn;
    return fn (args: ...) callconv(.C) info.return_type.?;
}

pub const FFIExport = struct {
    name: []const u8,
    fn_ptr: *const anyopaque,
    signature: FFISignature,

    pub fn init(name: []const u8, func: anytype) FFIExport {
        const Fn = @TypeOf(func);
        return .{
            .name = name,
            .fn_ptr = @ptrCast(*const anyopaque, func),
            .signature = FFISignature.fromType(Fn),
        };
    }
};

pub const FFISignature = struct {
    args: []const TypeId,
    return_type: ?TypeId,

    pub fn fromType(comptime T: type) FFISignature {
        const info = @typeInfo(T).Fn;
        var args: [info.args.len]TypeId = undefined;
        inline for (info.args) |arg, i| {
            args[i] = TypeId.init(arg.type);
        }
        return .{
            .args = &args,
            .return_type = if (info.return_type) |ret| TypeId.init(ret) else null,
        };
    }
};
