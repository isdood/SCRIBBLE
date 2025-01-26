#!/usr/bin/env bash

# Sudo Forge RunSTD3 - Sparkle Standard Library Implementation Part 3
# Author: isdood
# Created: 2025-01-26 12:43:17 UTC
# Repository: isdood/scribble

# Check if script is run as root
if [ "$EUID" -ne 0 ]; then
    echo "🔒 This script requires root permissions to set up the Sparkle environment"
    echo "Running with sudo..."
    sudo "$0" "$@"
    exit $?
fi

set -e

echo "⚒️ Forge RunSTD3 - Additional Standard Library Components"
echo "Created: 2025-01-26 12:43:17 UTC"
echo "Author: isdood"

# Get the real user who ran sudo
REAL_USER="${SUDO_USER:-$USER}"
SCRIPT_DIR="$(dirname "$(readlink -f "$0")")"
SPARKLE_DIR="$SCRIPT_DIR/.sparkle"

echo "🔑 Running as root, will set permissions for user: isdood"

# Verify Sparkle installation
if [ ! -d "$SPARKLE_DIR" ]; then
    echo "❌ Error: Sparkle not installed. Please run mega_fix.sh first."
    exit 1
fi

# Create new std directories
echo "📚 Creating new std directories..."
mkdir -p "$SPARKLE_DIR/std/coll"
mkdir -p "$SPARKLE_DIR/std/comp"
mkdir -p "$SPARKLE_DIR/std/conv"
mkdir -p "$SPARKLE_DIR/std/def"

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
        "def"        # Default value handling\
    ],\
    "shout" => [],\
    "align" => []\
)' "$SPARKLE_DIR/src/SeedManager.jl"

# Create std/coll implementation
echo "📝 Creating std/coll/init.jl..."
cat > "$SPARKLE_DIR/std/coll/init.jl" << 'EOT'
module Coll

export OrderedDict, DefaultDict, Counter
export frequency_count, most_common, least_common
export merge_with, zip_with, cartesian_product

using DataStructures

"""
Count frequency of elements in a collection
"""
function frequency_count(collection)
    counter = Counter{eltype(collection)}()
    for item in collection
        counter[item] += 1
    end
    counter
end

"""
Get n most common elements
"""
function most_common(collection, n=nothing)
    counter = frequency_count(collection)
    sorted = sort(collect(counter), by=x->x[2], rev=true)
    isnothing(n) ? sorted : sorted[1:min(n, length(sorted))]
end

"""
Get n least common elements
"""
function least_common(collection, n=nothing)
    counter = frequency_count(collection)
    sorted = sort(collect(counter), by=x->x[2])
    isnothing(n) ? sorted : sorted[1:min(n, length(sorted))]
end

"""
Merge collections with a combining function
"""
function merge_with(f, d1::AbstractDict, d2::AbstractDict)
    result = empty(d1)
    for k in union(keys(d1), keys(d2))
        if haskey(d1, k) && haskey(d2, k)
            result[k] = f(d1[k], d2[k])
        else
            result[k] = get(d1, k, get(d2, k, nothing))
        end
    end
    result
end

"""
Combine two collections element-wise with a function
"""
function zip_with(f, xs, ys)
    [f(x, y) for (x, y) in zip(xs, ys)]
end

"""
Compute cartesian product of collections
"""
function cartesian_product(collections...)
    collect(Iterators.product(collections...))
end

end
EOT

# Create std/comp implementation
echo "📝 Creating std/comp/init.jl..."
cat > "$SPARKLE_DIR/std/comp/init.jl" << 'EOT'
module Comp

export compare, max_by, min_by, clamp
export is_between, approx_equal

"""
Three-way comparison function
"""
function compare(a, b)
    a < b ? -1 : (a > b ? 1 : 0)
end

"""
Get maximum element by function
"""
function max_by(f, collection)
    isempty(collection) && throw(ArgumentError("Collection is empty"))
    maximum(x -> (f(x), x), collection)[2]
end

"""
Get minimum element by function
"""
function min_by(f, collection)
    isempty(collection) && throw(ArgumentError("Collection is empty"))
    minimum(x -> (f(x), x), collection)[2]
end

"""
Clamp value between min and max
"""
function clamp(value, min_val, max_val)
    min(max(value, min_val), max_val)
end

"""
Check if value is between min and max (inclusive)
"""
function is_between(value, min_val, max_val)
    min_val <= value <= max_val
end

"""
Check if two values are approximately equal
"""
function approx_equal(a, b; rtol=1e-5, atol=1e-8)
    isapprox(a, b, rtol=rtol, atol=atol)
end

end
EOT

# Create std/conv implementation
echo "📝 Creating std/conv/init.jl..."
cat > "$SPARKLE_DIR/std/conv/init.jl" << 'EOT'
module Conv

export to_int, to_float, to_string, to_bool
export parse_json, to_json
export to_base64, from_base64

using JSON

"""
Convert to Integer safely
"""
function to_int(x; base=10)
    try
        convert(Int, x)
    catch
        try
            parse(Int, string(x), base=base)
        catch
            nothing
        end
    end
end

"""
Convert to Float safely
"""
function to_float(x)
    try
        convert(Float64, x)
    catch
        try
            parse(Float64, string(x))
        catch
            nothing
        end
    end
end

"""
Convert to String safely
"""
function to_string(x)
    try
        string(x)
    catch
        nothing
    end
end

"""
Convert to Boolean safely
"""
function to_bool(x)
    try
        convert(Bool, x)
    catch
        lowercase(string(x)) in ["true", "1", "yes", "on"] ? true :
        lowercase(string(x)) in ["false", "0", "no", "off"] ? false : nothing
    end
end

"""
Parse JSON string safely
"""
function parse_json(str)
    try
        JSON.parse(str)
    catch
        nothing
    end
end

"""
Convert to JSON string safely
"""
function to_json(x)
    try
        JSON.json(x)
    catch
        nothing
    end
end

"""
Convert to Base64
"""
function to_base64(x)
    try
        base64encode(x)
    catch
        nothing
    end
end

"""
Convert from Base64
"""
function from_base64(x)
    try
        base64decode(x)
    catch
        nothing
    end
end

end
EOT

# Create std/def implementation
echo "📝 Creating std/def/init.jl..."
cat > "$SPARKLE_DIR/std/def/init.jl" << 'EOT'
module Def

export default, with_default, map_default
export lazy_default, ensure_default

"""
Get default value for a type
"""
function default(T::Type)
    try
        zero(T)
    catch
        try
            T()
        catch
            nothing
        end
    end
end

"""
Get value or default if nothing
"""
function with_default(value, default_value)
    isnothing(value) ? default_value : value
end

"""
Map function over value, return default if function fails
"""
function map_default(f, value, default_value)
    try
        f(value)
    catch
        default_value
    end
end

"""
Get default value using a lazy evaluation function
"""
function lazy_default(value, default_fn::Function)
    isnothing(value) ? default_fn() : value
end

"""
Ensure value meets predicate or return default
"""
function ensure_default(pred::Function, value, default_value)
    try
        pred(value) ? value : default_value
    catch
        default_value
    end
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
echo "  - std**coll   (Collection utilities)"
echo "  - std**comp   (Comparison operations)"
echo "  - std**conv   (Type conversion utilities)"
echo "  - std**def    (Default value handling)"
echo "Try 'seed plant std**conv' in Sparkle."
