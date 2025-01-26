#!/usr/bin/env bash

# Sudo Forge RunSTD2 - Sparkle Standard Library Implementation Part 2
# Author: isdood
# Created: 2025-01-26 12:39:44 UTC
# Repository: isdood/scribble

# Check if script is run as root
if [ "$EUID" -ne 0 ]; then
    echo "🔒 This script requires root permissions to set up the Sparkle environment"
    echo "Running with sudo..."
    sudo "$0" "$@"
    exit $?
fi

set -e

echo "⚒️ Forge RunSTD2 - Additional Standard Library Components"
echo "Created: 2025-01-26 12:39:44 UTC"
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
mkdir -p "$SPARKLE_DIR/std/any"
mkdir -p "$SPARKLE_DIR/std/array"
mkdir -p "$SPARKLE_DIR/std/ascii"

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
        "ascii"      # ASCII text operations\
    ],\
    "shout" => [],\
    "align" => []\
)' "$SPARKLE_DIR/src/SeedManager.jl"

# Create std/any implementation
echo "📝 Creating std/any/init.jl..."
cat > "$SPARKLE_DIR/std/any/init.jl" << 'EOT'
module Any

export is_nothing, is_something, unwrap, unwrap_or, try_convert

"""
Check if a value is nothing
"""
is_nothing(x) = isnothing(x)

"""
Check if a value is not nothing
"""
is_something(x) = !isnothing(x)

"""
Unwrap a value or throw an error if it's nothing
"""
function unwrap(x)
    isnothing(x) && throw(ArgumentError("Attempted to unwrap nothing"))
    x
end

"""
Unwrap a value or return a default value if it's nothing
"""
unwrap_or(x, default) = isnothing(x) ? default : x

"""
Try to convert a value to a specific type, return nothing if conversion fails
"""
function try_convert(T::Type, x)
    try
        convert(T, x)
    catch
        nothing
    end
end

end
EOT

# Create std/array implementation
echo "📝 Creating std/array/init.jl..."
cat > "$SPARKLE_DIR/std/array/init.jl" << 'EOT'
module Array

export chunk, compact, unique_by, flatten, group_by, partition

"""
Split array into chunks of specified size
"""
function chunk(arr, size::Integer)
    [arr[i:min(i + size - 1, end)] for i in 1:size:length(arr)]
end

"""
Remove nil elements from array
"""
function compact(arr)
    filter(!isnothing, arr)
end

"""
Get unique elements based on a function
"""
function unique_by(f, arr)
    unique(x -> f(x), arr)
end

"""
Flatten a nested array structure
"""
function flatten(arr)
    result = []
    for item in arr
        if item isa AbstractArray
            append!(result, flatten(item))
        else
            push!(result, item)
        end
    end
    result
end

"""
Group array elements by function result
"""
function group_by(f, arr)
    groups = Dict()
    for item in arr
        key = f(item)
        if !haskey(groups, key)
            groups[key] = []
        end
        push!(groups[key], item)
    end
    groups
end

"""
Split array into two arrays based on predicate
"""
function partition(pred, arr)
    trues = []
    falses = []
    for item in arr
        if pred(item)
            push!(trues, item)
        else
            push!(falses, item)
        end
    end
    (trues, falses)
end

end
EOT

# Create std/ascii implementation
echo "📝 Creating std/ascii/init.jl..."
cat > "$SPARKLE_DIR/std/ascii/init.jl" << 'EOT'
module ASCII

export to_ascii, strip_non_ascii, is_ascii, ascii_only
export to_ascii_upper, to_ascii_lower
export encode_hex, decode_hex

"""
Convert string to ASCII, replacing non-ASCII characters
"""
function to_ascii(s::AbstractString)
    join(isascii(c) ? c : '?' for c in s)
end

"""
Remove all non-ASCII characters from string
"""
function strip_non_ascii(s::AbstractString)
    join(c for c in s if isascii(c))
end

"""
Check if string contains only ASCII characters
"""
function is_ascii(s::AbstractString)
    all(isascii, s)
end

"""
Ensure string contains only ASCII characters or throw error
"""
function ascii_only(s::AbstractString)
    is_ascii(s) || throw(ArgumentError("String contains non-ASCII characters"))
    s
end

"""
Convert string to uppercase ASCII
"""
function to_ascii_upper(s::AbstractString)
    ascii_only(uppercase(s))
end

"""
Convert string to lowercase ASCII
"""
function to_ascii_lower(s::AbstractString)
    ascii_only(lowercase(s))
end

"""
Encode string as ASCII hex
"""
function encode_hex(s::AbstractString)
    bytes = Vector{UInt8}(s)
    join(string(b, base=16, pad=2) for b in bytes)
end

"""
Decode ASCII hex string
"""
function decode_hex(s::AbstractString)
    length(s) % 2 == 0 || throw(ArgumentError("Hex string length must be even"))
    bytes = [parse(UInt8, s[i:i+1], base=16) for i in 1:2:length(s)]
    String(bytes)
end

end
EOT

# Set permissions
echo "🔒 Setting permissions..."
chown -R "$REAL_USER:$REAL_USER" "$SPARKLE_DIR"
find "$SPARKLE_DIR" -type d -exec chmod 755 {} \;
find "$SPARKLE_DIR" -type f -exec chmod 644 {} \;

echo "✨ Additional standard library components have been forged!"
echo "Available components:"
echo "  - std**any    (Type-agnostic operations)"
echo "  - std**array  (Array utilities)"
echo "  - std**ascii  (ASCII text operations)"
echo "Try 'seed plant std**array' in Sparkle."
