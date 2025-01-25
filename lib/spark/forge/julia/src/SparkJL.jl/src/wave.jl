"""
Wave implementation
"""

mutable struct Wave
    data::Vector{Float64}
    is_optimized::Bool
    simd_enabled::Bool
    gpu_enabled::Bool
end

Wave(data::Vector{Float64}) = Wave(data, false, false, false)

function weave!(wave::Wave, pattern::WeavePattern)
    pairs = distribute_threads(pattern)
    n_threads = length(pairs)

    if n_threads > 1
        chunk_size = cld(length(wave.data), n_threads)
        @sync for i in 1:n_threads
            Threads.@spawn begin
                start_idx = (i-1) * chunk_size + 1
                end_idx = min(i * chunk_size, length(wave.data))
                optimize_chunk!(wave, start_idx:end_idx, pairs[i])
            end
        end
    end

    wave.is_optimized = true
    wave
end

function optimize_chunk!(wave::Wave, range::UnitRange, pair::BasePair)
    if pair == ZX
        optimize_compute!(wave, range)
    else
        optimize_memory!(wave, range)
    end
end
