#!/usr/bin/env bash

# Fix Sparkle v6 - Spark Runtime Terminal Repair Script v0.6
# Author: isdood
# Created: 2025-01-26 11:53:26 UTC
# Repository: isdood/scribble

set -e

echo "🔧 Fixing Sparkle v6 installation..."

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
TEMP_DIR=$(mktemp -d)
PKG_DIR="$TEMP_DIR/SparkSandbox"

# Clean and create directories
rm -rf "$SCRIPT_DIR/.sparkle" 2>/dev/null || true
mkdir -p "$SCRIPT_DIR/.sparkle"
mkdir -p "$SCRIPT_DIR/.sparkle/src"
mkdir -p "$PKG_DIR/src"

# Create Project.toml
UUID="b03cc3df-2e3a-4564-98fe-76823717dd5f"
echo "📝 Creating Project.toml..."
mkdir -p "$PKG_DIR"
cat > "$PKG_DIR/Project.toml" << 'EOT'
name = "SparkSandbox"
uuid = "b03cc3df-2e3a-4564-98fe-76823717dd5f"
authors = ["isdood"]
version = "0.1.0"

[deps]
REPL = "3fa0cd96-eef1-5676-8a61-b3b8758bbffb"
UnicodePlots = "b8865327-cd53-5732-bb35-84acbb429228"
Statistics = "10745b16-79ce-11e8-11f9-7d13ad32a3b2"
ColorSchemes = "35d6a980-a343-548e-a6ea-1d62b119f2f4"
TOML = "fa267f1f-6049-4f14-aa54-33bafae1ed76"
Dates = "ade2ca70-3891-5945-98fb-dc099432e06a"

[compat]
julia = "1.11"
UnicodePlots = "3.7"
ColorSchemes = "3.28"
EOT

# Install global dependencies
echo "📦 Installing global dependencies..."
julia -e '
    using Pkg
    Pkg.add([
        PackageSpec(name="UnicodePlots", version="3.7.2"),
        PackageSpec(name="ColorSchemes", version="3.28.0"),
        PackageSpec(name="TOML"),
        PackageSpec(name="Statistics")
    ])
    Pkg.precompile()
'

echo "📝 Creating module files..."

# Create SparkSandbox.jl
echo "📝 Creating SparkSandbox.jl..."
mkdir -p "$PKG_DIR/src"
cat > "$PKG_DIR/src/SparkSandbox.jl" << 'EOT'
module SparkSandbox

using REPL
using REPL.LineEdit
using Statistics
using Dates
using TOML
using UnicodePlots
using ColorSchemes

include("Types.jl")
include("Crystal.jl")
include("SeedManager.jl")
include("REPL.jl")

# Re-export all public functions
export crystal, wave, weave, optimize, visualize
export seed_plant, seed_unplant, seed_garden, seed_sprout
export init_sparkle

end # module
EOT
