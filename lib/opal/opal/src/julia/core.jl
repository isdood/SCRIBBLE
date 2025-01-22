module OpalCore

using ZigBindings
using Crystallography

export ResonanceField, CrystalLattice

struct ResonanceField
    harmony_level::Float64
    attunement::Float64
    field_strength::Float64
end

struct CrystalLattice
    dimensions::Vector{Int}
    resonance_map::Matrix{Float64}
    harmony_paths::Vector{Vector{Int}}
end

end # module
