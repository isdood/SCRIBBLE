module Comp

export compare, max_by, min_by, clamp
export is_between, approx_equal

"""
Three-way comparison function
"""
function compare(a, b)
    a < b ? -1 : (a > b ? 1 : 0)
end

"""
Get maximum element by function
"""
function max_by(f, collection)
    isempty(collection) && throw(ArgumentError("Collection is empty"))
    maximum(x -> (f(x), x), collection)[2]
end

"""
Get minimum element by function
"""
function min_by(f, collection)
    isempty(collection) && throw(ArgumentError("Collection is empty"))
    minimum(x -> (f(x), x), collection)[2]
end

"""
Clamp value between min and max
"""
function clamp(value, min_val, max_val)
    min(max(value, min_val), max_val)
end

"""
Check if value is between min and max (inclusive)
"""
function is_between(value, min_val, max_val)
    min_val <= value <= max_val
end

"""
Check if two values are approximately equal
"""
function approx_equal(a, b; rtol=1e-5, atol=1e-8)
    isapprox(a, b, rtol=rtol, atol=atol)
end

end
