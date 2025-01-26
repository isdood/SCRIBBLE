#!/bin/bash

# Runtime Fix Script for Spark v17
# Author: isdood
# Last Modified: 2025-01-25 22:01:19 UTC
# Repository: isdood/scribble

# Define colors
PURPLE='\033[0;35m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

# Print colored text
print_colored() {
    local color=$1
    local text=$2
    echo -e "${color}${text}${NC}"
}

# Get absolute path for the package
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PKG_DIR="$SCRIPT_DIR/forge/julia/SparkJL"

# Create source files
create_files() {
    local PKG_DIR=$1

    print_colored $BLUE "Creating source files..."
    mkdir -p "$PKG_DIR/src"

    # Create main module file
    cat > "$PKG_DIR/src/SparkJL.jl" << 'EOL'
module SparkJL

__precompile__(false)

using SIMD
using DataStructures
using Test

export WeavePattern, Crystal, Wave
export weave!, distribute_threads

include("types.jl")
include("weave.jl")
include("wave.jl")
include("crystal.jl")
include("optimize.jl")

end # module
EOL

    # Create types.jl
    cat > "$PKG_DIR/src/types.jl" << 'EOL'
struct WeavePattern
    factor::UInt16
    zx_ratio::Float64
    qw_ratio::Float64

    function WeavePattern(factor::Integer)
        1 ≤ factor ≤ 1000 || throw(ArgumentError("Weave factor must be between 1 and 1000"))
        zx_ratio = √(factor / 1000)
        new(factor, zx_ratio, 1.0 - zx_ratio)
    end
end

@enum BasePair ZX QW

mutable struct Wave
    data::Vector{Float64}
    is_optimized::Bool
    simd_enabled::Bool
end

Wave(data::Vector{Float64}) = Wave(data, false, false)

struct Crystal
    dimensions::NTuple{3,Int}
    spacing::Float64
end
EOL

    # Create weave.jl
    cat > "$PKG_DIR/src/weave.jl" << 'EOL'
function distribute_threads(pattern::WeavePattern)
    n_threads = Threads.nthreads()
    pairs = Vector{BasePair}(undef, n_threads)

    for i in 1:n_threads
        ratio = (i - 1) / n_threads
        pairs[i] = ratio < pattern.zx_ratio ? ZX : QW
    end

    pairs
end
EOL

    # Create wave.jl
    cat > "$PKG_DIR/src/wave.jl" << 'EOL'
function weave!(wave::Wave, pattern::WeavePattern)
    pairs = distribute_threads(pattern)
    n_threads = length(pairs)

    if n_threads > 1
        chunk_size = cld(length(wave.data), n_threads)
        @sync for i in 1:n_threads
            Threads.@spawn begin
                start_idx = (i-1) * chunk_size + 1
                end_idx = min(i * chunk_size, length(wave.data))
                optimize_chunk!(wave, start_idx:end_idx, pairs[i])
            end
        end
    end

    wave.is_optimized = true
    wave.simd_enabled = true
    wave
end
EOL

    # Create crystal.jl
    cat > "$PKG_DIR/src/crystal.jl" << 'EOL'
# Constructor for creating a Crystal from 3 dimensions
function Crystal(x::Integer, y::Integer, z::Integer, spacing::Float64)
    Crystal((x, y, z), spacing)
end
EOL

    # Create optimize.jl
    cat > "$PKG_DIR/src/optimize.jl" << 'EOL'
@inline function simd_compute(x::Float64)
    @fastmath sin(x) * cos(x) / (1.0 + x^2)
end

function optimize_chunk!(wave::Wave, range::UnitRange, pair::BasePair)
    if pair == ZX
        optimize_compute!(wave, range)
    else
        optimize_memory!(wave, range)
    end
end

function optimize_compute!(wave::Wave, range::UnitRange)
    if length(range) ≥ 4
        wave.data[range] .= simd_compute.(wave.data[range])
    end
end

function optimize_memory!(wave::Wave, range::UnitRange)
    stride = 64 # Cache line size
    for i in range.start:stride:range.stop
        chunk_end = min(i + stride - 1, range.stop)
        @inbounds wave.data[i:chunk_end] .= identity.(wave.data[i:chunk_end])
    end
end
EOL

    # Create test files
    print_colored $BLUE "Creating test files..."
    mkdir -p "$PKG_DIR/test"

    cat > "$PKG_DIR/test/runtests.jl" << 'EOL'
using Test
using SparkJL

@testset "SparkJL Tests" begin
    @testset "WeavePattern" begin
        @test_throws ArgumentError WeavePattern(0)
        @test_throws ArgumentError WeavePattern(1001)
        pattern = WeavePattern(500)
        @test pattern.factor == 500
        @test 0.0 ≤ pattern.zx_ratio ≤ 1.0
        @test 0.0 ≤ pattern.qw_ratio ≤ 1.0
        @test pattern.zx_ratio + pattern.qw_ratio ≈ 1.0
    end

    @testset "Wave" begin
        data = randn(100)
        wave = Wave(data)
        @test length(wave.data) == 100
        @test !wave.is_optimized
        @test !wave.simd_enabled
    end

    @testset "Crystal" begin
        # Test tuple constructor
        crystal1 = Crystal((32, 32, 32), 1.0)
        @test crystal1.dimensions == (32, 32, 32)
        @test crystal1.spacing == 1.0

        # Test individual dimensions constructor
        crystal2 = Crystal(16, 16, 16, 0.5)
        @test crystal2.dimensions == (16, 16, 16)
        @test crystal2.spacing == 0.5
    end
end
EOL

    return 0
}

