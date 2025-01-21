#!/usr/bin/env bash

# Lazuline Zig Setup Script
# Created: 2025-01-21 18:31:45
# Author: isdood

# Create Zig project structure
mkdir -p zig/zig-bind/src
cd zig/zig-bind

# Create build.zig
cat > build.zig << 'END'
const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const lib = b.addSharedLibrary(.{
        .name = "lazuline_zig",
        .root_source_file = .{ .path = "src/main.zig" },
        .target = target,
        .optimize = optimize,
    });

    lib.linkLibC();
    b.installArtifact(lib);

    // Tests
    const main_tests = b.addTest(.{
        .root_source_file = .{ .path = "src/main.zig" },
        .target = target,
        .optimize = optimize,
    });

    const run_main_tests = b.addRunArtifact(main_tests);

    const test_step = b.step("test", "Run library tests");
    test_step.dependOn(&run_main_tests.step);
}
END

# Create main.zig
cat > src/main.zig << 'END'
const std = @import("std");

export fn lazuline_init() c_int {
    return 0;
}

export fn lazuline_version() [*:0]const u8 {
    return "0.1.0";
}

test "basic initialization" {
    try std.testing.expectEqual(@as(c_int, 0), lazuline_init());
}

test "version string" {
    const version = lazuline_version();
    try std.testing.expect(std.mem.eql(u8, std.mem.span(version), "0.1.0"));
}
END

# Create build.sh helper script
cat > build.sh << 'END'
#!/usr/bin/env bash
set -e

echo "[$(date -u '+%Y-%m-%d %H:%M:%S UTC')] Building Zig components..."
zig build
zig build test

echo "✨ Zig build complete!"
END

chmod +x build.sh

echo "✨ Zig project structure created!"
echo "Run './build.sh' to build and test"
