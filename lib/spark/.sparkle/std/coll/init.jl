module Coll

export OrderedDict, DefaultDict, Counter
export frequency_count, most_common, least_common
export merge_with, zip_with, cartesian_product

using DataStructures

"""
Count frequency of elements in a collection
"""
function frequency_count(collection)
    counter = Counter{eltype(collection)}()
    for item in collection
        counter[item] += 1
    end
    counter
end

"""
Get n most common elements
"""
function most_common(collection, n=nothing)
    counter = frequency_count(collection)
    sorted = sort(collect(counter), by=x->x[2], rev=true)
    isnothing(n) ? sorted : sorted[1:min(n, length(sorted))]
end

"""
Get n least common elements
"""
function least_common(collection, n=nothing)
    counter = frequency_count(collection)
    sorted = sort(collect(counter), by=x->x[2])
    isnothing(n) ? sorted : sorted[1:min(n, length(sorted))]
end

"""
Merge collections with a combining function
"""
function merge_with(f, d1::AbstractDict, d2::AbstractDict)
    result = empty(d1)
    for k in union(keys(d1), keys(d2))
        if haskey(d1, k) && haskey(d2, k)
            result[k] = f(d1[k], d2[k])
        else
            result[k] = get(d1, k, get(d2, k, nothing))
        end
    end
    result
end

"""
Combine two collections element-wise with a function
"""
function zip_with(f, xs, ys)
    [f(x, y) for (x, y) in zip(xs, ys)]
end

"""
Compute cartesian product of collections
"""
function cartesian_product(collections...)
    collect(Iterators.product(collections...))
end

end
