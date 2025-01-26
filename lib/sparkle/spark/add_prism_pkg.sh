#!/usr/bin/env bash

# Add Prism to STD_PACKAGES
# Created: 2025-01-26 15:40:47 UTC
# Author: isdood

set -e

BASE_DIR="/home/guavabot1/scribble/scribble/lib/spark"
SPARKLE_DIR="$BASE_DIR/.sparkle"
SEEDMANAGER="$SPARKLE_DIR/src/SeedManager.jl"

echo "📝 Adding Prism to standard packages..."

# Create backup
cp "$SEEDMANAGER" "${SEEDMANAGER}.bak"

# Update STD_PACKAGES to include prism
sed -i '/const STD_PACKAGES = Dict(/a \ \ \ \ \ \ \ \ "prism",     # 3D Memory Resonance Filesystem' "$SEEDMANAGER"

echo "✨ Added Prism to standard packages"
echo ""
echo "Original SeedManager.jl backed up to: ${SEEDMANAGER}.bak"
echo ""
echo "Please try again:"
echo "1. Restart Sparkle: ./sparkle.sh"
echo "2. Run:"
echo "   sparkle> seed sprout"
echo "   sparkle> seed plant std**prism"
