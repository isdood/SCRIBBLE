#!/usr/bin/env bash

# Fix Sparkle v4 - Spark Runtime Terminal Repair Script v0.4
# Author: isdood
# Created: 2025-01-26 11:41:35 UTC
# Repository: isdood/scribble

set -e

echo "🔧 Fixing Sparkle installation..."

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
cat > "$PKG_DIR/Project.toml" << EOT
name = "SparkSandbox"
uuid = "$UUID"
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

# Create Types.jl
echo "📝 Creating Types.jl..."
cat > "$PKG_DIR/src/Types.jl" << 'EOT'
# Type definitions
export Crystal, Wave, Pattern, patterns, GLOBAL_STATE

struct Crystal
    dimensions
    spacing
    data
end

struct Wave
    data
    frequency
end

struct Pattern
    name
    transform
end

# Global state
const patterns = Dict{String,Pattern}()
mutable struct SparkleState
    current_crystal::Union{Crystal,Nothing}
    current_wave::Union{Wave,Nothing}
    patterns::Dict{String,Pattern}
end

const GLOBAL_STATE = SparkleState(nothing, nothing, patterns)

# Register default patterns
patterns["default"] = Pattern("Default", w -> w)
patterns["invert"] = Pattern("Invert", w -> Wave(-w.data, w.frequency))
patterns["double"] = Pattern("Double", w -> Wave(w.data .* 2, w.frequency))
patterns["smooth"] = Pattern("Smooth", w -> begin
    data = copy(w.data)
    for i in 2:length(data)-1
        data[i] = mean(w.data[i-1:i+1])
    end
    Wave(data, w.frequency)
end)
EOT

# Create Crystal.jl
echo "📝 Creating Crystal.jl..."
cat > "$PKG_DIR/src/Crystal.jl" << 'EOT'
[Previous Crystal.jl content]
EOT

# Create SeedManager.jl
echo "📝 Creating SeedManager.jl..."
cat > "$PKG_DIR/src/SeedManager.jl" << 'EOT'
[Previous SeedManager.jl content]
EOT

# Create REPL.jl
echo "📝 Creating REPL.jl..."
cat > "$PKG_DIR/src/REPL.jl" << 'EOT'
[Previous REPL.jl content]
EOT

# Create init.jl
echo "📝 Creating init.jl..."
cat > "$PKG_DIR/init.jl" << 'EOT'
using Pkg
Pkg.activate(".")
Pkg.instantiate()

push!(LOAD_PATH, "@v#.#", "@stdlib")
push!(LOAD_PATH, dirname(pwd()))

try
    using SparkSandbox

    # Make SparkSandbox functions available in Main without redefining core functions
    for name in names(SparkSandbox; all=true)
        if !startswith(string(name), "#") &&
           name ∉ (:eval, :include, :using, :import) &&
           !isdefined(Main, name)
            @eval Main const $name = SparkSandbox.$name
        end
    end

    atreplinit() do repl
        @async begin
            sleep(0.1)
            try
                SparkSandbox.init_sparkle(repl)
                println("\n✨ Welcome to Sparkle - Spark Runtime Terminal ✨")
                println("Press '*' to enter Sparkle mode, type '?' for help\n")
                println("Created: 2025-01-26 11:41:35")
                println("User: isdood")
            catch e
                @warn "Failed to initialize Sparkle mode" exception=e
            end
        end
    end
catch e
    @error "Failed to load SparkSandbox" exception=e
    exit(1)
end
EOT

# Store template files properly
echo "📦 Saving template files..."
mkdir -p "$SCRIPT_DIR/.sparkle/src"
cp -r "$PKG_DIR/src/"* "$SCRIPT_DIR/.sparkle/src/"
cp "$PKG_DIR/Project.toml" "$SCRIPT_DIR/.sparkle/"
cp "$PKG_DIR/init.jl" "$SCRIPT_DIR/.sparkle/"

# Set correct permissions
chmod 644 "$SCRIPT_DIR/.sparkle/init.jl"
chmod 644 "$SCRIPT_DIR/.sparkle/Project.toml"
chmod 644 "$SCRIPT_DIR/.sparkle/src/"*
chmod 755 "$SCRIPT_DIR/.sparkle"
chmod 755 "$SCRIPT_DIR/.sparkle/src"

# Create sparkle.sh with correct permissions
cat > "$SCRIPT_DIR/sparkle.sh" << 'EOT'
#!/usr/bin/env bash

# Sparkle - Spark Runtime Terminal v0.1
# Created: 2025-01-26 11:41:35 UTC
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
    echo "Error: .sparkle directory not found. Please run fix_sparkle4.sh first."
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
        echo "Please run fix_sparkle4.sh to restore template files."
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
EOT

chmod +x "$SCRIPT_DIR/sparkle.sh"

# Verify template files and permissions
echo "🔍 Verifying template files..."
for file in "${required_files[@]}"; do
    if [ ! -f "$SCRIPT_DIR/.sparkle/$file" ]; then
        echo "❌ Error: Failed to copy $file to template directory"
        exit 1
    fi
    if [ ! -r "$SCRIPT_DIR/.sparkle/$file" ]; then
        echo "❌ Error: File $file is not readable"
        exit 1
    fi
done

echo "✅ Template files verified successfully"
echo "✨ Sparkle has been fixed! Try running ./sparkle.sh again."

# Cleanup
rm -rf "$TEMP_DIR"