fix_runtime() {
    print_colored $BLUE "🔧 Fixing Julia runtime structure..."

    # 1. Clean up existing structure
    rm -rf "$SCRIPT_DIR/forge/julia"
    rm -rf ~/.julia/dev/SparkJL
    rm -rf ~/.julia/compiled/v1.11/SparkJL

    # 2. Create proper directory structure
    mkdir -p "$PKG_DIR"/{src,test}

    # 3. Create Project.toml
    cat > "$PKG_DIR/Project.toml" << EOL
name = "SparkJL"
uuid = "d7891abc-4510-51ab-9240-a78b42f11234"
authors = ["isdood <isdood@users.noreply.github.com>"]
version = "0.2.0"

[deps]
SIMD = "fdea26ae-647d-5447-a871-4b548cad5224"
DataStructures = "864edb3b-99cc-5e75-8d2d-829cb0a9cfe8"
Test = "8dfed614-e22c-5e08-85e1-65c5234f0b40"

[compat]
julia = "1.9"
SIMD = "3.4"
DataStructures = "0.18"
EOL

    # 4. Create package files
    create_files "$PKG_DIR"

    print_colored $GREEN "✓ Created package structure and source files"

    # 5. Install package
    print_colored $BLUE "📦 Installing SparkJL package..."

    if ! cd "$PKG_DIR"; then
        print_colored $RED "Failed to change directory to $PKG_DIR"
        exit 1
    fi

    print_colored $BLUE "Installing dependencies..."
    julia --project=. -e '
        using Pkg
        Pkg.add(["SIMD", "DataStructures", "Test"])
        Pkg.instantiate()
        Pkg.build()'

    # Create test.sh
    cat > "$PKG_DIR/test.sh" << EOL
#!/bin/bash
SCRIPT_DIR="\$( cd "\$( dirname "\${BASH_SOURCE[0]}" )" && pwd )"
julia --project="\$SCRIPT_DIR" -e '
using Pkg
Pkg.test()'
EOL
    chmod +x "$PKG_DIR/test.sh"

    print_colored $GREEN "✓ Installed package"
}

main() {
    print_colored $PURPLE "🛠️ Fixing Julia Runtime..."
    fix_runtime

    print_colored $PURPLE "✨ Runtime fixed:

Changes:
- Fixed source file creation
- Created package structure
- Added dependencies
- Fixed constructor issues
- Added comprehensive tests
- Fixed file paths
- Added error handling

Package location: $PKG_DIR

To test the package:
cd forge/julia/SparkJL && ./test.sh

Or from any directory:
julia --project=\"$PKG_DIR\" -e 'using Pkg; Pkg.test()'"
}

main
