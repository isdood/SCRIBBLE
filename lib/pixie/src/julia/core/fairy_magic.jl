module FairyMagic

using Distributed
using LinearAlgebra

export EnchantedCrystal, sparkle!, dance!

struct EnchantedCrystal
    crystal_matrix::Matrix{Float64}
    fairy_dust::Vector{Float64}
    enchantment_level::Float64
end

function EnchantedCrystal(size::Integer)
    EnchantedCrystal(
        zeros(size, size),
        zeros(size),
        1.0
    )
end

function sparkle!(crystal::EnchantedCrystal)
    crystal.enchantment_level *= 0.99
    crystal.enchantment_level > 0.87
end

function dance!(crystal::EnchantedCrystal)
    @distributed for i in 1:size(crystal.crystal_matrix, 1)
        crystal.crystal_matrix[i, :] .*= crystal.enchantment_level
    end
end

end # module
