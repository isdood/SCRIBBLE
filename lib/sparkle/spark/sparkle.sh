#!/usr/bin/env bash

# Sparkle - Spark Runtime Terminal v0.1
# Created: 2025-01-26 11:59:23 UTC
# Author: isdood

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
TEMP_DIR=$(mktemp -d)
PKG_DIR="$TEMP_DIR/SparkSandbox"

# Create package structure
mkdir -p "$PKG_DIR"
mkdir -p "$PKG_DIR/src"

# Verify .sparkle directory exists
if [ ! -d "$SCRIPT_DIR/.sparkle" ]; then
    echo "Error: .sparkle directory not found. Please run mega_fix.sh first."
    exit 1
fi

# Verify required template files exist
required_files=(
    "Project.toml"
    "init.jl"
    "src/SparkSandbox.jl"
    "src/Types.jl"
    "src/Crystal.jl"
    "src/SeedManager.jl"
    "src/REPL.jl"
)

for file in "${required_files[@]}"; do
    if [ ! -f "$SCRIPT_DIR/.sparkle/$file" ]; then
        echo "Error: Required template file $file not found in .sparkle directory."
        echo "Please run mega_fix.sh to restore template files."
        rm -rf "$TEMP_DIR"
        exit 1
    fi
done

# Copy template files with correct permissions
echo "📦 Setting up Sparkle environment..."
mkdir -p "$PKG_DIR/src"
cp -r "$SCRIPT_DIR/.sparkle/src/"* "$PKG_DIR/src/"
cp "$SCRIPT_DIR/.sparkle/Project.toml" "$PKG_DIR/"
cp "$SCRIPT_DIR/.sparkle/init.jl" "$PKG_DIR/"
chmod 644 "$PKG_DIR/src/"*
chmod 644 "$PKG_DIR/Project.toml"
chmod 644 "$PKG_DIR/init.jl"

# Show banner
cat << 'BANNER'
    ✨ 𝕊𝕡𝕒𝕣𝕜𝕝𝕖 ✨
    Spark Runtime Terminal
    Version 0.1-alpha
BANNER

# Create trap to clean up temporary directory
trap 'rm -rf "$TEMP_DIR"' EXIT

# Start Julia REPL with proper environment
cd "$PKG_DIR" || exit 1
JULIA_PROJECT="." exec julia -i --color=yes init.jl
