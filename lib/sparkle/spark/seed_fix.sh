#!/usr/bin/env bash

# Fix Sparkle seed initialization
# Created: 2025-01-26 18:55:24 UTC
# Author: isdood

set -e

BASE_DIR="/home/guavabot1/scribble/scribble/lib/sparkle/spark"
SPARKLE_DIR="$BASE_DIR/.sparkle"
SANDBOX_FILE="$SPARKLE_DIR/src/SparkSandbox.jl"

echo "📝 Fixing Sparkle seed initialization..."

# Update SparkSandbox.jl to create default config
cat > "$SANDBOX_FILE" << 'EOT'
module SparkSandbox

using REPL
using REPL.LineEdit
using Statistics
using Dates
using TOML
using UnicodePlots
using ColorSchemes
using LinearAlgebra

# Get the absolute path to .sparkle directory
const SPARKLE_ROOT = dirname(dirname(@__FILE__))

# Include core types and modules
include("Types.jl")
include("Crystal.jl")
include("SeedManager.jl")
include("REPL.jl")

"""
Create default config.spark if it doesn't exist
"""
function ensure_config()
    if !isfile("config.spark")
        config = Dict(
            "project" => Dict(
                "name" => "SparkSandbox",
                "version" => "0.1.0",
                "author" => "isdood",
                "created" => "2025-01-26 18:55:24"
            ),
            "packages" => Dict(
                "std" => ["look"]  # Default components
            )
        )

        open("config.spark", "w") do io
            TOML.print(io, config)
        end

        # Create std directory structure
        mkpath(joinpath("std", "look"))

        # Write look module
        open(joinpath("std", "look", "init.jl"), "w") do f
            write(f, """
            module Look
            export look
            function look(args...)
                entries = readdir(".")
                if isempty(entries)
                    println("(empty directory)")
                else
                    for entry in sort(entries)
                        if isdir(entry)
                            printstyled(entry, "/\\n", color=:blue)
                        else
                            println(entry)
                        end
                    end
                end
            end
            end # module Look
            """)
        end
    end
end

# Function to load a component
function load_component(package::String, component::String)
    println("📦 Loading component $component from package $package...")

    # Use absolute path to std directory
    path = joinpath("std", component, "init.jl")
    println("📂 Looking for component at: $path")

    if !isfile(path)
        println("❌ Component file not found: $path")
        return false
    end

    # Include the module
    try
        include(path)
        println("✨ Successfully loaded $component")

        # If it's a Look module, make functions available globally
        if component == "look"
            @eval import .Look: look
            @eval export look
        end

        # If it's a Prism module, make functions available globally
        if component == "prism"
            @eval import .Prism: init_prism, mount_prism, unmount_prism
            @eval export init_prism, mount_prism, unmount_prism
        end

        return true
    catch e
        println("❌ Error loading component $component: ", e)
        return false
    end
end

# Load all components from config.spark
function load_components()
    println("\n🔄 Loading components...")

    # Ensure config exists
    ensure_config()

    config = TOML.parsefile("config.spark")
    if !haskey(config, "packages")
        println("❌ No packages section in config.spark")
        return
    end

    for (pkg, components) in config["packages"]
        for comp in components
            load_component(pkg, comp)
        end
    end

    println("✨ Component loading complete\n")
end

# Initialize
function __init__()
    @async begin
        sleep(0.1)  # Wait for REPL to initialize
        ensure_config()  # Create default config
        load_components()  # Load components
    end
end

# Re-export all public functions
export crystal, wave, weave, optimize, visualize
export seed_plant, seed_unplant, seed_garden, seed_sprout
export init_sparkle

# Export module loading
export load_component, load_components

end # module
EOT

echo "✨ Fixed Sparkle seed initialization"
echo ""
echo "Changes made:"
echo "1. Added ensure_config() function"
echo "2. Created default config.spark with look component"
echo "3. Added automatic config creation on startup"
echo "4. Added component initialization"
echo "5. Updated paths for new directory structure"
echo ""
echo "Please try again:"
echo "1. Clean up:"
echo "   ./clean_sandbox.sh"
echo ""
echo "2. Restart Sparkle:"
echo "   ./sparkle.sh"
echo ""
echo "3. The look command should be available immediately"

# Set permissions
chown guavabot1:guavabot1 "$SANDBOX_FILE"
chmod 644 "$SANDBOX_FILE"
