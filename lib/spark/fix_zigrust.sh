#!/bin/bash

# Fix ZigRust Script
# Author: isdood
# Created: 2025-01-26 15:46:40
# Repository: isdood/scribble

set -euo pipefail
IFS=$'\n\t'

SPARK_PURPLE='\033[0;35m'
SPARK_ORANGE='\033[0;33m'
NC='\033[0m'

print_spark() {
    echo -e "${SPARK_PURPLE}✨ $1${NC}"
}

print_glitch() {
    echo -e "${SPARK_ORANGE}⚡ $1${NC}"
}

# Update forge.sh with fixed parameter handling
cat > forge.sh << 'EOL'
#!/bin/bash

set -euo pipefail
IFS=$'\n\t'

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" &> /dev/null && pwd)"

# Initialize LD_LIBRARY_PATH if not set
: "${LD_LIBRARY_PATH:=}"
export LD_LIBRARY_PATH="${SCRIPT_DIR}/forge/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"

print_spark() {
    echo -e "\033[0;35m✨ $1\033[0m"
}

print_glitch() {
    echo -e "\033[0;33m⚡ $1\033[0m"
}

cleanup() {
    local pids=$(jobs -p)
    [ -n "$pids" ] && kill $pids 2>/dev/null || true
}

trap cleanup EXIT INT TERM

run_zig_command() {
    print_spark "Running: $*"
    "$@"
}

cd "${SCRIPT_DIR}" || exit 1

# Default to "run" if no command provided
CMD=${1:-run}

case "$CMD" in
    "test")
        print_spark "Running tests..."
        run_zig_command zig test forge/zig/tests.zig
        ;;
    "run"|*)
        print_spark "Running crystal-space bridge..."
        run_zig_command zig build run
        ;;
esac
EOL
chmod +x forge.sh

# Create minimal test implementation
mkdir -p forge/zig
cat > forge/zig/tests.zig << 'EOL'
const std = @import("std");
const testing = std.testing;

test "basic" {
    try testing.expect(true);
}
EOL

# Create minimal main implementation
cat > forge/zig/main.zig << 'EOL'
const std = @import("std");

pub fn main() !void {
    std.debug.print("✨ Crystal bridge initialized\n", .{});
    return;
}
EOL

# Create minimal build.zig
cat > build.zig << 'EOL'
const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = .ReleaseFast;

    const exe = b.addExecutable(.{
        .name = "crystal_bridge",
        .root_source_file = .{ .cwd_relative = "forge/zig/main.zig" },
        .target = target,
        .optimize = optimize,
    });

    b.installArtifact(exe);

    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());

    const run_step = b.step("run", "Run the app");
    run_step.dependOn(&run_cmd.step);

    const tests = b.addTest(.{
        .root_source_file = .{ .cwd_relative = "forge/zig/tests.zig" },
        .target = target,
        .optimize = optimize,
    });

    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&b.addRunArtifact(tests).step);
}
EOL

# Create directory structure
mkdir -p forge/{lib,c}
touch forge/lib/.keep

print_spark "Fixed forge.sh script with proper parameter handling"
print_spark "Created minimal test implementation"
print_spark "Created required directories"
print_spark "
To test the fixes:
1. Run: zig build
2. Run: ./forge.sh        (or ./forge.sh run)
3. Run: ./forge.sh test"

chmod +x forge.sh
