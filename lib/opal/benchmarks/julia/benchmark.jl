using BenchmarkTools
include("../../src/julia/core.jl")

using .OpalCore

function benchmark_resonance_field()
    field = ResonanceField()
    println("\nBenchmarking Resonance Field optimization:")
    result = @benchmark optimize!($field) samples=100 seconds=2
    display(result)
    return field  # Return for validation
end

function benchmark_crystal_lattice()
    lattice = CrystalLattice()
    println("\nBenchmarking Crystal Lattice optimization:")
    result = @benchmark optimize!($lattice) samples=100 seconds=2
    display(result)
    return lattice  # Return for validation
end

# Run benchmarks with validation
field = benchmark_resonance_field()
@assert field.intensity <= field.max_intensity "Intensity exceeded maximum"
@assert 0.0 <= field.phase <= 2π "Phase out of bounds"

lattice = benchmark_crystal_lattice()
@assert lattice.spacing <= lattice.max_spacing "Spacing exceeded maximum"
@assert 0.0 <= lattice.orientation <= 2π "Orientation out of bounds"
