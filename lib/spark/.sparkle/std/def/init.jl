module Def

export default, with_default, map_default
export lazy_default, ensure_default

"""
Get default value for a type
"""
function default(T::Type)
    try
        zero(T)
    catch
        try
            T()
        catch
            nothing
        end
    end
end

"""
Get value or default if nothing
"""
function with_default(value, default_value)
    isnothing(value) ? default_value : value
end

"""
Map function over value, return default if function fails
"""
function map_default(f, value, default_value)
    try
        f(value)
    catch
        default_value
    end
end

"""
Get default value using a lazy evaluation function
"""
function lazy_default(value, default_fn::Function)
    isnothing(value) ? default_fn() : value
end

"""
Ensure value meets predicate or return default
"""
function ensure_default(pred::Function, value, default_value)
    try
        pred(value) ? value : default_value
    catch
        default_value
    end
end

end
