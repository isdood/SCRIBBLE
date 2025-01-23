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
