module Thunder

export parallel_map, parallel_filter, parallel_reduce
export @thunder, @storm, distribute_work

using Base.Threads

"""
Parallel map implementation
"""
function parallel_map(f::Function, collection; chunk_size=nothing)
    n = length(collection)
    if isnothing(chunk_size)
        chunk_size = max(1, div(n, nthreads()))
    end

    chunks = [collection[i:min(i+chunk_size-1, n)] for i in 1:chunk_size:n]
    results = Vector{Any}(undef, length(chunks))

    @threads for i in eachindex(chunks)
        results[i] = map(f, chunks[i])
    end

    reduce(vcat, results)
end

"""
Parallel filter implementation
"""
function parallel_filter(pred::Function, collection; chunk_size=nothing)
    n = length(collection)
    if isnothing(chunk_size)
        chunk_size = max(1, div(n, nthreads()))
    end

    chunks = [collection[i:min(i+chunk_size-1, n)] for i in 1:chunk_size:n]
    results = Vector{Any}(undef, length(chunks))

    @threads for i in eachindex(chunks)
        results[i] = filter(pred, chunks[i])
    end

    reduce(vcat, results)
end

"""
Parallel reduce implementation
"""
function parallel_reduce(f::Function, collection; init=nothing)
    if isnothing(init)
        if isempty(collection)
            throw(ArgumentError("empty collection with no init value"))
        end
        init = first(collection)
        collection = collection[2:end]
    end

    n = length(collection)
    chunk_size = max(1, div(n, nthreads()))
    chunks = [collection[i:min(i+chunk_size-1, n)] for i in 1:chunk_size:n]
    results = Vector{Any}(undef, length(chunks))

    @threads for i in eachindex(chunks)
        results[i] = reduce(f, chunks[i], init=init)
    end

    reduce(f, results)
end

"""
Thunder macro for parallel execution
"""
macro thunder(expr)
    quote
        @threads for i in 1:nthreads()
            $(esc(expr))
        end
    end
end

"""
Storm macro for parallel batch processing
"""
macro storm(collection, expr)
    quote
        local data = $(esc(collection))
        local n = length(data)
        local chunk_size = max(1, div(n, nthreads()))
        local chunks = [data[i:min(i+chunk_size-1, n)] for i in 1:chunk_size:n]
        local results = Vector{Any}(undef, length(chunks))

        @threads for i in eachindex(chunks)
            results[i] = map(x -> $(esc(expr)), chunks[i])
        end

        reduce(vcat, results)
    end
end

"""
Distribute work across threads
"""
function distribute_work(tasks::Vector{<:Function})
    results = Vector{Any}(undef, length(tasks))

    @threads for i in eachindex(tasks)
        results[i] = tasks[i]()
    end

    results
end

end
