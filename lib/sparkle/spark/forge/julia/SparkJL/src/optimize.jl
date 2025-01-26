@inline function simd_compute(x::Float64)
    @fastmath sin(x) * cos(x) / (1.0 + x^2)
end

function optimize_chunk!(wave::Wave, range::UnitRange, pair::BasePair)
    if pair == ZX
        optimize_compute!(wave, range)
    else
        optimize_memory!(wave, range)
    end
end

function optimize_compute!(wave::Wave, range::UnitRange)
    if length(range) ≥ 4
        wave.data[range] .= simd_compute.(wave.data[range])
    end
end

function optimize_memory!(wave::Wave, range::UnitRange)
    stride = 64 # Cache line size
    for i in range.start:stride:range.stop
        chunk_end = min(i + stride - 1, range.stop)
        @inbounds wave.data[i:chunk_end] .= identity.(wave.data[i:chunk_end])
    end
end
