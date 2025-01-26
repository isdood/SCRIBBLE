#!/usr/bin/env bash

# Sparkle - Spark Runtime Terminal v0.1
# Created: 2025-01-26 00:14:30 UTC
# Author: isdood

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
TEMP_DIR=$(mktemp -d)
PKG_DIR="$TEMP_DIR/SparkSandbox"

# Create package structure
mkdir -p "$PKG_DIR/src"

# Copy template files
cp -r "$SCRIPT_DIR/.sparkle/"* "$PKG_DIR/src/"
cp "$SCRIPT_DIR/.sparkle/Project.toml" "$PKG_DIR/"

# Create initialization script
cat > "$PKG_DIR/init.jl" << 'INIT'
# Set up environment
using Pkg
Pkg.activate(".")
Pkg.instantiate()
Pkg.precompile()

push!(LOAD_PATH, "@v#.#", "@stdlib")
push!(LOAD_PATH, dirname(pwd()))

try
    @eval using SparkSandbox
    atreplinit() do repl
        @async begin
            sleep(0.1)
            try
                SparkSandbox.init_sparkle(repl)
                println("\n✨ Welcome to Sparkle - Spark Runtime Terminal ✨")
                println("Press '*' to enter Sparkle mode, type '?' for help\n")
                println("Created: 2025-01-26 00:14:30")
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
INIT

# Show banner
cat << 'BANNER'
    ✨ 𝕊𝕡𝕒𝕣𝕜𝕝𝕖 ✨
    Spark Runtime Terminal
    Version 0.1-alpha
BANNER

# Start Julia REPL with proper environment
cd "$PKG_DIR" && julia --project=. -i init.jl

# Cleanup
rm -rf "$TEMP_DIR"
