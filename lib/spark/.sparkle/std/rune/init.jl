module Rune

export inscribe, decode, bind, unbind
export RunePattern, match_rune, apply_rune

"""
Rune patterns for pattern matching
"""
struct RunePattern
    pattern::String
    flags::String
    transformation::Function
end

"""
Create a new rune pattern
"""
function inscribe(pattern::String, flags::String="", transform::Function=identity)
    RunePattern(pattern, flags, transform)
end

"""
Decode text using rune pattern
"""
function decode(text::String, rune::RunePattern)
    re = Regex(rune.pattern, rune.flags)
    matches = collect(eachmatch(re, text))
    map(m -> rune.transformation(m.match), matches)
end

"""
Bind rune pattern to data
"""
function bind(data, rune::RunePattern)
    if typeof(data) <: AbstractString
        decode(data, rune)
    else
        map(x -> decode(string(x), rune), data)
    end
end

"""
Remove rune pattern from data
"""
function unbind(data, rune::RunePattern)
    if typeof(data) <: AbstractString
        replace(data, Regex(rune.pattern, rune.flags) => "")
    else
        map(x -> replace(string(x), Regex(rune.pattern, rune.flags) => ""), data)
    end
end

"""
Check if data matches rune pattern
"""
function match_rune(data, rune::RunePattern)
    if typeof(data) <: AbstractString
        !isempty(decode(data, rune))
    else
        map(x -> !isempty(decode(string(x), rune)), data)
    end
end

"""
Apply rune transformation
"""
function apply_rune(data, rune::RunePattern)
    if typeof(data) <: AbstractString
        matches = decode(data, rune)
        isempty(matches) ? data : rune.transformation(matches[1])
    else
        map(x -> apply_rune(x, rune), data)
    end
end

end
