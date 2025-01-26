# Type definitions
struct Crystal
    dimensions
    spacing
    data
end

struct Wave
    data
    frequency
end

struct Pattern
    name
    transform
end

# Global state
const patterns = Dict{String,Pattern}()
mutable struct SparkleState
    current_crystal::Union{Crystal,Nothing}
    current_wave::Union{Wave,Nothing}
    patterns::Dict{String,Pattern}
end

const GLOBAL_STATE = SparkleState(nothing, nothing, patterns)

# Register default patterns
patterns["default"] = Pattern("Default", w -> w)
patterns["invert"] = Pattern("Invert", w -> Wave(-w.data, w.frequency))
patterns["double"] = Pattern("Double", w -> Wave(w.data .* 2, w.frequency))
patterns["smooth"] = Pattern("Smooth", w -> begin
    data = copy(w.data)
    for i in 2:length(data)-1
        data[i] = mean(w.data[i-1:i+1])
    end
    Wave(data, w.frequency)
end)
