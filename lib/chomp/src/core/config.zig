///! Compiler Configuration
///! =====================
///! Author: Caleb J.D. Terkovics <isdood>
///! Last Updated: 2025-01-21 02:43:11 UTC

const std = @import("std");

pub const Config = struct {
    pub const OptLevel = enum {
        Debug,
        Release,
        ReleaseSafe,
        ReleaseFast,
    };

    pub const Target = struct {
        arch: std.Target.Cpu.Arch,
        os: std.Target.Os.Tag,
        abi: std.Target.Abi,
    };

    opt_level: OptLevel,
    target: Target,
    enable_safety: bool,
    enable_warnings: bool,

    pub fn default() Config {
        return .{
            .opt_level = .Debug,
            .target = .{
                .arch = std.Target.current.cpu.arch,
                .os = std.Target.current.os.tag,
                .abi = std.Target.current.abi,
            },
            .enable_safety = true,
            .enable_warnings = true,
        };
    }
};
