#!/bin/bash

# Set the date for file headers
DATE="2025-01-22"
AUTHOR="isdood"

# Print the current directory structure for reference
echo "Current Directory Structure:"
tree -L 3

# Create the new build.zig file
cat > build.zig << EOL
const std = @import("std");

pub fn build(b: *std.build.Builder) void {
    const mode = b.standardReleaseOptions();

    const exe = b.addExecutable("lazuline", "src/main.zig");
    exe.setBuildMode(mode);
    exe.install();

    const lib = b.addStaticLibrary("lazuline_lib", "src/lib.zig");
    lib.setBuildMode(mode);
    lib.install();

    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());

    b.default_step.dependOn(&run_cmd.step);
}
EOL

# Update the project configuration
# Assuming there's a Makefile or similar build script to update
if [ -f Makefile ]; then
    echo "Updating Makefile to use build.zig..."
    sed -i 's/^build:.*$/build:\n\tzig build/' Makefile
else
    echo "No Makefile found. Creating a new one to use build.zig..."
    cat > Makefile << EOL
# Makefile to build the project using Zig
# Created: ${DATE}
# Author: ${AUTHOR}

build:
\tzig build

clean:
\trm -rf zig-cache

install:
\tzig build install

run:
\tzig build run
EOL
fi

# Print the updated directory structure
echo "Updated Directory Structure:"
tree -L 3

echo "Project modified to use build.zig as the main build script successfully!"
echo "Next steps:"
echo "1. Review the created build.zig file and Makefile"
echo "2. Run 'make build' to test the new build script"
echo "3. Update the documentation as needed"
