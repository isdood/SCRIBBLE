module SparkJL

__precompile__(false)

using SIMD
using DataStructures
using Test

export WeavePattern, Crystal, Wave
export weave!, distribute_threads

include("types.jl")
include("weave.jl")
include("wave.jl")
include("crystal.jl")
include("optimize.jl")

end # module
