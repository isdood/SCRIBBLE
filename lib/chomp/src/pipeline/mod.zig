///! Compiler Pipeline
///! ===============
///! Author: Caleb J.D. Terkovics <isdood>
///! Created: 2025-01-21 02:54:58 UTC
///! License: MIT

const std = @import("std");
const ast = @import("../ast/mod.zig");
const ir = @import("../ir/mod.zig");
const codegen = @import("../codegen/mod.zig");
const safety = @import("../safety/mod.zig");

pub const Pipeline = struct {
    allocator: std.mem.Allocator,
    config: PipelineConfig,
    statistics: Statistics,
    current_phase: Phase,

    pub const PipelineConfig = struct {
        safety_level: safety.Level,
        optimization_level: OptLevel,
        enable_cache: bool = true,
        parallel_processing: bool = true,
        debug_info: bool = true,
    };

    pub const Statistics = struct {
        start_time: i64,
        phase_timings: std.AutoHashMap(Phase, u64),
        memory_usage: usize,
        cache_hits: usize,
        errors_found: usize,
    };

    pub const Phase = enum {
        parsing,
        analysis,
        ir_generation,
        optimization,
        codegen,
        linking,
    };

    pub fn init(allocator: std.mem.Allocator, config: PipelineConfig) !*Pipeline {
        var self = try allocator.create(Pipeline);
        self.* = .{
            .allocator = allocator,
            .config = config,
            .statistics = .{
                .start_time = std.time.milliTimestamp(),
                .phase_timings = std.AutoHashMap(Phase, u64).init(allocator),
                .memory_usage = 0,
                .cache_hits = 0,
                .errors_found = 0,
            },
            .current_phase = .parsing,
        };
        return self;
    }

    pub fn compile(self: *Pipeline, source_files: []const []const u8) !void {
        try self.runPhase(.parsing, source_files);
        try self.runPhase(.analysis, null);
        try self.runPhase(.ir_generation, null);
        try self.runPhase(.optimization, null);
        try self.runPhase(.codegen, null);
        try self.runPhase(.linking, null);
    }

    fn runPhase(self: *Pipeline, phase: Phase, data: ?*anyopaque) !void {
        const phase_start = std.time.milliTimestamp();
        self.current_phase = phase;

        switch (phase) {
            .parsing => try self.parsing(data.?),
            .analysis => try self.analysis(),
            .ir_generation => try self.irGeneration(),
            .optimization => try self.optimization(),
            .codegen => try self.codeGeneration(),
            .linking => try self.linking(),
        }

        const phase_duration = std.time.milliTimestamp() - phase_start;
        try self.statistics.phase_timings.put(phase, @intCast(u64, phase_duration));
    }

    fn parsing(self: *Pipeline, source_files: []const []const u8) !void {
        // Handle each source file
        for (source_files) |source| {
            const parser = try Parser.init(self.allocator, source);
            defer parser.deinit();

            try parser.parse();
        }
    }

    fn analysis(self: *Pipeline) !void {
        var analyzer = try Analyzer.init(self.allocator, self.config.safety_level);
        defer analyzer.deinit();

        try analyzer.analyze();
    }

    fn irGeneration(self: *Pipeline) !void {
        var ir_gen = try IRGenerator.init(self.allocator);
        defer ir_gen.deinit();

        try ir_gen.generate();
    }

    fn optimization(self: *Pipeline) !void {
        var optimizer = try Optimizer.init(self.allocator, self.config.optimization_level);
        defer optimizer.deinit();

        try optimizer.optimize();
    }

    fn codeGeneration(self: *Pipeline) !void {
        var code_gen = try codegen.CodeGen.init(
            self.allocator,
            self.config.safety_level,
        );
        defer code_gen.deinit();

        try code_gen.generate();
    }

    fn linking(self: *Pipeline) !void {
        var linker = try Linker.init(self.allocator);
        defer linker.deinit();

        try linker.link();
    }
};
