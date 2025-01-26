# This file is loaded by Sparkle when the package is planted
using .Look: look

# Register the look command with Sparkle
function register_sparkle_command(cmd::String, args::Vector{String})
    if cmd == "look"
        Look.look(args...)
        return true
    end
    return false
end
