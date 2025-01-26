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
