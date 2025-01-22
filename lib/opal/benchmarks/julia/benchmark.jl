using BenchmarkTools
include("../../src/julia/core.jl")

using .OpalCore

function benchmark_resonance_field()
    field = ResonanceField()
    @btime optimize!($field)
end

function benchmark_crystal_lattice()
    lattice = CrystalLattice()
    @btime optimize!($lattice)
end

println("Benchmarking Resonance Field:")
benchmark_resonance_field()

println("Benchmarking Crystal Lattice:")
benchmark_crystal_lattice()
