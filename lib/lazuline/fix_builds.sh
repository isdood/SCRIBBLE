#!/bin/bash
# fix_builds.sh
# Created: 2025-01-21 18:35:18
# Author: isdood

echo "[$(date -u '+%Y-%m-%d %H:%M:%S UTC')] Fixing build configurations..."

# Create Zig structure
mkdir -p zig/zig-bind/src
cat > zig/zig-bind/build.zig << 'END'
const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const lib = b.addSharedLibrary(.{
        .name = "lazuline_zig",
        .root_source_file = .{ .cwd_relative = "src/main.zig" },  // Fixed: using cwd_relative instead of path
        .target = target,
        .optimize = optimize,
    });

    lib.linkLibC();
    b.installArtifact(lib);

    const main_tests = b.addTest(.{
        .root_source_file = .{ .cwd_relative = "src/main.zig" },  // Fixed: using cwd_relative instead of path
        .target = target,
        .optimize = optimize,
    });

    const run_main_tests = b.addRunArtifact(main_tests);
    const test_step = b.step("test", "Run library tests");
    test_step.dependOn(&run_main_tests.step);
}
END

cat > zig/zig-bind/src/main.zig << 'END'
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

# Update test script
cat > test.sh << 'END'
#!/bin/bash

echo "[$(date -u '+%Y-%m-%d %H:%M:%S UTC')] Starting Lazuline test suite..."

# Build and test Rust components
echo "Building Rust components..."
cargo clean && cargo build --release

if [ $? -ne 0 ]; then
    echo "[$(date -u '+%Y-%m-%d %H:%M:%S UTC')] Error: Failed to build Rust components"
    exit 1
fi

# Build and test Zig components
echo "Building Zig components..."
cd zig/zig-bind && zig build && zig build test

if [ $? -ne 0 ]; then
    echo "[$(date -u '+%Y-%m-%d %H:%M:%S UTC')] Error: Failed to build Zig components"
    exit 1
fi

cd ../..
echo "[$(date -u '+%Y-%m-%d %H:%M:%S UTC')] ✨ All tests completed successfully!"
END

chmod +x test.sh

echo "✨ Build configurations fixed! Run './test.sh' to test."
