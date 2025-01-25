"""
Crystal implementation
"""

mutable struct Crystal
    dimensions::NTuple{3,Int}
    spacing::Float64
    simd_enabled::Bool
    gpu_enabled::Bool
end

Crystal(dims::NTuple{3,Int}, spacing::Float64) = Crystal(dims, spacing, false, false)
