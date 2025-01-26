#!/usr/bin/env bash

# Sudo Forge RunSTD - Sparkle Standard Library Implementation with Sudo
# Author: isdood
# Created: 2025-01-26 12:32:14 UTC
# Repository: isdood/scribble

# Check if script is run as root
if [ "$EUID" -ne 0 ]; then
    echo "🔒 This script requires root permissions to set up the Sparkle environment"
    echo "Running with sudo..."
    sudo "$0" "$@"
    exit $?
fi

set -e

echo "⚒️ Forge RunSTD - Sparkle Standard Library Implementation"
echo "Created: 2025-01-26 12:32:14 UTC"
echo "Author: isdood"

# Get the real user who ran sudo
REAL_USER="${SUDO_USER:-$USER}"
SCRIPT_DIR="$(dirname "$(readlink -f "$0")")"
SPARKLE_DIR="$SCRIPT_DIR/.sparkle"

echo "🔑 Running as root, will set permissions for user: $REAL_USER"

# Verify Sparkle installation
if [ ! -d "$SPARKLE_DIR" ]; then
    echo "❌ Error: Sparkle not installed. Please run mega_fix.sh first."
    exit 1
fi

# Clean up existing directories first
echo "🧹 Cleaning up existing directories..."
rm -rf "$SPARKLE_DIR/std" 2>/dev/null || true
rm -rf "$SPARKLE_DIR/packages" 2>/dev/null || true

# Create all directories first
echo "📚 Creating directory structure..."
mkdir -p "$SPARKLE_DIR/std"
mkdir -p "$SPARKLE_DIR/std/math"
mkdir -p "$SPARKLE_DIR/std/cubed"
mkdir -p "$SPARKLE_DIR/std/whisper"
mkdir -p "$SPARKLE_DIR/std/clone"
mkdir -p "$SPARKLE_DIR/std/inq"
mkdir -p "$SPARKLE_DIR/std/align"
mkdir -p "$SPARKLE_DIR/std/shout"
mkdir -p "$SPARKLE_DIR/packages"

# Set initial permissions
chown -R "$REAL_USER:$REAL_USER" "$SPARKLE_DIR"
find "$SPARKLE_DIR" -type d -exec chmod 755 {} \;

# Update SeedManager.jl
echo "🔧 Updating SeedManager.jl..."
cat > "$SPARKLE_DIR/src/SeedManager.jl" << 'EOT'
# Seed package manager functions
export seed_plant, seed_unplant, seed_garden, seed_sprout

using TOML, Dates

# Standard library packages
const STD_PACKAGES = Dict(
    "std" => [
        "math",      # Basic mathematical operations
        "cubed",     # Cube operations
        "whisper",   # Text case manipulation
        "clone",     # Object cloning
        "inq",       # Inquiry functions
        "align",     # Text alignment
        "shout"      # Text case manipulation
    ],
    "shout" => [],
    "align" => []
)

function seed_plant(package_spec)
    parts = split(package_spec, "**")
    if length(parts) == 1
        println("🌱 Planting full package: $(parts[1])")
        _install_full_package(parts[1])
    elseif length(parts) == 2
        println("🌱 Planting component $(parts[2]) from package $(parts[1])")
        _install_package_component(parts[1], parts[2])
    else
        error("Invalid package specification format")
    end
end

function seed_unplant(package_spec)
    parts = split(package_spec, "**")
    if length(parts) == 1
        println("🗑️ Unplanting full package: $(parts[1])")
        _remove_full_package(parts[1])
    elseif length(parts) == 2
        println("🗑️ Unplanting component $(parts[2]) from package $(parts[1])")
        _remove_package_component(parts[1], parts[2])
    else
        error("Invalid package specification format")
    end
end

function seed_garden()
    if !isfile("config.spark")
        error("No config.spark found. Initialize with 'seed sprout' first.")
    end

    config = TOML.parsefile("config.spark")
    println("\n🌿 Installed Packages:")
    println("====================")

    if haskey(config, "packages")
        for (pkg, components) in config["packages"]
            println("📦 $pkg")
            if !isempty(components)
                for comp in components
                    println("  └─ $comp")
                end
            end
        end
    else
        println("No packages installed yet.")
    end
    println()
end

function seed_sprout()
    if isfile("config.spark")
        error("config.spark already exists!")
    end

    config = Dict(
        "project" => Dict(
            "name" => basename(pwd()),
            "version" => "0.1.0",
            "author" => "isdood",
            "created" => "2025-01-26 12:32:14"
        ),
        "packages" => Dict(),
        "dependencies" => Dict()
    )

    open("config.spark", "w") do io
        TOML.print(io, config)
    end
    println("🌱 Initialized new Spark project")
end

