module Potion

export brew, mix, distill, transform
export PotionEffect, apply_effect, remove_effect

using Base.Threads

"""
Effect types for data transformations
"""
@enum PotionEffect begin
    TRANSFORM_UPPERCASE
    TRANSFORM_LOWERCASE
    TRANSFORM_REVERSE
    TRANSFORM_SHUFFLE
    TRANSFORM_UNIQUE
    TRANSFORM_SORT
    TRANSFORM_COMPACT
end

"""
Brew a new transformation
"""
function brew(data, effect::PotionEffect)
    if effect == TRANSFORM_UPPERCASE
        uppercase(data)
    elseif effect == TRANSFORM_LOWERCASE
        lowercase(data)
    elseif effect == TRANSFORM_REVERSE
        reverse(data)
    elseif effect == TRANSFORM_SHUFFLE
        shuffle(collect(data))
    elseif effect == TRANSFORM_UNIQUE
        unique(data)
    elseif effect == TRANSFORM_SORT
        sort(collect(data))
    elseif effect == TRANSFORM_COMPACT
        filter(!isnothing, data)
    end
end

"""
Mix multiple effects
"""
function mix(data, effects::Vector{PotionEffect})
    result = data
    for effect in effects
        result = brew(result, effect)
    end
    result
end

"""
Extract patterns from data
"""
function distill(data, pattern)
    matches = collect(eachmatch(Regex(pattern), data))
    [m.match for m in matches]
end

"""
Apply transformation with custom function
"""
function transform(data, f::Function)
    map(f, data)
end

"""
Apply effect to data
"""
function apply_effect(data, effect::PotionEffect)
    brew(data, effect)
end

"""
Remove effect from data
"""
function remove_effect(data, effect::PotionEffect)
    if effect == TRANSFORM_UPPERCASE
        lowercase(data)
    elseif effect == TRANSFORM_LOWERCASE
        uppercase(data)
    elseif effect == TRANSFORM_REVERSE
        reverse(data)
    else
        data
    end
end

end
