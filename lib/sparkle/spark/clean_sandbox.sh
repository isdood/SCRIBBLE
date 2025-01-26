#!/usr/bin/env bash

# Cleanup temporary Sparkle sandboxes
# Created: 2025-01-26 15:54:51
# Author: isdood

echo "🧹 Cleaning up temporary sandboxes..."

# Find all temporary sandbox directories
for tmp_dir in /tmp/tmp.*; do
    if [ -d "$tmp_dir" ]; then
        echo "Checking $tmp_dir..."
        rm -rf "$tmp_dir/SparkSandbox" 2>/dev/null
        rm -f "$tmp_dir/Project.toml" 2>/dev/null
        rm -f "$tmp_dir/Manifest.toml" 2>/dev/null
    fi
done

echo "✨ Cleanup complete"
