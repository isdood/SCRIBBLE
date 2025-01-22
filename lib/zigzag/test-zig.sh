#!/bin/bash
set -e  # Exit on error
echo "Running Zig tests..."
zig test src/zig/vector/vector3d.zig
