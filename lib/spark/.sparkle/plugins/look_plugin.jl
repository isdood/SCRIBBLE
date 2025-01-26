# Look Plugin for Sparkle
include(joinpath(@__DIR__, "..", "..", "forge", "std", "look", "look.jl"))
using .Look

function handle_sparkle_command(input::String)
    parts = split(strip(input))
    isempty(parts) && return false

    cmd = parts[1]
    args = length(parts) > 1 ? parts[2:end] : String[]

    if cmd == "look"
        Look.look(args...)
        return true
    end

    return false
end

# Register command with Sparkle
handle_sparkle_command
