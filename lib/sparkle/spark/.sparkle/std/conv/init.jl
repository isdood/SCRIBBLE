module Conv

export to_int, to_float, to_string, to_bool
export parse_json, to_json
export to_base64, from_base64

using JSON

"""
Convert to Integer safely
"""
function to_int(x; base=10)
    try
        convert(Int, x)
    catch
        try
            parse(Int, string(x), base=base)
        catch
            nothing
        end
    end
end

"""
Convert to Float safely
"""
function to_float(x)
    try
        convert(Float64, x)
    catch
        try
            parse(Float64, string(x))
        catch
            nothing
        end
    end
end

"""
Convert to String safely
"""
function to_string(x)
    try
        string(x)
    catch
        nothing
    end
end

"""
Convert to Boolean safely
"""
function to_bool(x)
    try
        convert(Bool, x)
    catch
        lowercase(string(x)) in ["true", "1", "yes", "on"] ? true :
        lowercase(string(x)) in ["false", "0", "no", "off"] ? false : nothing
    end
end

"""
Parse JSON string safely
"""
function parse_json(str)
    try
        JSON.parse(str)
    catch
        nothing
    end
end

"""
Convert to JSON string safely
"""
function to_json(x)
    try
        JSON.json(x)
    catch
        nothing
    end
end

"""
Convert to Base64
"""
function to_base64(x)
    try
        base64encode(x)
    catch
        nothing
    end
end

"""
Convert from Base64
"""
function from_base64(x)
    try
        base64decode(x)
    catch
        nothing
    end
end

end
