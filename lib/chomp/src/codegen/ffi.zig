///! FFI Binding Generator
///! ===================
///! Author: Caleb J.D. Terkovics <isdood>
///! Created: 2025-01-21 02:49:34 UTC
///! License: MIT

const std = @import("std");
const ir = @import("../ir/mod.zig");
const safety = @import("../safety/mod.zig");

pub const FFIGenerator = struct {
    codegen: *CodeGen,
    buffer: std.ArrayList(u8),

    const Self = @This();

    pub fn init(codegen: *CodeGen) !*Self {
        return Self{
            .codegen = codegen,
            .buffer = std.ArrayList(u8).init(codegen.allocator),
        };
    }

    pub fn generate(self: *Self) !void {
        switch (self.codegen.target.lang) {
            .zig => try self.generateZigFFI(),
            .rust => try self.generateRustFFI(),
        }
    }

    fn generateZigFFI(self: *Self) !void {
        try self.writeLine("// FFI bindings for Rust functions");
        try self.writeLine("pub const extern \"rust\" = struct {");

        for (self.codegen.ir_module.functions) |func| {
            if (func.isExtern) {
                try self.generateZigExternFn(func);
            }
        }

        try self.writeLine("};");
    }

    fn generateRustFFI(self: *Self) !void {
        try self.writeLine("#[no_mangle]");
        try self.writeLine("pub extern \"C\" {");

        for (self.codegen.ir_module.functions) |func| {
            if (func.isExtern) {
                try self.generateRustExternFn(func);
            }
        }

        try self.writeLine("}");
    }

    fn generateSafetyWrapper(self: *Self, func: *ir.Function) !void {
        if (self.codegen.safety_level == .strict) {
            try self.writeLine("// Safety wrapper");
            try self.writeLine("try safety.verifyFFICall();");
            try self.writeLine("try safety.trackFFILifetimes();");
        }
    }
};
