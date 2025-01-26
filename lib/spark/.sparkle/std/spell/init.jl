module Spell

export cast, bind, unbind, modify
export SpellPattern, compile_spell, compose_spells

"""
Spell pattern for command execution
"""
struct SpellPattern
    pattern::String
    action::Function
    description::String
end

const SPELL_REGISTRY = Dict{String, SpellPattern}()

"""
Create and register a new spell
"""
function cast(pattern::String, action::Function; description::String="")
    spell = SpellPattern(pattern, action, description)
    SPELL_REGISTRY[pattern] = spell
    spell
end

"""
Bind data to spell pattern
"""
function bind(data, pattern::String)
    if !haskey(SPELL_REGISTRY, pattern)
        throw(KeyError("Spell pattern not found: $pattern"))
    end

    spell = SPELL_REGISTRY[pattern]
    spell.action(data)
end

"""
Remove spell binding
"""
function unbind(pattern::String)
    delete!(SPELL_REGISTRY, pattern)
end

"""
Modify existing spell
"""
function modify(pattern::String, new_action::Function)
    if !haskey(SPELL_REGISTRY, pattern)
        throw(KeyError("Spell pattern not found: $pattern"))
    end

    old_spell = SPELL_REGISTRY[pattern]
    SPELL_REGISTRY[pattern] = SpellPattern(
        pattern,
        new_action,
        old_spell.description
    )
end

"""
Compile spell pattern
"""
function compile_spell(pattern::String)
    try
        Regex(pattern)
        true
    catch
        false
    end
end

"""
Compose multiple spells
"""
function compose_spells(spells::Vector{SpellPattern})
    data -> begin
        results = []
        for spell in spells
            try
                push!(results, spell.action(data))
            catch e
                @warn "Spell failed: $(spell.pattern)" exception=e
            end
        end
        results
    end
end

end