function _install_full_package(package)
    if !haskey(STD_PACKAGES, package)
        error("Package $package not found in standard library")
    end

    config = _load_config()
    if !haskey(config, "packages")
        config["packages"] = Dict()
    end

    if !haskey(config["packages"], package)
        config["packages"][package] = String[]
        if !isempty(STD_PACKAGES[package])
            config["packages"][package] = copy(STD_PACKAGES[package])
        end
    end

    _save_config(config)
    println("✨ Successfully planted $package")
end

function _install_package_component(package, component)
    if !haskey(STD_PACKAGES, package)
        error("Package $package not found in standard library")
    end

    if !(component in STD_PACKAGES[package])
        error("Component $component not found in package $package")
    end

    config = _load_config()
    if !haskey(config, "packages")
        config["packages"] = Dict()
    end

    if !haskey(config["packages"], package)
        config["packages"][package] = String[]
    end

    if !(component in config["packages"][package])
        push!(config["packages"][package], component)
    end

    _save_config(config)
    println("✨ Successfully planted $package**$component")
end

function _remove_full_package(package)
    config = _load_config()
    if haskey(config["packages"], package)
        delete!(config["packages"], package)
        _save_config(config)
        println("✨ Successfully unplanted $package")
    else
        println("Package $package is not installed")
    end
end

function _remove_package_component(package, component)
    config = _load_config()
    if haskey(config["packages"], package)
        components = config["packages"][package]
        filter!(c -> c != component, components)
        if isempty(components)
            delete!(config["packages"], package)
        else
            config["packages"][package] = components
        end
        _save_config(config)
        println("✨ Successfully unplanted $package**$component")
    else
        println("Package $package is not installed")
    end
end

function _load_config()
    if !isfile("config.spark")
        error("No config.spark found. Initialize with 'seed sprout' first.")
    end
    TOML.parsefile("config.spark")
end

function _save_config(config)
    open("config.spark", "w") do io
        TOML.print(io, config)
    end
end
EOT

# Create standard library implementations
echo "📝 Creating standard library implementations..."

# std/math implementation
cat > "$SPARKLE_DIR/std/math/init.jl" << 'EOT'
module Math

export add, subtract, multiply, divide

add(a, b) = a + b
subtract(a, b) = a - b
multiply(a, b) = a * b
divide(a, b) = a / b

end
EOT

# std/cubed implementation
cat > "$SPARKLE_DIR/std/cubed/init.jl" << 'EOT'
module Cubed

export cube, uncube

cube(x) = x^3
uncube(x) = cbrt(x)

end
EOT

# std/whisper implementation
cat > "$SPARKLE_DIR/std/whisper/init.jl" << 'EOT'
module Whisper

export whisper

whisper(text::String) = lowercase(text)

end
EOT

# std/clone implementation
cat > "$SPARKLE_DIR/std/clone/init.jl" << 'EOT'
module Clone

export clone, deep_clone

function clone(obj)
    copy(obj)
end

function deep_clone(obj)
    deepcopy(obj)
end

end
EOT

# std/inq implementation
cat > "$SPARKLE_DIR/std/inq/init.jl" << 'EOT'
module Inq

export inquire, inspect, describe

function inquire(obj)
    typeof(obj)
end

function inspect(obj)
    fieldnames(typeof(obj))
end

function describe(obj)
    println("Type: ", typeof(obj))
    println("Fields: ", fieldnames(typeof(obj)))
    println("Methods: ", methods(typeof(obj)))
end

end
EOT

# std/align implementation
cat > "$SPARKLE_DIR/std/align/init.jl" << 'EOT'
module Align

export align_left, align_right, align_center

function align_left(text::String, width::Integer)
    rpad(text, width)
end

function align_right(text::String, width::Integer)
    lpad(text, width)
end

function align_center(text::String, width::Integer)
    padding = width - length(text)
    left_pad = div(padding, 2)
    right_pad = padding - left_pad
    " "^left_pad * text * " "^right_pad
end

end
EOT

# std/shout implementation
cat > "$SPARKLE_DIR/std/shout/init.jl" << 'EOT'
module Shout

export shout, whisper, toggle_case

shout(text::String) = uppercase(text)
whisper(text::String) = lowercase(text)
toggle_case(text::String) = join(islowercase(c) ? uppercase(c) : lowercase(c) for c in text)

end
EOT

# Final permissions setup
echo "🔒 Setting final permissions..."
chown -R "$REAL_USER:$REAL_USER" "$SPARKLE_DIR"
find "$SPARKLE_DIR" -type d -exec chmod 755 {} \;
find "$SPARKLE_DIR" -type f -exec chmod 644 {} \;

echo "✨ Standard library and packages have been forged!"
echo "Try 'seed plant std**whisper' or 'seed plant std**inq' in Sparkle."
