#!/bin/bash

# Set the date for file headers
DATE="2025-01-22"
AUTHOR="isdood"

# Create or modify the build.zig file
cat > build.zig << EOL
const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const mode = b.standardOptimizeOption(.{});

    const exe = b.addExecutable(.{
        .name = "lazuline",
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = mode,
    });

    const lib = b.addStaticLibrary(.{
        .name = "lazuline_lib",
        .root_source_file = b.path("src/lib.zig"),
        .target = target,
        .optimize = mode,
    });

    // Install the executable and library
    const exe_install = b.getInstallStep();
    const lib_install = b.getInstallStep();
    exe_install.dependOn(&exe.step);
    lib_install.dependOn(&lib.step);

    // Ensure default step depends on install steps
    b.default_step.dependOn(&exe_install);
    b.default_step.dependOn(&lib_install);
}
EOL

# Print a success message
echo "build.zig file created/modified successfully!"
echo "Next steps:"
echo "1. Review the created/modified build.zig file"
echo "2. Run 'zig build' to test the new build script"
echo "3. Update the documentation as needed"
