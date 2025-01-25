module SparkJL

using LinearAlgebra
using SIMD
using CUDA
using DataStructures

export WeavePattern, Crystal, Wave
export weave!, distribute_threads

include("types.jl")
include("weave.jl")
include("crystal.jl")
include("wave.jl")

end # module
