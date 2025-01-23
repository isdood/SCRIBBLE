#!/bin/bash

echo "Current working directory: $(pwd)"

# First, ensure we're in the resonance directory
if [[ ! "$(pwd)" =~ /resonance$ ]]; then
    echo "Error: Must be run from the resonance directory"
    exit 1
fi

# Clean up and recreate Julia package structure
rm -rf src/julia
mkdir -p src/julia/{src,test}

echo "Creating Julia package structure in: $(pwd)/src/julia"

# Create the main module file
cat > src/julia/src/Resonance.jl << 'EOL'
module Resonance

using DifferentialEquations
using CUDA
using LinearAlgebra

export CrystalStructure, HarmonyField, compute_harmony

struct CrystalStructure
    lattice::Array{Float64, 3}
    harmony_field::Array{Complex{Float64}, 3}
    whimsy_coefficient::Float64
end

struct HarmonyField
    crystal_pattern::Array{Complex{Float64}, 3}
    harmony_level::Float64
    whimsy_factor::Float64
end

function create_harmony_matrix(lattice::Array{Float64, 3})
    size_x, size_y, size_z = size(lattice)
    harmony = zeros(Complex{Float64}, size_x, size_y, size_z)
    
    for i in 1:size_x, j in 1:size_y, k in 1:size_z
        harmony[i,j,k] = complex(lattice[i,j,k] * cos(π/4), lattice[i,j,k] * sin(π/4))
    end
    
    return harmony
end

function initial_crystal_pattern(harmony_field::Array{Complex{Float64}, 3})
    return harmony_field .* exp.(im .* rand(size(harmony_field)...))
end

function compute_harmony(state::CrystalStructure)
    harmony_matrix = create_harmony_matrix(state.lattice)
    pattern = initial_crystal_pattern(state.harmony_field)
    
    function crystal_evolution!(du, u, p, t)
        du .= harmony_matrix .* u
    end
    
    tspan = (0.0, 1.0)
    prob = ODEProblem(crystal_evolution!, pattern, tspan)
    sol = solve(prob, Tsit5(), saveat=0.1)
    
    harmony_level = mean(abs.(sol[end]))
    whimsy = std(angle.(sol[end]))
    
    HarmonyField(sol[end], harmony_level, whimsy)
end

end # module
EOL

# Create Project.toml in the Julia package directory
cat > src/julia/Project.toml << 'EOL'
name = "Resonance"
uuid = "12345678-1234-5678-1234-567812345678"
authors = ["isdood"]
version = "0.1.0"

[deps]
DifferentialEquations = "0c46a032-eb83-5123-abaf-570d42b7fbaa"
CUDA = "052768ef-5323-5732-b1bb-66c8b64840ba"
LinearAlgebra = "37e2e46d-f89d-539d-b4ee-838fcccc9c8e"

[compat]
julia = "1.6"
DifferentialEquations = "7"
CUDA = "5"
EOL

# Create test file
cat > src/julia/test/runtests.jl << 'EOL'
using Resonance
using Test

@testset "Resonance.jl" begin
    # Create a small test lattice
    lattice = rand(4, 4, 4)
    harmony_field = complex.(rand(4, 4, 4))
    crystal = CrystalStructure(lattice, harmony_field, 0.618)
    
    result = compute_harmony(crystal)
    
    @test size(result.crystal_pattern) == (4, 4, 4)
    @test 0.0 ≤ result.harmony_level ≤ 1.0
    @test 0.0 ≤ result.whimsy_factor ≤ π
end
EOL

# Initialize Julia environment
cd src/julia
echo "Initializing Julia environment in: $(pwd)"

julia --project=. -e '
using Pkg
Pkg.activate(".")
Pkg.resolve()
Pkg.instantiate()
'

# Create a simple example script
mkdir -p ../../examples
cat > ../../examples/harmony_example.jl << 'EOL'
using Resonance

# Create a test crystal structure
lattice = rand(8, 8, 8)
harmony_field = complex.(rand(8, 8, 8))
crystal = CrystalStructure(lattice, harmony_field, 0.618)

# Compute harmony
result = compute_harmony(crystal)

println("Harmony level: ", result.harmony_level)
println("Whimsy factor: ", result.whimsy_factor)
EOL

echo "Julia package structure fixed!"
echo "To test the package, run:"
echo "cd src/julia"
echo "julia --project=."
echo "julia> using Resonance"
echo "julia> include(\"../../examples/harmony_example.jl\")"
