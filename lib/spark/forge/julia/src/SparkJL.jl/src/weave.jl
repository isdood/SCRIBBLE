"""
Thread distribution functionality
"""

function distribute_threads(pattern::WeavePattern)
    n_threads = Threads.nthreads()
    pairs = Vector{BasePair}(undef, n_threads)

    for i in 1:n_threads
        ratio = (i - 1) / n_threads
        pairs[i] = ratio < pattern.zx_ratio ? ZX : QW
    end

    pairs
end
