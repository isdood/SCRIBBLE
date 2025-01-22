const std = @import("std");
const Builder = std.build.Builder;

pub fn build(b: *Builder) void {
    const mode = b.standardReleaseOptions();
    const exe = b.addExecutable("zig_benchmark", "benchmarks/zig/benchmark.zig");
    exe.setBuildMode(mode);
    exe.install();
}
