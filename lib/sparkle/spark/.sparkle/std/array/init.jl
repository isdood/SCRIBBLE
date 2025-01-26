module Array

export chunk, compact, unique_by, flatten, group_by, partition

"""
Split array into chunks of specified size
"""
function chunk(arr, size::Integer)
    [arr[i:min(i + size - 1, end)] for i in 1:size:length(arr)]
end

"""
Remove nil elements from array
"""
function compact(arr)
    filter(!isnothing, arr)
end

"""
Get unique elements based on a function
"""
function unique_by(f, arr)
    unique(x -> f(x), arr)
end

"""
Flatten a nested array structure
"""
function flatten(arr)
    result = []
    for item in arr
        if item isa AbstractArray
            append!(result, flatten(item))
        else
            push!(result, item)
        end
    end
    result
end

"""
Group array elements by function result
"""
function group_by(f, arr)
    groups = Dict()
    for item in arr
        key = f(item)
        if !haskey(groups, key)
            groups[key] = []
        end
        push!(groups[key], item)
    end
    groups
end

"""
Split array into two arrays based on predicate
"""
function partition(pred, arr)
    trues = []
    falses = []
    for item in arr
        if pred(item)
            push!(trues, item)
        else
            push!(falses, item)
        end
    end
    (trues, falses)
end

end
