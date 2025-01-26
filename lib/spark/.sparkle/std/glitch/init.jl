module Glitch

export corrupt_bytes, bit_shift, data_bend
export glitch_text, random_artifacts

using Random

"""
Corrupt bytes in data with given probability
"""
function corrupt_bytes(data::Vector{UInt8}, probability::Float64=0.1)
    corrupted = copy(data)
    for i in eachindex(corrupted)
        if rand() < probability
            corrupted[i] = rand(UInt8)
        end
    end
    corrupted
end

"""
Bit shift data by specified amount
"""
function bit_shift(data::Vector{UInt8}, shift::Int)
    shifted = copy(data)
    for i in eachindex(shifted)
        shifted[i] = shifted[i] << shift % 8
    end
    shifted
end

"""
Apply data bending effect
"""
function data_bend(data::Vector{UInt8}, intensity::Float64=0.5)
    bent = copy(data)
    for i in eachindex(bent)
        if rand() < intensity
            bent[i] = bent[i] ⊻ rand(UInt8)
        end
    end
    bent
end

"""
Create glitched text effect
"""
function glitch_text(text::String, intensity::Float64=0.3)
    glitch_chars = "!@#$%^&*()_+-=[]{}|;:,.<>?"
    chars = collect(text)
    for i in eachindex(chars)
        if rand() < intensity
            chars[i] = rand(glitch_chars)
        end
    end
    String(chars)
end

"""
Generate random glitch artifacts
"""
function random_artifacts(length::Integer)
    artifacts = Vector{UInt8}(undef, length)
    for i in eachindex(artifacts)
        artifacts[i] = rand(UInt8)
    end
    artifacts
end

end
