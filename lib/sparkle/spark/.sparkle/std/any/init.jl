module Any

export is_nothing, is_something, unwrap, unwrap_or, try_convert

"""
Check if a value is nothing
"""
is_nothing(x) = isnothing(x)

"""
Check if a value is not nothing
"""
is_something(x) = !isnothing(x)

"""
Unwrap a value or throw an error if it's nothing
"""
function unwrap(x)
    isnothing(x) && throw(ArgumentError("Attempted to unwrap nothing"))
    x
end

"""
Unwrap a value or return a default value if it's nothing
"""
unwrap_or(x, default) = isnothing(x) ? default : x

"""
Try to convert a value to a specific type, return nothing if conversion fails
"""
function try_convert(T::Type, x)
    try
        convert(T, x)
    catch
        nothing
    end
end

end
