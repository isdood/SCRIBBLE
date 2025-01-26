#!/usr/bin/env bash

# Sudo Forge RunSTD4 - Sparkle Standard Library Implementation Part 4
# Author: isdood
# Created: 2025-01-26 12:51:27 UTC
# Repository: isdood/scribble

# Check if script is run as root
if [ "$EUID" -ne 0 ]; then
    echo "🔒 This script requires root permissions to set up the Sparkle environment"
    echo "Running with sudo..."
    sudo "$0" "$@"
    exit $?
fi

set -e

echo "⚒️ Forge RunSTD4 - Additional Standard Library Components"
echo "Created: 2025-01-26 12:51:27 UTC"
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

# Create new std directories
echo "📚 Creating new std directories..."
mkdir -p "$SPARKLE_DIR/std/echo"
mkdir -p "$SPARKLE_DIR/std/glitch"
mkdir -p "$SPARKLE_DIR/std/history"

# Update SeedManager.jl to include new components
echo "🔧 Updating SeedManager.jl..."
sed -i '/const STD_PACKAGES = Dict(/,/)/c\
const STD_PACKAGES = Dict(\
    "std" => [\
        "math",      # Basic mathematical operations\
        "cubed",     # Cube operations\
        "whisper",   # Text case manipulation\
        "clone",     # Object cloning\
        "inq",       # Inquiry functions\
        "align",     # Text alignment\
        "shout",     # Text case manipulation\
        "any",       # Type-agnostic operations\
        "array",     # Array utilities\
        "ascii",     # ASCII text operations\
        "coll",      # Collection utilities\
        "comp",      # Comparison operations\
        "conv",      # Type conversion utilities\
        "def",       # Default value handling\
        "echo",      # Echo utilities\
        "glitch",    # Glitch art effects\
        "history"    # Command history\
    ]\
)' "$SPARKLE_DIR/src/SeedManager.jl"

# Create std/echo implementation
echo "📝 Creating std/echo/init.jl..."
cat > "$SPARKLE_DIR/std/echo/init.jl" << 'EOT'
module Echo

export echo, echo_nl, echo_err, echo_fmt
export echo_debug, echo_info, echo_warn

using Dates

"""
Echo a message with optional formatting
"""
function echo(msg...; color=:normal)
    printstyled(join(msg, " "), color=color)
end

"""
Echo a message with newline
"""
function echo_nl(msg...; color=:normal)
    printstyled(join(msg, " "), "\n", color=color)
end

"""
Echo to stderr
"""
function echo_err(msg...)
    printstyled(stderr, join(msg, " "), "\n", color=:red)
end

"""
Echo with printf-style formatting
"""
function echo_fmt(fmt::String, args...; color=:normal)
    printstyled(@sprintf(fmt, args...), color=color)
end

"""
Echo debug message with timestamp
"""
function echo_debug(msg...)
    printstyled("[DEBUG ", now(), "] ", color=:cyan)
    println(join(msg, " "))
end

"""
Echo info message with timestamp
"""
function echo_info(msg...)
    printstyled("[INFO ", now(), "] ", color=:blue)
    println(join(msg, " "))
end

"""
Echo warning message with timestamp
"""
function echo_warn(msg...)
    printstyled("[WARN ", now(), "] ", color=:yellow)
    println(join(msg, " "))
end

end
EOT

# Create std/glitch implementation
echo "📝 Creating std/glitch/init.jl..."
cat > "$SPARKLE_DIR/std/glitch/init.jl" << 'EOT'
module Glitch

export corrupt_bytes, bit_shift, data_bend
export glitch_text, random_artifacts

using Random

"""
Corrupt bytes in data with given probability
"""
function corrupt_bytes(data::Vector{UInt8}, probability::Float64=0.1)
    corrupted = copy(data)
    for i in eachindex(corrupted)
        if rand() < probability
            corrupted[i] = rand(UInt8)
        end
    end
    corrupted
end

"""
Bit shift data by specified amount
"""
function bit_shift(data::Vector{UInt8}, shift::Int)
    shifted = copy(data)
    for i in eachindex(shifted)
        shifted[i] = shifted[i] << shift % 8
    end
    shifted
end

"""
Apply data bending effect
"""
function data_bend(data::Vector{UInt8}, intensity::Float64=0.5)
    bent = copy(data)
    for i in eachindex(bent)
        if rand() < intensity
            bent[i] = bent[i] ⊻ rand(UInt8)
        end
    end
    bent
end

"""
Create glitched text effect
"""
function glitch_text(text::String, intensity::Float64=0.3)
    glitch_chars = "!@#$%^&*()_+-=[]{}|;:,.<>?"
    chars = collect(text)
    for i in eachindex(chars)
        if rand() < intensity
            chars[i] = rand(glitch_chars)
        end
    end
    String(chars)
end

"""
Generate random glitch artifacts
"""
function random_artifacts(length::Integer)
    artifacts = Vector{UInt8}(undef, length)
    for i in eachindex(artifacts)
        artifacts[i] = rand(UInt8)
    end
    artifacts
end

end
EOT

# Create std/history implementation
echo "📝 Creating std/history/init.jl..."
cat > "$SPARKLE_DIR/std/history/init.jl" << 'EOT'
module History

export add_history, clear_history, get_history
export search_history, save_history, load_history

using Dates

const HISTORY_FILE = joinpath(homedir(), ".spark_history")
const MAX_HISTORY = 1000

mutable struct HistoryEntry
    timestamp::DateTime
    command::String
    status::Symbol  # :success or :error
end

global _history = HistoryEntry[]

"""
Add command to history
"""
function add_history(command::String, status::Symbol=:success)
    entry = HistoryEntry(now(), command, status)
    push!(_history, entry)
    if length(_history) > MAX_HISTORY
        popfirst!(_history)
    end
    entry
end

"""
Clear command history
"""
function clear_history()
    empty!(_history)
    nothing
end

"""
Get command history
"""
function get_history(; limit::Union{Int,Nothing}=nothing)
    isnothing(limit) ? _history : _history[end-min(limit,length(_history))+1:end]
end

"""
Search command history
"""
function search_history(pattern::String)
    filter(e -> occursin(pattern, e.command), _history)
end

"""
Save history to file
"""
function save_history(file::String=HISTORY_FILE)
    open(file, "w") do io
        for entry in _history
            println(io, "$(entry.timestamp)|$(entry.status)|$(entry.command)")
        end
    end
end

"""
Load history from file
"""
function load_history(file::String=HISTORY_FILE)
    clear_history()
    if isfile(file)
        for line in eachline(file)
            parts = split(line, "|", limit=3)
            if length(parts) == 3
                timestamp = DateTime(parts[1])
                status = Symbol(parts[2])
                command = parts[3]
                push!(_history, HistoryEntry(timestamp, command, status))
            end
        end
    end
    nothing
end

# Load history on module initialization
load_history()

end
EOT

# Set permissions
echo "🔒 Setting permissions..."
chown -R "$REAL_USER:$REAL_USER" "$SPARKLE_DIR"
find "$SPARKLE_DIR" -type d -exec chmod 755 {} \;
find "$SPARKLE_DIR" -type f -exec chmod 644 {} \;

echo "✨ Additional standard library components have been forged!"
echo "Available components:"
echo "  - std**echo     (Echo utilities)"
echo "  - std**glitch   (Glitch art effects)"
echo "  - std**history  (Command history)"
echo "  - std**inq      (Inquiry functions, already implemented)"
echo "Try 'seed plant std**echo' in Sparkle."
