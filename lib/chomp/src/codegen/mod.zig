///! Code Generation Module
///! ====================
///! Author: Caleb J.D. Terkovics <isdood>
///! Created: 2025-01-21 02:49:34 UTC
///! License: MIT

const std = @import("std");
const ast = @import("../ast/mod.zig");
const ir = @import("../ir/mod.zig");
const safety = @import("../safety/mod.zig");

pub const CodeGen = struct {
    allocator: std.mem.Allocator,
    target: Target,
    safety_level: safety.Level,
    ir_module: *ir.Module,

    const Self = @This();

    pub fn init(allocator: std.mem.Allocator, target: Target, safety_level: safety.Level) !*Self {
        return Self{
            .allocator = allocator,
            .target = target,
            .safety_level = safety_level,
            .ir_module = undefined, // Set during generation
        };
    }

    pub fn generateCode(self: *Self, module: *ir.Module) !void {
        self.ir_module = module;

        switch (self.target.lang) {
            .zig => try self.generateZig(),
            .rust => try self.generateRust(),
        }
    }

    pub fn generateZig(self: *Self) !void {
        var gen = try ZigGenerator.init(self);
        try gen.generate();
    }

    pub fn generateRust(self: *Self) !void {
        var gen = try RustGenerator.init(self);
        try gen.generate();
    }
};

/// Target language specifics
pub const Target = struct {
    lang: Language,
    os: std.Target.Os.Tag,
    cpu: std.Target.Cpu.Arch,

    pub const Language = enum {
        zig,
        rust,
    };
};
