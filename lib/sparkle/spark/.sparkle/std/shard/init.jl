module Shard

export partition, merge_shards, distribute
export ShardConfig, create_shard, get_shard

using Base.Threads

"""
Shard configuration
"""
struct ShardConfig
    size::Int
    overlap::Int
    distributed::Bool
end

"""
Create a new shard configuration
"""
function create_shard(; size::Int=1000, overlap::Int=0, distributed::Bool=false)
    ShardConfig(size, overlap, distributed)
end

"""
Partition data into shards
"""
function partition(data, config::ShardConfig)
    n = length(data)
    shard_count = ceil(Int, n / (config.size - config.overlap))
    shards = Vector{Any}(undef, shard_count)

    @threads for i in 1:shard_count
        start_idx = (i-1) * (config.size - config.overlap) + 1
        end_idx = min(start_idx + config.size - 1, n)
        shards[i] = data[start_idx:end_idx]
    end

    shards
end

"""
Merge shards back together
"""
function merge_shards(shards::Vector, config::ShardConfig)
    if isempty(shards)
        return []
    end

    result = copy(shards[1])
    for i in 2:length(shards)
        append!(result, shards[i][(config.overlap+1):end])
    end

    result
end

"""
Distribute shards across workers
"""
function distribute(data, config::ShardConfig)
    shards = partition(data, config)
    if config.distributed
        # TODO: Implement distributed processing
        shards
    else
        shards
    end
end

"""
Get specific shard
"""
function get_shard(shards::Vector, index::Int)
    1 <= index <= length(shards) ? shards[index] : nothing
end

end
