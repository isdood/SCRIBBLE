#!/usr/bin/env bash
set -e

echo "[$(date -u '+%Y-%m-%d %H:%M:%S UTC')] Building Zig components..."
zig build
zig build test

echo "✨ Zig build complete!"
