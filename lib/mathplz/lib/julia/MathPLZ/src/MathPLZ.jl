module MathPLZ

using LinearAlgebra
using StaticArrays
using BioStructures

# Crystal lattice operations
include("crystal.jl")
include("quantum.jl")
include("bio.jl")

export CrystalLattice, QuantumState, BioState,
       fold_protein, compute_dna_encoding,
       lattice_coherence, quantum_stability

end
